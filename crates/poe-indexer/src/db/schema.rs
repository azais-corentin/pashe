/*
Table accounts {
  id UInt64 [increment]
  name string [note: 'accountName']
}

Table stashes {
  id FixedString(64)
  name String
  account_id UInt64 [ref: > accounts.id]
}

Table items {
  id FixedString(64)
  timestamp DateTime [default: `now()`]
  realm LowCardinality(String)
  stash_id FixedString(64) [ref: > stashes.id]
  name LowCardinality(String)
  base LowCardinality(String)
  links UInt8
  ilvl UInt8
  corrupted bool
  stack_size UInt32
  // For gems
  level UInt8
  quality Int16
  // cluster jewels
  passives UInt8
  // maps and essences
  tier UInt8
  // base types
  influences "Array(LowCardinality(String))"
}

Table statistics_events {
  timestamp DateTime [default: `now()`]
  stash_count UInt64
  item_count UInt64
  bytes UInt64
}

Enum period_types {
  total
  year
  day
  hour
}

Table statistics_per_periods {
  period_type period_types
  period_start DateTime
  total_stash_count UInt64
  total_item_count UInt64
  total_bytes UInt64
}
*/

/*
Use a MergeTree + SummingMergeTree with a materialized view to maintain real-time sums in ClickHouse.

https://g.co/gemini/share/5f338e009cf4
*/
