use anyhow::Result;
use db::{DbError, create, reset, to, version};
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use testcontainers_modules::testcontainers::{ImageExt, runners::AsyncRunner};

async fn cleanup(client: &clickhouse::Client, migrations_path: &str) {
    reset(client, true).await.unwrap();
    fs::remove_dir_all(migrations_path).ok();
}

#[tokio::test]
#[serial]
async fn test_migration_cycle() -> Result<()> {
    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("Failed to initialize tracing subscriber: {}", e);
    }

    let user = "pashe".to_string();
    let password = "pashe".to_string();
    let database = "pashe".to_string();

    let container = testcontainers_modules::clickhouse::ClickHouse::default()
        .with_tag("latest")
        .with_env_var("CLICKHOUSE_USER", &user)
        .with_env_var("CLICKHOUSE_PASSWORD", &password)
        .with_env_var("CLICKHOUSE_DB", &database)
        .start()
        .await
        .expect("Failed to start ClickHouse container");
    let host = container
        .get_host()
        .await
        .expect("Failed to get ClickHouse host");
    let port = container
        .get_host_port_ipv4(8123)
        .await
        .expect("Failed to get ClickHouse port");

    let url = format!("http://{host}:{port}");

    let client = db::DatabaseConfig::new(url, user, password, database).create_client();

    let migrations_directory = "test_migrations_cycle";
    cleanup(&client, migrations_directory).await;

    // 1. Create initial migration
    create(migrations_directory, "initial").await?;
    let files = fs::read_dir(migrations_directory)?;
    assert_eq!(files.count(), 2);

    // Create up and down files with some content
    let up_file = PathBuf::from(migrations_directory).join("000001_initial.up.sql");
    let down_file = PathBuf::from(migrations_directory).join("000001_initial.down.sql");
    fs::write(
        up_file,
        "CREATE TABLE test_table (id UInt64) ENGINE = MergeTree ORDER BY id;",
    )?;
    fs::write(down_file, "DROP TABLE test_table;")?;

    // 2. Migrate up
    to(&client, migrations_directory, "1").await?;
    let v = version(&client).await?;
    assert_eq!(v, 1);

    // 3. Create a second migration
    create(migrations_directory, "second").await?;
    let files = fs::read_dir(migrations_directory)?;
    assert_eq!(files.count(), 4);
    let up_file2 = PathBuf::from(migrations_directory).join("000002_second.up.sql");
    let down_file2 = PathBuf::from(migrations_directory).join("000002_second.down.sql");
    fs::write(up_file2, "ALTER TABLE test_table ADD COLUMN name String;")?;
    fs::write(down_file2, "ALTER TABLE test_table DROP COLUMN name;")?;

    // 4. Migrate up to version 2
    to(&client, migrations_directory, "2").await?;
    let v = version(&client).await?;
    assert_eq!(v, 2);

    // 5. Migrate down to version 1
    to(&client, migrations_directory, "1").await?;
    let v = version(&client).await?;
    assert_eq!(v, 1);

    // 6. Migrate down to version 0
    to(&client, migrations_directory, "0").await?;
    let v = version(&client).await?;
    assert_eq!(v, 0);

    // 7. Reset the database
    reset(&client, true).await?;
    let v = version(&client).await;
    assert!(matches!(v, Err(DbError::UnknownVersion)));

    cleanup(&client, migrations_directory).await;
    Ok(())
}
