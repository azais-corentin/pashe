use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Working directory
    #[arg(short, long, value_name = "DIR", default_value = ".", global = true)]
    pub directory: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage database migrations
    Migration(Migration),
    /// Reset the database
    Reset {
        /// Force reset without confirmation
        #[arg(long, short)]
        force: bool,
    },
    /// Prints the current version of the database
    Version,
}

#[derive(Parser)]
/// Manage database migrations
pub struct Migration {
    #[command(subcommand)]
    pub command: MigrationCommands,
}

#[derive(Subcommand)]
pub enum MigrationCommands {
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
