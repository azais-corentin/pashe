CREATE TABLE items
(
    `timestamp` DateTime('UTC') DEFAULT now() CODEC(Delta(4), ZSTD(1)),
    `league` LowCardinality(String),
    `base` LowCardinality(String),
    `name` LowCardinality(String),
    `links` UInt8,
    `ilvl` UInt8,
    `frame_type` UInt8,
    `corrupted` Bool,
    `stack_size` UInt16,
    `level` UInt8,
    `quality` UInt8,
    `passives` UInt8,
    `tier` UInt8,
    `influences` Array(LowCardinality(String)),
    `price_quantity` Float32,
    `price_currency` LowCardinality(String),
)
ENGINE = MergeTree
PARTITION BY (league, toYYYYMM(timestamp))
ORDER BY (league, frame_type, base, links, corrupted, price_quantity, ilvl);

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