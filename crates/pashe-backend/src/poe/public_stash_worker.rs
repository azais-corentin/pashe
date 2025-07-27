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

#[derive(Debug)]
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
    #[tracing::instrument(skip_all, level = "trace")]
    pub async fn fetch_stash(
        self: Arc<Self>,
        client: Arc<reqwest_middleware::ClientWithMiddleware>,
        change_id: String,
        next_change_id_tx: mpsc::UnboundedSender<String>,
        stash_changes_tx: mpsc::UnboundedSender<(PublicStashTabs, u32)>,
    ) -> Result<()> {
        debug!("Fetching change id: {}", change_id);

        let url = format!("{BASE_URL}/public-stash-tabs?id={change_id}");

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
    #[tracing::instrument(skip_all, level = "trace")]
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

            let mut items = Vec::new();
            for stash in stash_changes.stashes.iter() {
                for item in stash.items.iter() {
                    let league = match &item.league {
                        Some(league) => league.clone(),
                        None => {
                            debug!("Skipping item without league: {:?}", item);
                            continue;
                        }
                    };
                    let (level, quality) = extract_gem_properties(item);
                    let influences = extract_influences(item);
                    let (passives, tier) = extract_passives_and_tier(item);

                    items.push(db::Item {
                        timestamp: Utc::now(),
                        league,
                        stash_name: stash.stash.clone().unwrap_or_default(),
                        account_name: stash.account_name.clone().unwrap_or_default(),
                        name: item.name.clone(),
                        type_line: item.type_line.clone(),
                        base: item.base_type.clone(),
                        links: count_links(item),
                        ilvl: item.ilvl.max(0) as u8,
                        corrupted: item.corrupted.unwrap_or(false),
                        stack_size: item.stack_size.unwrap_or(1).max(1) as u32,
                        level,
                        quality,
                        passives,
                        tier,
                        influences,
                    });
                }
            }

            if let Err(e) = db.insert_items(items).await {
                error!("Failed to insert items: {}", e);
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
                local_stash_count.human_count_bare(),
                self.stash_count.load(Ordering::SeqCst).human_count_bare(),
                local_item_count.human_count_bare(),
                self.item_count.load(Ordering::SeqCst).human_count_bare(),
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

/// Extract gem level and quality from item properties
fn extract_gem_properties(item: &crate::poe::types::Item) -> (u8, u8) {
    let mut level = 0u8;
    let mut quality = 0u8;

    if let Some(properties) = &item.properties {
        for prop in properties {
            match prop.name.as_str() {
                "Level" => {
                    if let Some((value, _)) = prop.values.first() {
                        level = value.parse().unwrap_or(0).min(255) as u8;
                    }
                }
                "Quality" => {
                    if let Some((value, _)) = prop.values.first() {
                        // Remove the % symbol if present
                        let clean_value = value.trim_end_matches('%');
                        quality = clean_value.parse().unwrap_or(0).min(255) as u8;
                    }
                }
                _ => {}
            }
        }
    }

    (level, quality)
}

/// Extract influences from item
fn extract_influences(item: &crate::poe::types::Item) -> Vec<String> {
    let mut influences = Vec::new();

    // Check the influences field
    if let Some(inf) = &item.influences {
        if inf.elder.unwrap_or(false) {
            influences.push("Elder".to_string());
        }
        if inf.shaper.unwrap_or(false) {
            influences.push("Shaper".to_string());
        }
        if inf.searing.unwrap_or(false) {
            influences.push("Searing".to_string());
        }
        if inf.tangled.unwrap_or(false) {
            influences.push("Tangled".to_string());
        }
    }

    // Check legacy influence fields
    if item.elder.unwrap_or(false) {
        influences.push("Elder".to_string());
    }
    if item.shaper.unwrap_or(false) {
        influences.push("Shaper".to_string());
    }

    influences
}

/// Extract cluster jewel passives and map/essence tier
fn extract_passives_and_tier(item: &crate::poe::types::Item) -> (u8, u8) {
    let mut passives = 0u8;
    let mut tier = 0u8;

    if let Some(properties) = &item.properties {
        for prop in properties {
            match prop.name.as_str() {
                "Added Small Passive Skills" | "Added Passives" => {
                    if let Some((value, _)) = prop.values.first() {
                        passives = value.parse().unwrap_or(0).min(255) as u8;
                    }
                }
                "Map Tier" | "Tier" => {
                    if let Some((value, _)) = prop.values.first() {
                        tier = value.parse().unwrap_or(0).min(255) as u8;
                    }
                }
                _ => {}
            }
        }
    }

    // For maps, also check if it's in the type line
    if item.type_line.contains("Map") {
        // Try to extract tier from implicit mods or other sources
        if let Some(implicit_mods) = &item.implicit_mods {
            for mod_text in implicit_mods {
                if mod_text.contains("Tier") {
                    // Try to extract number from the mod text using simple parsing
                    if let Some(tier_start) = mod_text.find("Tier ") {
                        let after_tier = &mod_text[tier_start + 5..];
                        if let Some(space_pos) = after_tier.find(' ') {
                            let tier_str = &after_tier[..space_pos];
                            tier = tier_str.parse().unwrap_or(0).min(255) as u8;
                        } else {
                            // If no space, try to parse the rest
                            let tier_str = after_tier
                                .chars()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>();
                            tier = tier_str.parse().unwrap_or(0).min(255) as u8;
                        }
                    }
                }
            }
        }
    }

    (passives, tier)
}

/// Count socket links in an item
fn count_links(item: &crate::poe::types::Item) -> u8 {
    let mut max_link_group = 0u8;

    if let Some(sockets) = &item.sockets {
        let mut group_counts = std::collections::HashMap::new();

        for socket in sockets {
            let count = group_counts.entry(socket.group).or_insert(0);
            *count += 1;
        }

        max_link_group = group_counts.values().max().copied().unwrap_or(0).min(255) as u8;
    }

    max_link_group
}
