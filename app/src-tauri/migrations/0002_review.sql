CREATE TABLE threads (
    id TEXT PRIMARY KEY NOT NULL,
    page_id TEXT NOT NULL REFERENCES objects(id),
    base_revision INTEGER NOT NULL,
    anchor_json TEXT NOT NULL CHECK (json_valid(anchor_json)),
    resolved INTEGER NOT NULL DEFAULT 0 CHECK (resolved IN (0,1)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    FOREIGN KEY(page_id,base_revision) REFERENCES revisions(object_id,revision)
);
CREATE TABLE messages (
    id TEXT PRIMARY KEY NOT NULL,
    thread_id TEXT NOT NULL REFERENCES threads(id),
    actor_id TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE TABLE proposals (
    id TEXT PRIMARY KEY NOT NULL,
    thread_id TEXT NOT NULL REFERENCES threads(id),
    parent_id TEXT REFERENCES proposals(id),
    base_revision INTEGER NOT NULL,
    anchor_json TEXT NOT NULL CHECK (json_valid(anchor_json)),
    candidate_json TEXT NOT NULL CHECK (json_valid(candidate_json)),
    evidence_json TEXT NOT NULL CHECK (json_valid(evidence_json)),
    request_text TEXT NOT NULL,
    origin TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','accepted','rejected','superseded')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE UNIQUE INDEX one_pending_proposal_per_thread ON proposals(thread_id) WHERE status='pending';
CREATE TABLE review_decisions (
    proposal_id TEXT PRIMARY KEY NOT NULL REFERENCES proposals(id),
    decision TEXT NOT NULL CHECK (decision IN ('accepted','rejected')),
    actor_id TEXT NOT NULL,
    published_revision INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE TABLE citations (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    source_revision INTEGER NOT NULL,
    block_id TEXT NOT NULL,
    quote TEXT NOT NULL,
    FOREIGN KEY(source_id,source_revision) REFERENCES revisions(object_id,revision)
);
