CREATE TABLE chat_turns (
    id TEXT PRIMARY KEY NOT NULL,
    page_id TEXT NOT NULL,
    page_revision INTEGER NOT NULL,
    request_json TEXT NOT NULL CHECK(json_valid(request_json)),
    evidence_json TEXT NOT NULL CHECK(json_valid(evidence_json)),
    answer_json TEXT CHECK(answer_json IS NULL OR json_valid(answer_json)),
    model TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending','completed','failed','cancelled')),
    error TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    FOREIGN KEY(page_id,page_revision) REFERENCES revisions(object_id,revision)
);
CREATE TABLE chat_page_drafts (
    id TEXT PRIMARY KEY NOT NULL,
    turn_id TEXT NOT NULL UNIQUE REFERENCES chat_turns(id),
    title TEXT NOT NULL,
    parent_id TEXT REFERENCES objects(id),
    document_json TEXT NOT NULL CHECK(json_valid(document_json)),
    version INTEGER NOT NULL DEFAULT 1,
    published_page_id TEXT UNIQUE REFERENCES objects(id),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
