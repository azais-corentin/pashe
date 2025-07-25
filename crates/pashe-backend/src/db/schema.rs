use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Schema migration tracking
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct SchemaMigration {
    pub version: String,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub applied_at: DateTime<Utc>,
}

/// Account information
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub name: String,
}

/// Stash information linked to an account
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct Stash {
    /// Fixed 64-character identifier
    #[serde(with = "serde_bytes")]
    pub id: [u8; 64],
    pub name: String,
    pub account_id: u64,
}

/// Individual item in a stash
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct Item {
    /// Fixed 64-character identifier
    #[serde(with = "serde_bytes")]
    pub id: [u8; 64],
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub timestamp: DateTime<Utc>,
    pub realm: String,
    /// References stashes.id
    #[serde(with = "serde_bytes")]
    pub stash_id: [u8; 64],
    pub name: String,
    pub base: String,
    pub links: u8,
    pub ilvl: u8,
    /// Note: ClickHouse uses UInt8 instead of bool for corrupted
    pub corrupted: u8,
    pub stack_size: u32,
    /// For gems
    pub level: u8,
    pub quality: u8,
    /// For cluster jewels
    pub passives: u8,
    /// For maps and essences
    pub tier: u8,
    /// Base type influences
    pub influences: Vec<String>,
}

/// Statistics event tracking
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct StatisticsEvent {
    #[serde(skip_serializing, with = "clickhouse::serde::chrono::datetime")]
    pub timestamp: DateTime<Utc>,
    pub stash_count: u32,
    pub item_count: u32,
    pub bytes: u32,
}

/// Period types for statistics aggregation
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum PeriodType {
    Total = 0,
    Year = 1,
    Month = 2,
    Day = 3,
    Hour = 4,
    Minute = 5,
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
