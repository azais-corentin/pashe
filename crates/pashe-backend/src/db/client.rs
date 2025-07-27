use super::error::Error;
use super::schema::StatisticsEvent;
use crate::db::Item;
use human_repr::HumanCount;
use std::time::Duration;
use tracing::debug;

pub struct Client {
    client: clickhouse::Client,
}

impl Client {
    pub fn new(url: &str, user: &str, password: &str, database: &str) -> Self {
        let client = clickhouse::Client::default()
            .with_url(url)
            .with_user(user)
            .with_password(password)
            .with_database(database);
        Self { client }
    }

    #[tracing::instrument(skip_all, level = "trace")]
    pub async fn insert_statistics_event(&self, event: StatisticsEvent) -> Result<(), Error> {
        let mut insert = self.client.insert::<StatisticsEvent>("statistics_events")?;

        insert.write(&event).await?;
        insert.end().await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, level = "trace")]
    pub async fn insert_items(&self, items: Vec<Item>) -> Result<(), Error> {
        let mut inserter = self
            .client
            .inserter::<Item>("items")?
            .with_timeouts(Some(Duration::from_secs(5)), Some(Duration::from_secs(20)))
            .with_max_bytes(50_000_000)
            .with_max_rows(750_000);

        items.iter().try_for_each(|item| inserter.write(item))?;
        inserter.commit().await?;
        let stats = inserter.end().await?;

        if stats.rows > 0 {
            debug!(
                "{} items ({}) have been inserted",
                stats.rows.human_count_bare(),
                stats.bytes.human_count_bytes(),
            )
        } else {
            debug!("No rows inserted");
        }

        Ok(())
    }
}
