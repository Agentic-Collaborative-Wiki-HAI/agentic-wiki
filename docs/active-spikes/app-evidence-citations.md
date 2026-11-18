---
attention: live
status: active
opened: "2026-11-17"
---
# S04 · Source evidence, citations and margin notes
Proposed spike: `app-evidence-citations`. Activated for the authorized M1 implementation batch on 2026-11-17; it does not amend standing policies. [Build map, evidence and decisions](../active-spikes/implementation-planning.md). Intended delivery: M1 minimum references; M2–M3 full authoring. Implementation owner: Aslan’s agent in the current build conversation.

## Outcome
Trace a claim or suggestion to an exact source passage, distinguish a citation from a normal link or human note, and return without losing context.

## Scope
Implement source artifacts and revision-bound references, inline numbering, margin alignment, collapsible endnotes, citation-only filtering, return links and evidence previews. Add whole-object, passage and audio-range citations; manual print/web/DOI metadata; ordinary links; authored margin notes; provenance for reused passages. Store typed derivation relationships rather than displaying primary/secondary/tertiary tiers.

Out of scope: No universal citation-manager replacement, inference of source role from derivation depth, mandatory DOI network lookup, automatic citation refresh to the newest source revision, or live transclusion that silently rewrites published prose.

## Design basis
[Citation, link and margin note](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-86050&scaling=scale-down-width&hide-ui=1) · [Transcript citation preview](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89759&scaling=scale-down-width&hide-ui=1) · [Source without a URL](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-105989&scaling=scale-down-width&hide-ui=1) · [Reused passage](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89592&scaling=scale-down-width&hide-ui=1). Primary recording and Audio note remain object types. Margin notes and endnotes represent the same records; summaries/transcripts retain generation/review provenance.

## Dependencies and interface ownership
Completion prerequisites: S02, S03. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own SourceRef (object/revision plus optional passage/time/page locator), citation occurrence IDs, source metadata and provenance edges. S03 supplies current anchor placement; S02 stores identity and pinned revisions. S05 consumes immutable evidence references; S08 supplies timecoded transcript passages; S09 resolves selectable sources.

## Rough task sequence
1. Implement revision-pinned discussion evidence and explicit article-citation authoring.
2. Add common rendering for inline references, margin notes and endnotes with actual scroll/return behavior.
3. Implement source selection and manual entry, then whole-transcript and audio-range variants.
4. Integrate transcript edits and source-change findings; retain old evidence snapshots and connect review proposals rather than silently advancing references.

## Acceptance criteria
- The inline marker, margin entry and endnote open the same evidence at the cited revision.
- A plain link, source citation and human margin note remain distinguishable and editable through their proper controls.
- Manual print citations save without a URL; missing network metadata does not prevent an explicit manual citation.
- Changing a source does not invalidate historical evidence or silently replace it. A proposed wording change does not automatically create or update an article citation.
- Notes follow their passages while scrolling; the citation-only switch visibly changes state and filtering.

## Automated and integration checks
Contract tests verify pinned references, deterministic numbering and no dangling source IDs. UI tests cover notes colliding in the rail, deleted anchors, long-page scroll, preview Close, keyboard focus, manual citation save and unchanged caller context.

## Human QA
Compare the page, transcript and history evidence views. Pressure-test whether attribution and source scope are understandable without evidence-tier labels.

## Risks, choices and recommendation
A valid source ID does not prove that its text supports a claim. Keep structural validation separate from human evidence judgment. Recommend snapshot-based reuse with provenance initially; automatic live reuse would require a separate product decision.

## Independent work and gates
Rendering can start from fixed S02 records. Manual source entry need not wait for semantic search or real transcription. Audio-range behavior waits for S07/S08 media contracts.


Continues from: docs/active-spikes/implementation-planning.md

## Current interface and authoring boundary
Citations and margin notes are explicit authoring actions. Both have inline identities and appear in document order in the reference rail and endnotes. The endnotes provide a Citations only filter. Notes store their title and body; citations retain their exact source revision. Accepting a proposed text revision does not create article citations from its supporting evidence. Existing explicit citations and notes survive bounded rewrites, including selections containing their inline atoms. Discussion evidence stays inspectable in the discussion and history. Media timestamp alignment and external/manual-print citations remain later capabilities.

See [interface QA](../design/app-interface-qa-2026-11-17.md) for checked cases and remaining limits.
