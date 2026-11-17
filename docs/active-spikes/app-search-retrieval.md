---
attention: live
status: active
opened: "2026-11-17"
---
# S09 · Wiki search, source finding and grounded answers
Active spike: `app-search-retrieval`. Aslan approved the N2 keyword search/source selection slice on 2026-11-17. Semantic retrieval and general chat answers remain later scope. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M3; semantic retrieval is a separate gate within this spike. Owner: Aslan’s agent in this implementation conversation.

## Outcome
Find pages and source passages, distinguish their location and provenance, and ask a question whose answer links to inspectable evidence.

## Scope
Implement a rebuildable local full-text index, result snippets, filters/sort, source selection and revision-bound retrieval results. Provide a bounded retrieval service to S06. Evaluate semantic retrieval on a small judged corpus before selecting embeddings/runtime; preserve that feature on the roadmap rather than presenting lexical search as equivalent.

Out of scope: No hosted vector database, claim that subscription inference includes an embeddings endpoint, unrestricted vault indexing, or Workshop-only cues silently added to the released Prototype. No “Related Pages” placeholder resurrected on the planning article.

## Design basis
[Search results](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-100665&scaling=scale-down-width&hide-ui=1) · [Concept search example](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-105551&scaling=scale-down-width&hide-ui=1) · [Citation source search](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-97331&scaling=scale-down-width&hide-ui=1) · [Cited chat answer](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-94018&scaling=scale-down-width&hide-ui=1). Breadcrumbs/source-change exploration later tried in Workshop is not automatically canonical. Inspect released results before translating those refinements.

## Dependencies and interface ownership
Completion prerequisites: S01, S02, S04. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own SearchQuery/SearchResult and EvidenceBundle with object ID, revision, matching passage, score kind and indexing state. S02 supplies current revisions; S04 resolves evidence; S06 requests context. Invalidate/rebuild on changes and deletions. Retrieval scores rank candidates; they are not factual confidence or evidence sufficiency.

## Rough task sequence
1. Define a public-safe evaluation corpus with exact-match, paraphrase, obsolete-source and wrong-equipment queries; record expected sources.
2. Implement SQLite FTS5 and indexing/reindex operations; connect wiki search and citation pickers.
3. Integrate bounded evidence retrieval into answers/proposals; test abstention and wrong-source behavior.
4. Benchmark a separately chosen local embedding model against lexical baseline, including disk/RAM/latency and semantic-query usefulness, before enabling a semantic option.

## Acceptance criteria
- Search reflects saved edits and excludes deleted/inaccessible material; rebuilt indexes reproduce results.
- Selecting a result opens the correct object/passage and retains return context. Filtering/sorting work without changing document state.
- Generated answers distinguish supported claims from missing evidence and never fabricate a source ID.
- Concept/semantic search is labeled implemented only after a real retrieval path passes its evaluation; seeded results do not qualify.

## Automated and integration checks
Index tests cover edits, rename/move, deletion, stale index and revision links. Fixed query judgments compare retrieval variants. Agent tests include conflicting booking instructions and a manual for the wrong centrifuge rotor. Human assessment remains necessary for answer usefulness and source support.

## Human QA
Search using the user’s own paraphrases, inspect result context, and follow citations from an answer. Compare lexical and semantic results before deciding whether extra complexity earns its place.

## Risks, choices and recommendation
Full-text search is a useful first capability but cannot substantiate the presentation’s vector-search claim. Recommend lexical first and a gated local semantic experiment, keeping model selection/cost as a later decision. The initial proposal route can use explicit evidence and need not wait for either.

## Independent work and gates
Corpus/query drafting and index experiments can proceed once S02’s record shape is agreed. Live grounded answers require S06. Semantic retrieval does not block M1 or the recording milestone.

## N2 implementation boundary
Keyword search indexes only saved workspace records. Results identify object, revision and exact passage offsets. Agent requests default to sources already cited by or attached to the page, with an explicit source picker to add other workspace material. Long source blocks are split into bounded excerpts; the request preview shows exactly which excerpts will be sent. No ambient Drive/vault lookup or embedding service is enabled.

Continues from: docs/active-spikes/implementation-planning.md

## Implemented keyword slice
SQLite FTS5 indexes only current saved paragraphs, with source IDs, revisions and UTF-16 offsets. Title changes refresh the index; parent changes update result breadcrumbs. Drafts stay outside search. Results open their saved passage; reopening wiki search retains the last query. The same picker supplies manual citations and explicitly chosen agent evidence. Default evidence includes deduplicated pinned citations plus relevant excerpts from directly linked sources, bounded to six excerpts and 12 KB of source text. Explicit selection supports up to eight excerpts; the request validator retains its total payload limit. Search ranking is lexical relevance, not evidence sufficiency.

The broader filters/sort, semantic retrieval, judged corpus, general answers and external discovery in the roadmap remain unfinished; completing this slice does not close S09.
