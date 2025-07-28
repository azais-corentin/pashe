CREATE TABLE items
(
    `timestamp` DateTime DEFAULT now(),
    `league` LowCardinality(String),
    `stash_name` String,
    `account_name` String,
    `name` LowCardinality(String),
    `type_line` LowCardinality(String),
    `base` LowCardinality(String),
    `links` UInt8,
    `ilvl` UInt8,
    `corrupted` Bool,
    `stack_size` UInt32,
    `level` UInt8,
    `quality` UInt8,
    `passives` UInt8,
    `tier` UInt8,
    `influences` Array(LowCardinality(String))
)
ENGINE = MergeTree
PARTITION BY (league, toYYYYMMDD(timestamp))
ORDER BY (league, base, links, ilvl, corrupted);

CREATE TABLE statistics_events
(
    `timestamp` DateTime DEFAULT now(),
    `stash_count` UInt32,
    `item_count` UInt32,
    `compressed_bytes` UInt32,
    `decompressed_bytes` UInt32
)
ENGINE = MergeTree
ORDER BY timestamp;

CREATE TABLE statistics_per_periods
(
    `period_type` Enum8('total' = 0, 'year' = 1, 'month' = 2, 'day' = 3, 'hour' = 4, 'minute' = 5),
    `period_start` DateTime,
    `total_stash_count` UInt64,
    `total_item_count` UInt64,
    `total_compressed_bytes` UInt64,
    `total_decompressed_bytes` UInt64
)
ENGINE = SummingMergeTree
ORDER BY (period_type, period_start);

-- Create a materialized view to aggregate statistics per period
CREATE MATERIALIZED VIEW statistics_per_periods_mv
TO statistics_per_periods
AS
SELECT
    periods.1 AS period_type,
    periods.2 AS period_start,
    sum(stash_count) AS total_stash_count,
    sum(item_count) AS total_item_count,
    sum(compressed_bytes) AS total_compressed_bytes,
    sum(decompressed_bytes) AS total_decompressed_bytes
FROM
    statistics_events
ARRAY JOIN
    [
        ('minute', toStartOfMinute(timestamp)),
        ('hour',   toStartOfHour(timestamp)),
        ('day',    toStartOfDay(timestamp)),
        ('month',  toStartOfMonth(timestamp)),
        ('year',   toStartOfYear(timestamp)),
        ('total',  toDateTime(0))
    ] AS periods
GROUP BY
    period_type,
    period_start;