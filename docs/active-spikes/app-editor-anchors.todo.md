---
attention: live
---
# app-editor-anchors tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-editor-anchors.md).

## Current scope
M0 foundation and feasibility under approved D1–D5. D6–D8 remain deferred. No publication, Figma edits or standing policy changes are part of this work.

## To Do
No unfinished task in this initial foundation slice. Broader M1+ scope remains in the concept brief and requires the next bounded implementation pass.

## Ready for Human QA
No separate manual check requested by this record; app-level visual QA is tracked in S01/S12.

## Done
- [x] Build the bounded Tiptap schema and stable block/anchor identities.
- [x] Prove the JS/Rust passage-replacement contract with Unicode, formatting, citations and malformed input.
- [x] Check actual editor Save/Cancel and selection behavior through the desktop boundary.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
The schema currently includes paragraphs, headings, bold/italic/code/link marks, hard breaks and citation atoms. Lists and other unsupported pasted structures are rejected rather than silently flattened. New blocks receive distinct IDs. Ten real ProseMirror/Rust patch fixtures and four invalid range/quote fixtures pass. The anchor helper shifts non-overlapping positions and detaches overlapping anchors; full split/merge/historical anchoring is future work. Native selection, Save/Cancel and draft recovery are verified.
