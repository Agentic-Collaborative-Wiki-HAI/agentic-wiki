ALTER TABLE objects ADD COLUMN metadata_version INTEGER NOT NULL DEFAULT 1;
CREATE TABLE object_events (
    id TEXT PRIMARY KEY NOT NULL,
    object_id TEXT NOT NULL REFERENCES objects(id),
    title TEXT NOT NULL,
    parent_id TEXT REFERENCES objects(id),
    metadata_version INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
ALTER TABLE citations ADD COLUMN range_json TEXT CHECK (range_json IS NULL OR json_valid(range_json));
CREATE VIRTUAL TABLE passage_search USING fts5(title, body, object_id UNINDEXED, revision UNINDEXED, block_id UNINDEXED, start_utf16 UNINDEXED, end_utf16 UNINDEXED, tokenize='unicode61 remove_diacritics 2');
CREATE TABLE search_versions (
    object_id TEXT PRIMARY KEY NOT NULL REFERENCES objects(id),
    revision INTEGER NOT NULL,
    title TEXT NOT NULL
);
