---
attention: live
---
# app-knowledge-store tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-knowledge-store.md).

## Current scope
Local revisioned records, N1/N3 page organization/recovery and N5/N6 Chat/draft persistence. Stable identity, transactional writes and independent restored workspaces are required. Shared operation and external publication remain outside this batch; explicit Save publishes only to the local wiki.

## To Do
No unfinished backend task in this approved slice. Broader shared-store and migration/release work remains in the concept brief.

## Ready for Human QA
No separate manual check requested by this record; app-level visual QA is tracked in S01/S12.

## Done
- [x] Define versioned rich-document and source-reference contracts for the foundation.
- [x] Implement SQLite migrations, seeded public-safe records and revision-checked document save/Cancel behavior.
- [x] Verify rollback, stale writes, invalid documents and reopen persistence using disposable databases.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
Schema version 1, Rust-derived TypeScript types, SQLx migrations, example records, revision-checked saves and durable drafts are implemented. The initial source-excerpt contract identifies exact source IDs/revisions; full citation/provenance persistence remains S04 work. Tests cover reopening, stale/invalid writes and a forced revision-insert failure that rolls back the document head while retaining its draft. Native process restarts also preserve saved revisions and recoverable drafts.

## Approved N1/N3 continuation — 2026-11-17
The approved slice adds durable page organization and portable recovery while retaining stable record IDs and content revisions.

- [x] Store title/location events and guard against stale writes, missing parents and tree cycles.
- [x] Export a consistent backup and validate restore into a separate copy, preserving records, drafts, evidence, original imports, review decisions and history while excluding connection settings.
- [x] Verify corruption rejection, original-workspace isolation and restart persistence. See the [workspace QA record](../design/app-workspace-qa-2026-11-17.md) for native coverage and remaining checks.

## Chat and answer-derived drafts — 2026-11-17
- [x] Add migration 0006 for revision-pinned Chat turns and versioned unpublished page drafts.
- [x] Keep drafts out of the wiki tree/search; atomically publish object, first revision and origin link only on explicit Save.
- [x] Verify stale saves, idempotent creation/publication, forced transaction rollback, restart, and backup/restore of an unpublished draft and its Chat origin.

The [Chat QA record](../design/app-chat-qa-2026-11-17.md) separates backend recovery tests from the native publication journey.
