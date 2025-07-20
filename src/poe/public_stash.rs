use crate::poe::constants::BASE_URL;
use crate::poe::types::PublicStashTabs;
use anyhow::{Context, Result};
use oauth2::http::response;
use std::fs;
use tracing::{debug, error};

pub async fn fetch_public_stashes(
    client: &reqwest::Client,
    next_change_id: &str,
) -> Result<PublicStashTabs> {
    let url = format!("{}/public-stash-tabs?id={}", BASE_URL, next_change_id);

    debug!("Fetching public stashes: {}", url);

    let response = client.get(url.clone()).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        let status = response.status();
        error!("Failed to fetch public stashes: HTTP {}", status);
        return Err(anyhow::anyhow!(
            "Failed to fetch public stashes: HTTP {}",
            status
        ));
    }

    let text_body = response
        .text()
        .await
        .with_context(|| format!("Failed to read response body from {}", url))?;

    let _span = tracing::debug_span!("Parsing response body");
    let stashes = match serde_json::from_str::<PublicStashTabs>(&text_body) {
        Ok(stashes) => stashes,
        Err(e) => {
            error!("Failed to parse response: {}", e);
            return Err(anyhow::anyhow!("Failed to parse response: {}", e));
        }
    };

    Ok(stashes)
}
