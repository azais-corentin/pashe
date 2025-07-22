CREATE TABLE accounts
(
    `id` UInt64,
    `name` String
)
ENGINE = MergeTree
ORDER BY id;

CREATE TABLE stashes
(
    `id` FixedString(64),
    `name` String,
    `account_id` UInt64
)
ENGINE = MergeTree
ORDER BY (account_id, id);

CREATE TABLE items
(
    `id` FixedString(64),
    `timestamp` DateTime DEFAULT now(),
    `realm` LowCardinality(String),
    `stash_id` FixedString(64),
    `name` LowCardinality(String),
    `base` LowCardinality(String),
    `links` UInt8,
    `ilvl` UInt8,
    `corrupted` UInt8,
    `stack_size` UInt32,
    `level` UInt8,
    `quality` Int16,
    `passives` UInt8,
    `tier` UInt8,
    `influences` Array(LowCardinality(String))
)
ENGINE = MergeTree
PARTITION BY toYYYYMMDD(timestamp)
ORDER BY (id);

CREATE TABLE statistics_events
(
    `timestamp` DateTime DEFAULT now(),
    `stash_count` UInt64,
    `item_count` UInt64,
    `bytes` UInt64
)
ENGINE = MergeTree
ORDER BY timestamp;

CREATE TABLE statistics_per_periods
(
    `period_type` Enum('total' = 0, 'year' = 1, 'month' = 2, 'day' = 3, 'hour' = 4),
    `period_start` DateTime,
    `total_stash_count` UInt64,
    `total_item_count` UInt64,
    `total_bytes` UInt64
)
ENGINE = SummingMergeTree
ORDER BY (period_type, period_start);