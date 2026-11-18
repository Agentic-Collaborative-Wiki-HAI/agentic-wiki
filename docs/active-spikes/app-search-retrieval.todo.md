---
attention: live
---
# Search and source selection tasks
Owner: Aslan’s agent in this implementation conversation. [Concept](app-search-retrieval.md). N2 approved 2026-11-17; keyword search only.

## Implemented keyword slice
- [x] Index saved text with revision-bound passages and correct refresh after edit/rename/move.
- [x] Connect wiki search and a reusable citation/evidence picker.
- [x] Default agent context to linked sources; allow explicit additions without automatic all-workspace retrieval.
- [x] Verify long transcripts, conflicting sources, manual citations, source revisions and return paths.

## Verification
- [x] Complete the native source-picker/live-proposal check, including explicit addition and removal of other saved evidence before a real request. The live workspace journey and regression suites pass; [QA evidence](../design/app-workspace-qa-2026-11-17.md) records the boundary.

## Later
Semantic retrieval, general chat answers and external-source discovery remain deferred.

## Ready for Human QA
Try your own saved page, import and source choices. Check relevance and wording support; automated checks do not establish research validity.
