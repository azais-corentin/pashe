/*
Table accounts {
  id integer [increment]
  name string [note: 'accountName']
}

Table stashes {
  id FixedString(64)
  name String
  account_id integer [ref: > accounts.id]
}

Table items {
  id FixedString(64)
  timestamp datetime [default: `now()`]
  realm LowCardinality(String)
  stash_id FixedString(64) [ref: > stashes.id]
  name LowCardinality(String)
  base LowCardinality(String)
  links integer
  ilvl integer
  corrupted bool
  stack_size integer
  // For gems
  level integer
  quality integer
  // cluster jewels
  passives integer
  // maps and essences
  tier integer
  // base types
  influences "Array(LowCardinality(String))"
}
*/
