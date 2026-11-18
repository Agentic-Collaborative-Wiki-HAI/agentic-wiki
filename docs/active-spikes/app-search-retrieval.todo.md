---
attention: live
---
# Search and source selection tasks
Owner: Aslan’s agent in this implementation conversation. [Concept](app-search-retrieval.md). N2 and N5–N6 approved 2026-11-17; keyword retrieval, sourced Chat and editable answer-derived drafts.

## Implemented keyword slice
- [x] Index saved text with revision-bound passages and correct refresh after edit/rename/move.
- [x] Connect wiki search and a reusable citation/evidence picker.
- [x] Default agent context to linked sources; allow explicit additions without automatic all-workspace retrieval.
- [x] Verify long transcripts, conflicting sources, manual citations, source revisions and return paths.

## Verification
- [x] Complete the native source-picker/live-proposal check, including explicit addition and removal of other saved evidence before a real request. The live workspace journey and regression suites pass; [QA evidence](../design/app-workspace-qa-2026-11-17.md) records the boundary.

## Later
Semantic retrieval, search filters/sort, the judged retrieval corpus and external-source discovery remain deferred.

## Ready for Human QA
Try your own saved page, import and source choices. Check relevance and wording support; automated checks do not establish research validity.

## Sourced Chat and drafts — 2026-11-17
- [x] N5: quiet-on-open Chat, explicit current-page/linked-source context, optional other sources, pinned evidence inspection, request failure/retry and missing-evidence answers.
- [x] N6: answer → editable unpublished draft → explicit Save page, preserving citations and AI provenance through restart.
- [x] Verify both a labeled fixture journey and real subscription answers, including an unsupported question; rerun native review, workspace and foundation regressions. [Evidence and limits](../design/app-chat-qa-2026-11-17.md).
- [ ] Human QA of answer usefulness, source support and the draft interaction. The model returning a valid source ID does not establish that the source supports its wording.
