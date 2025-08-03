use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{Display, EnumString};

/// Schema migration tracking
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct SchemaMigration {
    pub version: String,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Display, EnumString, Clone)]
pub enum ListingCurrency {
    #[strum(serialize = "alch")]
    AlchemyOrb,
    #[strum(serialize = "alt")]
    AlterationOrb,
    #[strum(serialize = "annul")]
    AnnulmentOrb,
    #[strum(serialize = "chance")]
    ChanceOrb,
    #[strum(serialize = "chaos")]
    ChaosOrb,
    #[strum(serialize = "divine")]
    DivineOrb,
    #[strum(serialize = "exalted")]
    ExaltedOrb,
    #[strum(serialize = "fusing")]
    FusingOrb,
    #[strum(serialize = "mirror")]
    MirrorOfKalandra,
    #[strum(serialize = "regal")]
    RegalOrb,
    #[strum(serialize = "scour")]
    ScouringOrb,
    #[default]
    #[strum(serialize = "unknown")]
    Unknown,
}

/// Individual item in a stash
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct Item {
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub timestamp: DateTime<Utc>,
    pub league: String,
    pub base: String,
    pub name: String,
    pub links: u8,
    pub ilvl: u8,
    pub frame_type: u8,
    pub corrupted: bool,
    pub stack_size: u16,
    /// For gems
    pub level: u8,
    pub quality: u8,
    /// For cluster jewels
    pub passives: u8,
    /// For maps and essences
    pub tier: u8,
    /// Base type influences
    pub influences: Vec<String>,
    /// Pricing
    pub price_quantity: f32,
    pub price_currency: String,
}

/// Statistics event tracking
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct StatisticsEvent {
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub timestamp: DateTime<Utc>,
    pub stash_count: u32,
    pub item_count: u32,
    pub compressed_bytes: u32,
    pub decompressed_bytes: u32,
}

/// Period types for statistics aggregation
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum PeriodType {
    Total,
    Year,
    Month,
    Day,
    Hour,
    Minute,
}

/// Aggregated statistics per time period
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct StatisticsPerPeriod {
    pub period_type: PeriodType,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub period_start: DateTime<Utc>,
    pub total_stash_count: u64,
    pub total_item_count: u64,
    pub total_bytes: u64,
}

/*
Use a MergeTree + SummingMergeTree with a materialized view to maintain real-time sums in ClickHouse.

https://g.co/gemini/share/5f338e009cf4
*/
