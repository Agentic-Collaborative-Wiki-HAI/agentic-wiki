---
attention: live
status: active
opened: "2026-11-17"
---
# S02 · Knowledge records, revisions and local storage
Active spike: `app-knowledge-store`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. Later milestone scope remains planned, not implemented. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0–M1; migration and backup support through M5. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Pages, evidence, decisions and conversations persist together, with an inspectable record of what changed and who acted.

## Scope
Use SQLite with SQLx migrations, schema-versioned document JSON, explicit object/revision identities, and original artifact bytes in managed local storage. Model pages, artifacts, transcript revisions, citations, anchors, threads/messages, proposals/decisions, actor attribution and job records. Implement only the required tables/commands as consuming slices arrive. Keep projection/index data rebuildable.

Out of scope: No event-sourcing framework, graph database, generic ontology engine, collaborative synchronization, or runtime data in the source repository. Markdown export is not a promise of lossless Markdown round trips.

## Design basis
[Decision history](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-98958&scaling=scale-down-width&hide-ui=1) · [Booking source history](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-104202&scaling=scale-down-width&hide-ui=1) · [Recording history](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-102352&scaling=scale-down-width&hide-ui=1). History distinguishes content revisions, review decisions, transcript reviews, discussion resolution and delegated repairs. Rejection is an event without a new content revision.

## Dependencies and interface ownership
Completion prerequisites: No completed feature prerequisite; contracts can be drafted against fixtures. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own ObjectId, RevisionId, ActorId, document-schema version, atomic commit and optimistic concurrency contracts. S03 owns editing operations/anchor mapping; S04 owns evidence-reference semantics; S05 owns proposal transitions; S11 owns reviewer authorization. S02 enforces their persisted invariants in the same transaction. A content revision is immutable; drafts are separately mutable and never mislabeled as published content.

## Rough task sequence
1. Agree the canonical storage/export choice and define the minimal typed records with S03–S05.
2. Create a fictional corpus based on the approved public design examples, including distinct Getting started and Using the microscope pages and Booking sheet revisions 4/5.
3. Implement compare-and-swap saves, atomic proposal decisions, persistent draft recovery and explicit error results.
4. Add migration fixtures, consistent snapshots, integrity checks and the storage side of S12 export/restore.

## Acceptance criteria
- Restart retains saved content, drafts, source references and decision history without creating duplicate records.
- A failed acceptance writes neither partial wording nor a partial citation change; repeated acceptance requests are idempotent.
- Unknown document versions and migrations fail safely; a migration failure leaves a recoverable original.
- Stable IDs survive title changes and tree moves. Figma hierarchy addresses are navigation labels, not database primary keys.

## Automated and integration checks
Rust tests use disposable databases for transaction rollback, duplicate commands, expected-revision failure and migrations from every supported fixture version. Verify a backup restores the same object IDs, revision links and attachment checksums.

## Human QA
Review the history vocabulary and a restored workspace. Confirm what is a draft versus a saved revision and whether attribution reads honestly.

## Risks, choices and recommendation
Database-first documents make relational integrity simpler but reduce direct editing with ordinary text tools. Recommend JSON plus SQLite as canonical, with portable full-fidelity export and readable Markdown exports; a file-first design needs a different conflict and metadata plan. Resolve this before schema implementation.

## Independent work and gates
Schema and transaction experiments need no shell or model call. S03’s schema probe should occur alongside this work; neither can declare the document contract final alone.


## Current activation
The first M0 foundation/check slice is implemented and documented in the companion task record. The native shell, persistence and bounded editor contract have passing evidence; Codex generation remains disabled because isolation was not proven. M1 and later acceptance criteria remain future scope. D6–D8 are deferred.

Continues from: docs/active-spikes/implementation-planning.md
