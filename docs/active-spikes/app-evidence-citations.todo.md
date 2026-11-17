---
attention: live
---
# Evidence and citation persistence tasks
Opened 2026-11-17. Owner: Aslan’s agent in this implementation conversation. [Concept](app-evidence-citations.md).

## Scope
The M1 evidence contract and approved N2 manual-citation/source-selection slice. Transcription, semantic retrieval, maintenance execution, shared operation and Figma writes remain outside this batch.

## To Do
Source-change review remains a later slice. The approved manual citation authoring slice is implemented below.

## Ready for Human QA
- Review passage selection, the discussion and evidence presentation, and the accept/reject/history journey in the native app. Automated checks are recorded in [M1 verification](../design/app-review-qa-2026-11-17.md); they do not replace human QA.

## Done
- [x] Persist revision-pinned source evidence and citation records.
- [x] Preview the exact source passage and return to the same discussion.
- [x] Verify evidence identity, unknown sources, citation/revision atomicity and repeated inline numbering.
- [x] Activate the reviewed brief for the explicitly authorized review journey.

## Verification boundary
The deterministic native suite exercises Tauri/SQLite persistence with labeled test candidates. A separate opt-in native suite passes with real subscription-generated proposals and their pinned evidence. No failed request is replaced with fixture output; the live check leaves the working wiki unchanged.

## Approved N2 continuation — 2026-11-17
Manual citation authoring selects an exact saved excerpt and pins its source revision and UTF-16 range. Agent requests use linked sources by default, with explicit selection of other saved sources.

- [x] Add citation from selected article text, inspect the source, and persist the citation with its new document revision atomically.
- [x] Expose citations in References and collapsible endnotes, with a return to the inline marker.
- [x] Verify stale/invalid anchors, source pinning, Unicode boundaries and retry idempotency. The [workspace QA record](../design/app-workspace-qa-2026-11-17.md) distinguishes the passed deterministic journey from the remaining live check.
