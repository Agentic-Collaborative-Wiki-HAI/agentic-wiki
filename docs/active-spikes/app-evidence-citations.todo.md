---
attention: live
---
# Evidence and citation persistence tasks
Opened 2026-11-17. Owner: Aslan’s agent in this implementation conversation. [Concept](app-evidence-citations.md).

## Scope
Authorized M1 batch under D1–D5. Transcription, semantic retrieval, maintenance execution, shared operation and Figma writes remain outside this batch.

## To Do
Broader manual citation authoring and source-change review remain later slices of the concept brief.

## Ready for Human QA
- Review passage selection, the discussion and evidence presentation, and the accept/reject/history journey in the native app. Automated checks are recorded in [M1 verification](../design/app-review-qa-2026-11-17.md); they do not replace human QA.

## Done
- [x] Persist revision-pinned source evidence and citation records.
- [x] Preview the exact source passage and return to the same discussion.
- [x] Verify evidence identity, unknown sources, citation/revision atomicity and repeated inline numbering.
- [x] Activate the reviewed brief for the explicitly authorized review journey.

## Verification boundary
The native suite exercises actual Tauri/SQLite persistence with an explicitly labeled test candidate. Live model generation remains gated on the app-owned sign-in; no failed request is replaced with fixture output.
