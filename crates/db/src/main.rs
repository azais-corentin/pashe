use anyhow::Result;
use clap::{Parser, Subcommand};
use clickhouse::Client;

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
        version: String,
    },
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

fn create(_cli: &Cli, name: &str) {
    println!("'migrate create' was used, name is: {name}");
}

fn to(_cli: &Cli, version: &str) {
    println!("'migrate to' was used, version is: {version}");
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

async fn version() -> Result<String> {
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
        Err(clickhouse::error::Error::RowNotFound) => "unknown".to_string(),
        Err(e) => return Err(e.into()),
    };

    println!("Current database version: {}", result);

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migration(migration) => match &migration.command {
            MigrationCommands::Create { name } => create(&cli, name),
            MigrationCommands::To { version } => to(&cli, version),
        },
        Commands::Reset => reset(&cli).await?,
        Commands::Version => {
            let _ = version().await?;
        }
    }

    Ok(())
}
