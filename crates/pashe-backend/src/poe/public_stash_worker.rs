use crate::{
    db::{self, StatisticsEvent},
    poe::{constants::BASE_URL, types::PublicStashTabs},
};
use anyhow::{Context, Result};
use chrono::Utc;
use human_repr::HumanCount;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

pub struct PublicStashWorker {
    shutdown_token: CancellationToken,
    stash_count: AtomicU64,
    item_count: AtomicU64,
    bytes: AtomicU64,
}

impl PublicStashWorker {
    pub fn new(shutdown_token: CancellationToken) -> Self {
        PublicStashWorker {
            shutdown_token,
            stash_count: AtomicU64::new(0),
            item_count: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
        }
    }

    /// Crawls a single stash change and sends the next change ID and stash data to respective queues
    pub async fn fetch_stash(
        self: Arc<Self>,
        client: Arc<reqwest_middleware::ClientWithMiddleware>,
        change_id: String,
        next_change_id_tx: mpsc::UnboundedSender<String>,
        stash_changes_tx: mpsc::UnboundedSender<(PublicStashTabs, u32)>,
    ) -> Result<()> {
        let url = format!("{BASE_URL}/public-stash-tabs?id={change_id}");
        debug!("Fetching change id: {}", change_id);

        let response = client.get(url.clone()).send().await?;

        if response.status() != reqwest::StatusCode::OK {
            let status = response.status();
            error!("Failed to fetch public stashes: HTTP {}", status);
            return Err(anyhow::anyhow!(
                "Failed to fetch public stashes: HTTP {}",
                status
            ));
        }

        // Extract and send the next change ID as soon as headers are available
        let next_change_id = response
            .headers()
            .get("x-next-change-id")
            .ok_or(anyhow::anyhow!(
                "Missing x-next-change-id header in response from {}",
                url
            ))?
            .to_str()?
            .to_owned();

        // Send the next change ID immediately
        if next_change_id_tx.send(next_change_id).is_err() {
            debug!("Next change ID receiver dropped");
            return Ok(());
        }

        // Now get the body and parse JSON
        let text_body = response
            .text()
            .await
            .with_context(|| format!("Failed to read response body from {url}"))?;

        let stash_changes = serde_json::from_str::<PublicStashTabs>(&text_body)
            .with_context(|| format!("Failed to parse response body from {url}"))?;

        // Send the parsed stash data along with byte count
        if stash_changes_tx
            .send((stash_changes, text_body.len() as u32))
            .is_err()
        {
            debug!("Stash changes receiver dropped");
        }

        Ok(())
    }

    /// Processes stash changes from the queue and updates statistics
    pub async fn process_stash(
        self: Arc<Self>,
        mut stash_changes_rx: mpsc::UnboundedReceiver<(PublicStashTabs, u32)>,
        db: db::Client,
    ) {
        while let Some((stash_changes, byte_count)) = stash_changes_rx.recv().await {
            if self.shutdown_token.is_cancelled() {
                debug!("Shutting down stash processor");
                break;
            }

            let local_stash_count = stash_changes.stashes.len() as u32;
            let local_item_count: u32 = stash_changes
                .stashes
                .iter()
                .map(|stash| stash.items.len() as u32)
                .sum();

            // Update statistics
            self.stash_count
                .fetch_add(local_stash_count as u64, Ordering::SeqCst);
            self.item_count
                .fetch_add(local_item_count as u64, Ordering::SeqCst);
            self.bytes.fetch_add(byte_count as u64, Ordering::SeqCst);

            debug!(
                "Processed batch: {}/{} stashes / {}/{} items / {}/{}",
                local_stash_count,
                self.stash_count.load(Ordering::SeqCst),
                local_item_count,
                self.item_count.load(Ordering::SeqCst),
                byte_count.human_count_bytes(),
                self.bytes.load(Ordering::SeqCst).human_count_bytes()
            );

            if let Err(e) = db
                .insert_statistics_event(StatisticsEvent {
                    timestamp: Utc::now(),
                    stash_count: local_stash_count,
                    item_count: local_item_count,
                    bytes: byte_count,
                })
                .await
            {
                error!("Failed to insert statistics event: {}", e);
            }
        }
    }
}
