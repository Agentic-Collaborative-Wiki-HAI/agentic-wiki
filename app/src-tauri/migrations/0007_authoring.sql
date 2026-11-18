CREATE TABLE margin_notes (id TEXT PRIMARY KEY NOT NULL, title TEXT NOT NULL, body TEXT NOT NULL, created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')));
CREATE TABLE agent_instructions (id INTEGER PRIMARY KEY CHECK(id=1), proposal TEXT NOT NULL, chat TEXT NOT NULL);
ALTER TABLE drafts ADD COLUMN title TEXT;
ALTER TABLE revisions ADD COLUMN title TEXT;
UPDATE revisions SET title=(SELECT title FROM objects WHERE objects.id=revisions.object_id);

CREATE TRIGGER snapshot_revision_title AFTER INSERT ON revisions WHEN NEW.title IS NULL BEGIN
  UPDATE revisions SET title=(SELECT title FROM objects WHERE id=NEW.object_id) WHERE object_id=NEW.object_id AND revision=NEW.revision;
END;
