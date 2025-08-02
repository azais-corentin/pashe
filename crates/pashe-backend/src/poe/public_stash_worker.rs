use crate::{
    db::{self, ListingCurrency, StatisticsEvent},
    poe::{constants::BASE_URL, types::PublicStashTabs},
};
use anyhow::{Context, Result};
use async_compression::tokio::bufread::GzipDecoder;
use chrono::Utc;
use futures_util::StreamExt;
use human_repr::{HumanCount, HumanDuration, HumanThroughput};
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, debug, error, info};
use winnow::prelude::*;
use winnow::{ascii::multispace1, combinator::alt, token::take_while};

#[derive(Debug)]
pub struct PublicStashWorker {
    shutdown_token: CancellationToken,
}

impl PublicStashWorker {
    pub fn new(shutdown_token: CancellationToken) -> Self {
        PublicStashWorker { shutdown_token }
    }

    /// Crawls a single stash change and sends the next change ID and stash data to respective queues
    #[tracing::instrument(skip_all, level = "trace")]
    pub async fn fetch_stash(
        self: Arc<Self>,
        client: Arc<reqwest_middleware::ClientWithMiddleware>,
        change_id: String,
        next_change_id_tx: mpsc::UnboundedSender<String>,
        stash_changes_tx: mpsc::UnboundedSender<(PublicStashTabs, u32, u32)>,
    ) -> Result<()> {
        debug!("Fetching change id: {}", change_id);

        let url = format!("{BASE_URL}/public-stash-tabs?id={change_id}");

        let response = client.get(url.clone()).send().await?;

        if response.status() != reqwest::StatusCode::OK {
            let status = response.status();
            error!("Failed to fetch public stashes: HTTP {}", status);

            // Retry
            info!("Retrying with change ID: {}", change_id);
            if next_change_id_tx.send(change_id).is_err() {
                debug!("Failed to send next change ID, receiver dropped");
                return Ok(());
            }

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
        let mut compressed_bytes = 0u32;

        // Get the response as a byte stream
        let mut bytes_stream = response.bytes_stream();
        let mut compressed_data = Vec::new();

        // Collect all compressed bytes while counting them
        while let Some(chunk) = bytes_stream.next().await {
            let chunk = chunk.with_context(|| format!("Failed to read chunk from {url}"))?;
            compressed_bytes += chunk.len() as u32;
            compressed_data.extend_from_slice(&chunk);
        }

        // Create a stream reader from the compressed data
        let compressed_reader = std::io::Cursor::new(compressed_data);
        let buf_reader = tokio::io::BufReader::new(compressed_reader);

        // Decompress using gzip
        let mut gzip_decoder = GzipDecoder::new(buf_reader);
        let mut decompressed_data = Vec::new();

        // Read all decompressed data
        gzip_decoder
            .read_to_end(&mut decompressed_data)
            .instrument(tracing::trace_span!("decompress_gzip"))
            .await
            .with_context(|| format!("Failed to decompress gzip response from {url}"))?;

        let decompressed_bytes = decompressed_data.len() as u32;

        // Convert decompressed bytes to string
        let text_body = String::from_utf8(decompressed_data)
            .with_context(|| format!("Failed to convert decompressed data to UTF-8 from {url}"))?;

        let stash_changes = serde_json::from_str::<PublicStashTabs>(&text_body)
            .with_context(|| format!("Failed to parse response body from {url}"))?;

        // Send the parsed stash data along with compressed byte count
        if stash_changes_tx
            .send((stash_changes, compressed_bytes, decompressed_bytes))
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
        mut stash_changes_rx: mpsc::UnboundedReceiver<(PublicStashTabs, u32, u32)>,
        db: db::Client,
    ) {
        while let Some((stash_changes, compressed_bytes, decompressed_bytes)) =
            stash_changes_rx.recv().await
        {
            if self.shutdown_token.is_cancelled() {
                debug!("Shutting down stash processor");
                break;
            }

            let start_time = std::time::Instant::now();

            let mut items = Vec::new();
            for stash in stash_changes.stashes.iter() {
                let stash_price = extract_price(stash.stash.as_ref());

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
                    let item_price = extract_price(item.note.as_ref());

                    let final_price = if let Some(item_price) = item_price {
                        item_price
                    } else if let Some(stash_price_ref) = stash_price.as_ref() {
                        stash_price_ref.clone()
                    } else {
                        continue;
                    };

                    items.push(db::Item {
                        timestamp: Utc::now(),
                        league,
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
                        note: item
                            .note
                            .clone()
                            .or(stash.stash.clone())
                            .unwrap_or_default(),
                        price_quantity: final_price.quantity,
                        price_currency: final_price.currency.to_string(),
                    });
                }
            }

            let end_time = std::time::Instant::now();

            if !items.is_empty() {
                debug!(
                    "Processed {} items in {} ({}, {}/item)",
                    items.len().human_count_bare(),
                    (end_time - start_time).human_duration(),
                    (items.len() as f64 / (end_time - start_time).as_secs_f64())
                        .human_throughput("items"),
                    ((end_time - start_time).as_secs_f64() / items.len() as f64).human_duration()
                );
            }

            if let Err(e) = db.insert_items(items).await {
                error!("Failed to insert items: {}", e);
            }

            let stash_count = stash_changes.stashes.len() as u32;
            let item_count: u32 = stash_changes
                .stashes
                .iter()
                .map(|stash| stash.items.len() as u32)
                .sum();

            debug!(
                "Processed batch: {} stashes / {} items / {}/{} bytes ({:.1}:1 ratio)",
                stash_count.human_count_bare(),
                item_count.human_count_bare(),
                compressed_bytes.human_count_bytes(),
                decompressed_bytes.human_count_bytes(),
                decompressed_bytes as f64 / compressed_bytes as f64,
            );

            if let Err(e) = db
                .insert_statistics_event(StatisticsEvent {
                    timestamp: Utc::now(),
                    stash_count,
                    item_count,
                    compressed_bytes,
                    decompressed_bytes,
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

#[derive(Clone)]
struct ListingPrice {
    quantity: f32,
    currency: ListingCurrency,
}

fn extract_price(note: Option<&String>) -> Option<ListingPrice> {
    let note = note?;

    // The parser is defined as a sequence of smaller parsers using a tuple.
    // This is an idiomatic way to define a sequence in `winnow`.
    // 1. Prefix: `~price` or `~b/o`
    // 2. Whitespace: one or more space characters.
    // 3. Amount: a decimal unsigned 32-bit integer.
    // 4. Whitespace: one or more space characters.
    // 5. Currency Name: a string of alphanumeric characters and hyphens.
    let mut parser = (
        alt(("~price", "~b/o")),
        multispace1::<_, winnow::error::InputError<_>>,
        winnow::ascii::float::<_, f32, winnow::error::InputError<_>>,
        multispace1::<_, winnow::error::InputError<_>>,
        take_while(1.., |c: char| c.is_alphanumeric() || c == '-'),
    );

    let (_, _, amount, _, currency_str) = parser.parse_next(&mut note.as_str()).ok()?;

    let currency = ListingCurrency::from_str(currency_str).ok()?;

    // debug!("Extracted price: {} => {} {}", note, amount, currency);

    Some(ListingPrice {
        quantity: amount,
        currency,
    })
}
