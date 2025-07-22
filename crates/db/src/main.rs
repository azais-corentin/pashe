use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clickhouse::Client;
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Working directory
    #[arg(short, long, value_name = "DIR", default_value = ".", global = true)]
    directory: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage database migrations
    Migration(Migration),
    /// Reset the database
    Reset,
    /// Prints the current version of the database
    Version,
}

#[derive(Parser)]
/// Manage database migrations
pub struct Migration {
    #[command(subcommand)]
    command: MigrationCommands,
}

#[derive(Subcommand)]
enum MigrationCommands {
    /// Create a new set of migration files (up and down migrations)
    #[command(arg_required_else_help = true)]
    Create {
        /// The name of the migration
        name: String,
    },
    /// Migrates to the specified version
    To {
        /// The version to migrate to
        version: u32,
    },
}

// Io(#[from] io::Error),

#[derive(Error, Debug)]
enum DbError {
    #[error("Clickhouse error")]
    Clickhouse(#[from] clickhouse::error::Error),
    #[error("number error")]
    Number(#[from] std::num::ParseIntError),
    #[error("unknown version")]
    UnknownVersion,
}

fn get_db() -> Client {
    let clickhouse_url =
        std::env::var("CLICKHOUSE_URL").expect("Missing the CLICKHOUSE_URL environment variable.");
    let clickhouse_user = std::env::var("CLICKHOUSE_USER")
        .expect("Missing the CLICKHOUSE_USER environment variable.");
    let clickhouse_password = std::env::var("CLICKHOUSE_PASSWORD")
        .expect("Missing the CLICKHOUSE_PASSWORD environment variable.");
    let clickhouse_database = std::env::var("CLICKHOUSE_DATABASE")
        .expect("Missing the CLICKHOUSE_DATABASE environment variable.");

    Client::default()
        .with_url(clickhouse_url)
        .with_user(clickhouse_user)
        .with_password(clickhouse_password)
        .with_database(clickhouse_database)
}

fn get_available_migration_versions(directory: &PathBuf) -> Result<Vec<u32>> {
    let mut versions = std::fs::read_dir(directory)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            let file_name = entry.file_name().into_string().ok()?;
            if file_name.ends_with(".up.sql") || file_name.ends_with(".down.sql") {
                let version: u32 = file_name.split('_').next()?.parse().ok()?;
                Some(version)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    versions.sort();
    versions.dedup();
    Ok(versions)
}

async fn create(cli: &Cli, name: &str) -> Result<()> {
    println!("'migrate create' was used, name is: {name}");

    // Create migration directory if it doesn't exist
    let directory = std::env::current_dir()?.join(&cli.directory);
    std::fs::create_dir_all(&directory).with_context(|| {
        format!(
            "Failed to create migration directory: {}",
            directory.display()
        )
    })?;

    // Check for any existing migrations, find the latest version and increment it by 1
    let version = get_available_migration_versions(&directory)?
        .iter()
        .max()
        .map_or(1, |v| v + 1);

    // Create migration files
    let up_file_path = directory.join(format!("{version:06}_{name}.up.sql"));
    let down_file_path = directory.join(format!("{version:06}_{name}.down.sql"));

    std::fs::File::create(&up_file_path).with_context(|| {
        format!(
            "Failed to create migration file: {}",
            up_file_path.display()
        )
    })?;
    std::fs::File::create(&down_file_path).with_context(|| {
        format!(
            "Failed to create migration file: {}",
            down_file_path.display()
        )
    })?;

    Ok(())
}

async fn to(_cli: &Cli, version: &u32) -> Result<()> {
    println!("'migrate to' was used, version is: {version}");

    let current_version = match crate::version().await {
        Ok(v) => v,
        Err(DbError::UnknownVersion) => {
            println!("Unknown database version, interpreting as version 0");
            0
        }
        Err(e) => return Err(e.into()),
    };

    if current_version == *version {
        println!("Database is already at version {version}");
        return Ok(());
    }

    if current_version > *version {
        println!("Downgrading database from version {current_version} to {version}");
    } else {
        println!("Upgrading database from version {current_version} to {version}");
    }

    Ok(())
}

async fn reset(_cli: &Cli) -> Result<()> {
    let client = get_db();
    let result = client.query("SHOW TABLES").fetch_all::<String>().await?;

    if result.is_empty() {
        println!("No tables found in the database.");
        return Ok(());
    }

    println!(
        "Are you sure you want to drop the table{} {}? [y/N]",
        if result.len() > 1 { "s" } else { "" },
        result.join(", ")
    );
    let mut confirmation = String::new();
    std::io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read line");

    if confirmation.trim().to_lowercase() != "y" {
        println!("No changes made, exiting.");
        return Ok(());
    }

    let tasks = result.iter().map(|table_name| {
        let client = client.clone();
        let table_name = table_name.clone();
        tokio::spawn(async move {
            client
                .query(format!("DROP TABLE IF EXISTS {table_name}").as_str())
                .execute()
                .await
                .expect("Failed to drop table");
            println!("Dropped table: {table_name}");
        })
    });

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn version() -> Result<u32, DbError> {
    let client = get_db();

    // Ensure the schema_migrations table exists
    client
        .query("CREATE TABLE IF NOT EXISTS schema_migrations (version String, applied_at DateTime DEFAULT now()) ENGINE = MergeTree ORDER BY version")
        .execute()
        .await?;

    let result = match client
        .query("SELECT version FROM schema_migrations")
        .fetch_one::<String>()
        .await
    {
        Ok(version) => version,
        Err(clickhouse::error::Error::RowNotFound) => {
            return Err(DbError::UnknownVersion);
        }
        Err(e) => return Err(DbError::Clickhouse(e)),
    };

    let version = result.parse()?;

    println!("Current database version: {version}");

    Ok(version)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migration(migration) => match &migration.command {
            MigrationCommands::Create { name } => create(&cli, name).await?,
            MigrationCommands::To { version } => to(&cli, version).await?,
        },
        Commands::Reset => reset(&cli).await?,
        Commands::Version => {
            let _ = version().await?;
        }
    }

    Ok(())
}
