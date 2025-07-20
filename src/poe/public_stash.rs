use crate::poe::constants::BASE_URL;
use anyhow::Result;
use tracing::{debug, error};

pub async fn fetch_public_stashes(client: &reqwest::Client, next_change_id: &str) -> Result<()> {
    let url = format!("{}/public-stash-tabs?id={}", BASE_URL, next_change_id);

    debug!("Fetching public stashes: {}", url);

    let response = client.get(url).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        let status = response.status();
        error!("Failed to fetch public stashes: HTTP {}", status);
        return Err(anyhow::anyhow!(
            "Failed to fetch public stashes: HTTP {}",
            status
        ));
    }

    let body = response.text().await?;

    debug!("Received response: {}", body);

    Ok(())
}
