use crate::poe::{constants::BASE_URL, types::PublicStashTabs};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

pub struct Crawler {
    shutdown_token: CancellationToken,
    stash_count: AtomicU64,
    item_count: AtomicU64,
    bytes: AtomicU64,
}

impl Crawler {
    pub fn new(shutdown_token: CancellationToken) -> Self {
        Crawler {
            shutdown_token,
            stash_count: AtomicU64::new(0),
            item_count: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
        }
    }
}

#[async_trait]
pub trait Crawl {
    async fn crawl(
        self: Arc<Self>,
        client: Arc<reqwest_middleware::ClientWithMiddleware>,
        next_change_id: String,
    ) -> Result<()>;
}

#[async_trait]
impl Crawl for Crawler {
    async fn crawl(
        self: Arc<Self>,
        client: Arc<reqwest_middleware::ClientWithMiddleware>,
        next_change_id: String,
    ) -> Result<()> {
        if self.shutdown_token.is_cancelled() {
            debug!("Shutting down crawler");
            return Ok(());
        }

        let url = format!("{}/public-stash-tabs?id={}", BASE_URL, next_change_id);

        debug!("Fetching next change id: {}", next_change_id);

        // TODO: Cancel the await if shutdown is requested (with select! or similar)
        let response = client.get(url.clone()).send().await?;

        if response.status() != reqwest::StatusCode::OK {
            let status = response.status();
            error!("Failed to fetch public stashes: HTTP {}", status);
            return Err(anyhow::anyhow!(
                "Failed to fetch public stashes: HTTP {}",
                status
            ));
        }

        let next_change_id = response
            .headers()
            .get("x-next-change-id")
            .ok_or(anyhow::anyhow!(
                "Missing x-next-change-id header in response from {}",
                url
            ))?
            .to_str()?
            .to_owned();

        let self_clone = Arc::clone(&self);
        let future = tokio::spawn(async move {
            if let Err(e) = self_clone.crawl(client, next_change_id).await {
                error!("Crawl failed: {}", e);
            }
        });

        // TODO: Cancel the await if shutdown is requested (with select! or similar)
        let text_body = response
            .text()
            .await
            .with_context(|| format!("Failed to read response body from {}", url))?;

        let stash_changes = serde_json::from_str::<PublicStashTabs>(&text_body)
            .with_context(|| format!("Failed to parse response body from {}", url))?;

        self.stash_count
            .fetch_add(stash_changes.stashes.len() as u64, Ordering::SeqCst);
        self.item_count.fetch_add(
            stash_changes
                .stashes
                .iter()
                .map(|stash| stash.items.len() as u64)
                .sum::<u64>(),
            Ordering::SeqCst,
        );
        self.bytes
            .fetch_add(text_body.len() as u64, Ordering::SeqCst);

        debug!(
            "Total stats: {} stashes / {} items / {}",
            self.stash_count.load(Ordering::SeqCst),
            self.item_count.load(Ordering::SeqCst),
            bytesize::ByteSize::b(self.bytes.load(Ordering::SeqCst))
                .display()
                .si()
        );

        Ok(future.await?)
    }
}
