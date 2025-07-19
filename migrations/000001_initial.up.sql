CREATE TABLE items (timestamp DateTime, name String) ENGINE = MergeTree()
ORDER BY timestamp;