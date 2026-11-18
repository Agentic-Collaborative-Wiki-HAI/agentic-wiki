---
attention: live
status: active
opened: "2026-11-17"
---
# S03 · Rich-text editing and durable passage anchors
Active spike: `app-editor-anchors`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. Later milestone scope remains planned, not implemented. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0 feasibility; M1 editor; M3 broader authoring. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Select, edit and annotate real text while comments and citations remain attached to the intended passage across edits and reloads.

## Scope
Use Tiptap/ProseMirror’s open-source editor with a deliberately bounded schema: paragraphs, headings, lists, basic marks, links and citation references. Add stable block IDs, revision-bound ranges, quote/context fallback, edit transaction maps and explicit detached-anchor states. Implement direct edits, draft recovery, selection menus, undo/redo and the editor side of human tracked suggestions.

Out of scope: No paid review/comment service assumed, whole-document HTML as the authoritative record, silent fuzzy reattachment, or CRDT implementation. Unsupported formatting must be rejected or explicitly normalized, never silently discarded.

## Design basis
[Read-mode selection](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-91088&scaling=scale-down-width&hide-ui=1) · [Edit-mode selection](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-91188&scaling=scale-down-width&hide-ui=1) · [Track changes](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89918&scaling=scale-down-width&hide-ui=1) · [Long-page editor](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-99743&scaling=scale-down-width&hide-ui=1). Read selection offers discussion; editing offers links, citations, notes and comments. [#43](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/43) supplies the regression: Cancel after an accepted edit must retain the accepted page.

## Dependencies and interface ownership
Completion prerequisites: S02. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own the versioned editor schema and passage-operation/anchor contracts. Preserve the original anchor and captured quote in history. S02 validates and stores records; S04 renders evidence; S05 renders proposed edits as a separate review layer. Decide the JS/Rust patch boundary through a small conformance probe, not an assumption that ProseMirror executes in Rust.

## Rough task sequence
1. Probe JSON round trips and one bounded paragraph replacement across Vue/Rust, including Unicode, marks, citations and malformed input.
2. Implement selection anchors, transaction mapping and restoration; distinguish mapped, ambiguous and removed passages.
3. Implement Save/Cancel, keyboard undo, formatting and recovery with clear draft/published boundaries.
4. Exercise overlapping notes, block split/merge, deletion, duplicate quotes, copy/paste and tracked human edits with S04/S05.

## Acceptance criteria
- An edit before an anchored passage moves the annotation correctly; deleting the passage preserves a readable historical annotation and flags the missing current target.
- Emoji, combining characters, repeated text and paragraph splits cannot silently attach a note to a different passage.
- Save/Cancel and undo preserve the current published revision; Cancel cannot restore the old spreadsheet wording after acceptance.
- The editor, Rust validation and saved representation agree on positions and allowed structure.

## Automated and integration checks
A shared fixture corpus tests editor serialization, validated patch application and anchor mappings across JS/Rust. Include UTF-16 versus Rust byte/index conventions explicitly. Desktop tests cover native selection, IME/composition, clipboard and focus; pure model tests cannot establish those behaviors.

## Human QA
Aslan selects, edits and comments in the real Mac app, including a long document. Inspect removed/ambiguous annotation messaging and whether selection controls feel usable.

## Risks, choices and recommendation
Free editor core does not include the complete proposal system. Recommend custom, limited extensions and a single-paragraph proposal operation first, retaining a path to richer edits. If equivalent validated patch behavior in Rust is not feasible, return with a concrete alternative before adding a JS sidecar or weakening validation.

## Independent work and gates
The editor probe can use in-memory records before S02 persistence is ready. Persistent anchors need the shared S02 contract. S05 cannot be considered reliable until this boundary passes.


## Current activation
The M0 foundation and bounded M1 review implementation are checked in the companion task records and [native review QA](../design/app-review-qa-2026-11-17.md). The full passage-to-decision route passes deterministic and live subscription checks. The Codex coding-session path remains disabled; product inference uses the isolated app-owned connection. Hands-on M1 review remains open. Broader milestones and D6–D8 remain deferred.

Continues from: docs/active-spikes/implementation-planning.md

## Current interface and authoring boundary
Page titles are edited alongside the document and saved as one revision. Draft recovery includes the title, and Cancel preserves the saved title and document. The selection popup offers explicit citations and margin notes in both read and edit modes. In edit mode these insert immutable reference identities into the unpublished draft; Save publishes them and Cancel discards their occurrences. Registering reference metadata alone does not change a page. Paragraph and inline-node offsets remain shared with Rust.

See [interface QA](../design/app-interface-qa-2026-11-17.md) for checked cases and remaining limits.
