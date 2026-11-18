---
attention: live
---
# app-editor-anchors tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-editor-anchors.md).

## Current scope
The bounded editor foundation and approved N6 editable answer-derived draft slice under D1–D5. D6–D8 remain deferred. No external publication, Figma edits or standing policy changes are part of this work.

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

## Answer-derived draft editor — 2026-11-17
- [x] Edit title, parent and rich text while retaining source citations and explicit AI origin.
- [x] Autosave unpublished drafts after 350 ms idle; flush before Keep draft and close or Save page, and reject stale versions.
- [x] Verify real native body/title edits, close/reopen, process restart, explicit publication and saved citations/history. [Chat QA](../design/app-chat-qa-2026-11-17.md).

The limited Bold/Italic toolbar and broader Prototype editor differences remain separate follow-up work.

## Approved interface correction — 2026-11-17
- [x] Implement the owning changes described in the conceptual record and verify the integrated native authoring journey in a disposable workspace.
- [ ] Human QA of the corrected interface and interaction. See [interface QA](../design/app-interface-qa-2026-11-17.md).
