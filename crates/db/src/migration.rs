use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::error::DbError;

#[derive(Debug, PartialEq, Eq, Clone)]
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

pub async fn create(directory: &str, name: &str) -> Result<()> {
    // Create migration directory if it doesn't exist
    let directory = std::env::current_dir()?.join(directory);
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

pub async fn to(client: &clickhouse::Client, directory: &str, target_version: &str) -> Result<()> {
    let directory = std::env::current_dir()?.join(directory);
    let versions = get_available_migration_versions(&directory)?;

    if versions.is_empty() {
        info!(
            "No migrations found in the directory: {}",
            directory.display()
        );
        return Ok(());
    }

    debug!(
        "Available migration(s): {}",
        versions
            .iter()
            .map(|v| format!("{}@{}", v.name, v.version))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let current_version = match crate::version(client).await {
        Ok(v) => v,
        Err(DbError::UnknownVersion) => {
            info!("Unknown database version, interpreting as version 0");
            0
        }
        Err(e) => return Err(e.into()),
    };

    let target_version = if target_version == "latest" {
        versions.last().map_or(0, |v| v.version)
    } else {
        target_version
            .parse::<u32>()
            .with_context(|| format!("Invalid target version: {target_version}"))?
    };

    if current_version == target_version {
        info!("Database is already at version {target_version}");
        return Ok(());
    }

    let (steps, direction) = if current_version > target_version {
        info!("Downgrading database from version {current_version} to {target_version}");

        let migrations_steps: Vec<_> = versions
            .iter()
            .filter(|m| m.version <= current_version && m.version > target_version)
            .rev()
            .collect();

        (migrations_steps, "down")
    } else {
        info!("Upgrading database from version {current_version} to {target_version}");

        let migrations_steps: Vec<_> = versions
            .iter()
            .filter(|m| m.version > current_version && m.version <= target_version)
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

    let mut latest_version = current_version;

    for m in steps {
        let file = directory.join(format!("{:06}_{}.{}.sql", m.version, m.name, direction));
        info!("Applying migration file: {}", file.display());

        let contents = std::fs::read_to_string::<PathBuf>(file.clone())
            .with_context(|| format!("Failed to read migration file: {}", file.display()))?;

        let queries = contents.split(';').filter(|query| !query.trim().is_empty());

        for query in queries {
            client
                .query(query)
                .execute()
                .await
                .with_context(|| format!("Failed to execute query: {query}"))?;
        }
        latest_version = if direction == "up" {
            m.version
        } else {
            m.version - 1
        };
    }

    // Update the schema_migrations table
    // Delete existing version and insert the new one to ensure only one row exists
    client
        .query("ALTER TABLE schema_migrations DELETE WHERE 1=1")
        .execute()
        .await
        .with_context(|| "Failed to delete old version from schema_migrations")?;

    client.query("INSERT INTO schema_migrations (version) SETTINGS async_insert=1, wait_for_async_insert=1 VALUES (?)")
        .bind(latest_version)
        .execute()
        .await
        .with_context(|| {
            format!("Failed to update schema_migrations to version {latest_version}")
        })?;

    Ok(())
}

pub async fn version(client: &clickhouse::Client) -> Result<u32, DbError> {
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
