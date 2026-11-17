CREATE TABLE objects (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    parent_id TEXT REFERENCES objects(id),
    kind TEXT NOT NULL CHECK (kind IN ('page', 'recording')),
    current_revision INTEGER NOT NULL CHECK (current_revision > 0)
);
CREATE TABLE revisions (
    object_id TEXT NOT NULL REFERENCES objects(id),
    revision INTEGER NOT NULL,
    document_json TEXT NOT NULL CHECK (json_valid(document_json)),
    actor_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    PRIMARY KEY (object_id, revision)
);
CREATE TABLE drafts (
    object_id TEXT PRIMARY KEY NOT NULL REFERENCES objects(id),
    base_revision INTEGER NOT NULL,
    document_json TEXT NOT NULL CHECK (json_valid(document_json))
);
