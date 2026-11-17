CREATE TABLE app_settings (
    key TEXT PRIMARY KEY NOT NULL,
    value_json TEXT NOT NULL CHECK (json_valid(value_json))
);
-- Imported text is a document object. Its source-specific type is separate from the legacy page/recording family.
CREATE TABLE imported_sources (
    object_id TEXT PRIMARY KEY NOT NULL REFERENCES objects(id),
    provider TEXT NOT NULL CHECK (provider IN ('local','drive')),
    locator TEXT NOT NULL,
    content_kind TEXT NOT NULL CHECK (content_kind IN ('transcript','notes')),
    UNIQUE(provider,locator)
);
CREATE TABLE import_snapshots (
    object_id TEXT NOT NULL,
    revision INTEGER NOT NULL,
    sha256 TEXT NOT NULL,
    original_bytes BLOB NOT NULL,
    source_name TEXT NOT NULL,
    source_url TEXT,
    source_modified TEXT,
    imported_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    PRIMARY KEY(object_id,revision),
    FOREIGN KEY(object_id,revision) REFERENCES revisions(object_id,revision)
);
CREATE TABLE import_requests (
    id TEXT PRIMARY KEY NOT NULL,
    object_id TEXT NOT NULL REFERENCES imported_sources(object_id)
);
