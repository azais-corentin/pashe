use anyhow::Result;
use db::{
    DbError,
    cli::{Cli, Commands},
    create, reset, to, version,
};
use serial_test::serial;
use std::fs;
use std::path::PathBuf;

fn setup() {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::try_init();
}

fn get_cli(migrations_path: &str, command: Commands) -> Cli {
    Cli {
        directory: migrations_path.to_string(),
        command,
    }
}

async fn cleanup(migrations_path: &str) {
    let cli = get_cli(migrations_path, Commands::Reset { force: true });
    reset(&cli).await.unwrap();
    fs::remove_dir_all(migrations_path).ok();
}

#[tokio::test]
#[serial]
async fn test_migration_cycle() -> Result<()> {
    setup();
    let migrations_path = "test_migrations_cycle";
    cleanup(migrations_path).await;
    // Ensure the directory is clean before starting
    let cli = get_cli(migrations_path, Commands::Version); // Default command

    // 1. Create initial migration
    create(&cli, "initial").await?;
    let files = fs::read_dir(migrations_path)?;
    assert_eq!(files.count(), 2);

    // Create up and down files with some content
    let up_file = PathBuf::from(migrations_path).join("000001_initial.up.sql");
    let down_file = PathBuf::from(migrations_path).join("000001_initial.down.sql");
    fs::write(
        up_file,
        "CREATE TABLE test_table (id UInt64) ENGINE = MergeTree ORDER BY id;",
    )?;
    fs::write(down_file, "DROP TABLE test_table;")?;

    // 2. Migrate up
    to(&cli, 1).await?;
    let v = version().await?;
    assert_eq!(v, 1);

    // 3. Create a second migration
    create(&cli, "second").await?;
    let files = fs::read_dir(migrations_path)?;
    assert_eq!(files.count(), 4);
    let up_file2 = PathBuf::from(migrations_path).join("000002_second.up.sql");
    let down_file2 = PathBuf::from(migrations_path).join("000002_second.down.sql");
    fs::write(up_file2, "ALTER TABLE test_table ADD COLUMN name String;")?;
    fs::write(down_file2, "ALTER TABLE test_table DROP COLUMN name;")?;

    // 4. Migrate up to version 2
    to(&cli, 2).await?;
    let v = version().await?;
    assert_eq!(v, 2);

    // 5. Migrate down to version 1
    to(&cli, 1).await?;
    let v = version().await?;
    assert_eq!(v, 1);

    // 6. Migrate down to version 0
    to(&cli, 0).await?;
    let v = version().await?;
    assert_eq!(v, 0);

    // 7. Reset the database
    // Set force to true to avoid confirmation prompt
    let cli = get_cli(migrations_path, Commands::Reset { force: true });
    reset(&cli).await?;
    let v = version().await;
    assert!(matches!(v, Err(DbError::UnknownVersion)));

    cleanup(migrations_path).await;
    Ok(())
}
