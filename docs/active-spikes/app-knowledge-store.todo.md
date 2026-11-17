---
attention: live
---
# app-knowledge-store tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-knowledge-store.md).

## Current scope
M0 foundation and feasibility under approved D1–D5. D6–D8 remain deferred. No publication, Figma edits or standing policy changes are part of this work.

## To Do
No unfinished task in this initial foundation slice. Broader M1+ scope remains in the concept brief and requires the next bounded implementation pass.

## Ready for Human QA
No separate manual check requested by this record; app-level visual QA is tracked in S01/S12.

## Done
- [x] Define versioned rich-document and source-reference contracts for the foundation.
- [x] Implement SQLite migrations, seeded public-safe records and revision-checked document save/Cancel behavior.
- [x] Verify rollback, stale writes, invalid documents and reopen persistence using disposable databases.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
Schema version 1, Rust-derived TypeScript types, SQLx migrations, example records, revision-checked saves and durable drafts are implemented. The initial source-excerpt contract identifies exact source IDs/revisions; full citation/provenance persistence remains S04 work. Tests cover reopening, stale/invalid writes and a forced revision-insert failure that rolls back the document head while retaining its draft. Native process restarts also preserve saved revisions and recoverable drafts.
