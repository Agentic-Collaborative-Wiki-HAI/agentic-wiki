---
attention: live
---
# Source import tasks
Opened 2026-11-17. Owner: Aslan’s agent in this implementation conversation. [Concept](app-artifact-intake.md).

## Scope
Approved: import existing Meet transcripts through Agent Hands and local text exports through a configured source folder. Keep unrelated design work, Figma, provider credentials and standing policies unchanged. Local transcription, binary media playback, broad retrieval and background sync remain deferred.

## Done
- [x] Persist source settings and expose them alongside the agent connection in Settings.
- [x] Add bounded local-folder discovery and an Agent Hands read/export adapter with full-text handling and useful errors.
- [x] Share preview/confirm/cancel between Add artifact and Chat; retain source provenance and managed originals.
- [x] Make imported artifacts navigable, editable and revisioned; deduplicate retries and guard source refresh against stale previews.
- [x] Test storage, adapter failures and native import/reopen flows. Record real-provider verification separately from fixtures.

## Ready for Human QA
- Inspect Settings → Sources, browse a meeting folder or local export, then preview and confirm. Check the transcript-versus-notes choice and the saved artifact. [QA evidence and limits](../design/app-import-qa-2026-11-17.md).

## Deferred
- Audio/video file management and playback from the original S07 brief.
- Local transcription, alignment and whole-transcript review attribution in S08.
- Automatic folder watchers, semantic retrieval and natural-language global Chat orchestration.

## Verification — 2026-11-17
Eight frontend checks, 34 Rust behavior checks and 27 generated-type checks pass, plus ten valid/four invalid editor-contract cases and the frontend build. Native import checks pass through the UI with synthetic local/Drive sources, including meeting subfolder navigation, Cancel, repeat import, source refresh, failure recovery, history and restart. An independently authorized real Meet document passed the production Agent Hands adapter through native IPC, complete-text comparison and restart in the temporary workspace; its mixed notes/transcript sections were preserved together. The native passage-review regression passes with labeled fixture proposals. No live model call was made for this batch. Formatting, Clippy and the ordinary desktop build pass; test instrumentation is absent from that binary.

The README covers setup and current limits. Source settings and imported text stay in the private app database; real meeting metadata and content are excluded from Git. The existing TODO index and unrelated design work remain unchanged. S07 remains active for human review and the explicitly deferred media work; completing this text-import slice does not complete the full M2 milestone.
