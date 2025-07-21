use anyhow::Result;
use clap::{Parser, Subcommand};

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
    /// Prints the current version of the database.
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
    /// Create a new set of migration files (up and down migrations).
    #[command(arg_required_else_help = true)]
    Create {
        /// The name of the migration
        name: String,
    },
    /// Migrates to the specified version.
    To {
        /// The version to migrate to
        version: String,
    },
}

fn create(cli: &Cli, name: &str) {
    println!("'migrate create' was used, name is: {name}");
}

fn to(cli: &Cli, version: &str) {
    println!("'migrate to' was used, version is: {version}");
}

fn reset(cli: &Cli) {
    println!("'migrate reset' was used");
}

fn version(cli: &Cli) {
    println!("'migrate version' was used");
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
        Commands::Reset => reset(&cli),
        Commands::Version => version(&cli),
    }

    Ok(())
}
