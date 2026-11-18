---
attention: live
---
# app-desktop-foundation tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-desktop-foundation.md).

## Current scope
The local desktop foundation and approved N1–N3 authoring controls: saved navigation, page organization, search/citation selection and workspace recovery. No publication, Figma edits or standing policy changes are part of this work.

## To Do
Human review of the new authoring controls remains open. Broader Prototype parity remains in the concept brief.

## Ready for Human QA
The foundation appearance is approved. New review-journey QA is tracked in [S05](app-proposal-review.todo.md) and [S12](app-release-quality.todo.md).

## Done
- [x] Create the app/ Tauri + Vue + TypeScript scaffold with reproducible setup and a bounded desktop command surface.
- [x] Compile and launch the actual macOS application; verify startup and shell behavior.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
The native Tauri window uses the current Getting started Prototype context and provided SVG icons. Two real SQLite-backed example pages are available. ToC controls, References/Comments switching, selection preview, the empty Chat shell, editing and history passed native checks. Search and unimplemented routes are explicitly disabled. See app/README.md for setup; no Figma file was edited.

## Human feedback — 2026-11-17
Aslan launched the native foundation and reported that it looks good. This confirms the initial visual check; the full review journey and its edge cases remain the next batch’s QA.

## Approved N1–N3 continuation — 2026-11-17
Aslan approved the authoring controls for the saved wiki tree, source search/citation picker and workspace recovery settings. The page identity remains stable when its title or location changes.

- [x] Implement create/rename/move, saved breadcrumbs, keyword search, manual citations and backup/restore controls in the native shell.
- [x] Complete the live workspace journey and native foundation/review/import regressions. Evidence and limits are in the [workspace QA record](../design/app-workspace-qa-2026-11-17.md).
