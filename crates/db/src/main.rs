use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clickhouse::Client;
use std::cmp::Ordering;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, info};
use tracing_subscriber::prelude::*;

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

#[derive(Debug, PartialEq, Eq)]
struct MigrationInfo {
    name: String,
    version: u32,
}

impl Ord for MigrationInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for MigrationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_available_migration_versions(directory: &PathBuf) -> Result<Vec<MigrationInfo>> {
    let mut versions = std::fs::read_dir(directory)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().ok()?.is_file() {
                return None;
            }
            let file_name = entry.file_name().into_string().ok()?;
            if file_name.ends_with(".up.sql") || file_name.ends_with(".down.sql") {
                let mut parts = file_name.splitn(2, '_');
                let version: u32 = parts.next()?.parse().ok()?;
                let name_part = parts.next()?;

                let name = name_part
                    .strip_suffix(".up.sql")
                    .or_else(|| name_part.strip_suffix(".down.sql"))
                    .unwrap_or(name_part)
                    .to_string();

                let migration_info = MigrationInfo { name, version };

                Some(migration_info)
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
        .map_or(1, |migration| migration.version + 1);

    // Create migration files
    let up_file_path = directory.join(format!("{version:06}_{name}.up.sql"));
    let down_file_path = directory.join(format!("{version:06}_{name}.down.sql"));

    info!(
        "Creating migration files {} and {}",
        up_file_path.display(),
        down_file_path.display()
    );

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

async fn to(cli: &Cli, target_version: &u32) -> Result<()> {
    let directory = std::env::current_dir()?.join(&cli.directory);
    let versions = get_available_migration_versions(&directory)?;

    debug!(
        "Available migration(s): {}",
        versions
            .iter()
            .map(|v| format!("{}@{}", v.name, v.version))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let current_version = match crate::version().await {
        Ok(v) => v,
        Err(DbError::UnknownVersion) => {
            info!("Unknown database version, interpreting as version 0");
            0
        }
        Err(e) => return Err(e.into()),
    };

    if current_version == *target_version {
        info!("Database is already at version {target_version}");
        return Ok(());
    }

    let db = get_db();

    let (steps, direction) = if current_version > *target_version {
        info!("Downgrading database from version {current_version} to {target_version}");

        let migrations_steps: Vec<_> = versions
            .into_iter()
            .filter(|m| m.version <= current_version && m.version > *target_version)
            .rev()
            .collect();

        (migrations_steps, "down")
    } else {
        info!("Upgrading database from version {current_version} to {target_version}");

        let migrations_steps: Vec<_> = versions
            .into_iter()
            .filter(|m| m.version > current_version && m.version <= *target_version)
            .collect();

        (migrations_steps, "up")
    };

    info!(
        "Applying migrations {}",
        steps
            .iter()
            .map(|v| v.version.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let migration_files = steps
        .iter()
        .map(|m| directory.join(format!("{}_{}.{}.sql", m.version, m.name, direction)));

    for file in migration_files {
        info!("Applying migration file: {}", file.display());

        let contents = std::fs::read_to_string::<PathBuf>(file.clone())
            .with_context(|| format!("Failed to read migration file: {}", file.display()))?;

        let queries = contents.split(";").filter(|query| !query.trim().is_empty());

        for query in queries {
            db.query(query)
                .execute()
                .await
                .with_context(|| format!("Failed to execute query: {query}"))?;
        }
    }

    // Update the schema_migrations table
    // Delete existing version and insert the new one to ensure only one row exists
    db.query("ALTER TABLE schema_migrations DELETE WHERE 1=1")
        .execute()
        .await
        .with_context(|| "Failed to delete old version from schema_migrations")?;

    db.query("INSERT INTO schema_migrations (version) VALUES (?)")
        .bind(target_version)
        .execute()
        .await
        .with_context(|| {
            format!("Failed to update schema_migrations to version {target_version:06}")
        })?;

    Ok(())
}

async fn reset(_cli: &Cli) -> Result<()> {
    let client = get_db();
    let result = client.query("SHOW TABLES").fetch_all::<String>().await?;

    if result.is_empty() {
        info!("No tables found in the database.");
        return Ok(());
    }

    info!(
        "Are you sure you want to drop the table{} {}? [y/N]",
        if result.len() > 1 { "s" } else { "" },
        result.join(", ")
    );
    let mut confirmation = String::new();
    std::io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read line");

    if confirmation.trim().to_lowercase() != "y" {
        info!("No changes made, exiting.");
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
            info!("Dropped table: {table_name}");
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

    info!("Current database version: {version}");

    Ok(version)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::TRACE)
        .with_target("hyper_util", tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(filter)
        .init();

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
