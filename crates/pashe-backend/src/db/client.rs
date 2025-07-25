use super::error::Error;
use super::schema::StatisticsEvent;

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

    pub async fn insert_statistics_event(&self, event: StatisticsEvent) -> Result<(), Error> {
        let mut insert = self.client.insert::<StatisticsEvent>("statistics_events")?;

        insert.write(&event).await?;
        insert.end().await?;
        Ok(())
    }
}
