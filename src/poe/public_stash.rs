use crate::poe::constants::BASE_URL;
use crate::poe::types::PublicStashTabs;
use anyhow::{Context, Result};
use tracing::{debug, error};

pub struct Crawler {
    stash_count: u64,
    item_count: u64,
    bytes: u64,
}

impl Default for Crawler {
    fn default() -> Self {
        Crawler {
            stash_count: 0,
            item_count: 0,
            bytes: 0,
        }
    }
}

pub trait Fetch {
    async fn fetch(
        &mut self,
        client: &reqwest_middleware::ClientWithMiddleware,
        next_change_id: &str,
    ) -> Result<PublicStashTabs>;
}

impl Fetch for Crawler {
    async fn fetch(
        &mut self,
        client: &reqwest_middleware::ClientWithMiddleware,
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

        // TODO: Read x-next-change-id header and schedule the next request ASAP

        let text_body = response
            .text()
            .await
            .with_context(|| format!("Failed to read response body from {}", url))?;

        let _span = tracing::debug_span!("Parsing response body");
        let stash_changes = match serde_json::from_str::<PublicStashTabs>(&text_body) {
            Ok(stashes) => stashes,
            Err(e) => {
                error!("Failed to parse response: {}", e);
                return Err(anyhow::anyhow!("Failed to parse response: {}", e));
            }
        };

        self.stash_count += stash_changes.stashes.len() as u64;
        self.item_count += stash_changes
            .stashes
            .iter()
            .map(|stash| stash.items.len() as u64)
            .sum::<u64>();
        self.bytes += text_body.len() as u64;

        debug!(
            "Total stats: {} stashes / {} items / {}",
            self.stash_count,
            self.item_count,
            bytesize::ByteSize::b(self.bytes).display().si()
        );

        Ok(stash_changes)
    }
}
