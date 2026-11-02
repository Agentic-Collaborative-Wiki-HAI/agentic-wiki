---
attention: live
---
# Research-grounded Showroom walkthrough
Opened 2026-11-02. Slug: `showroom-research-tour`. Owner: Aslan's ongoing Codex wireframe thread. Related outcome: [Showroom issue #36](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/36).

## Purpose
Present a short, linear account of how an agent could help maintain lab documentation: capture a relevant update, request a bounded comparison, inspect supporting evidence, review a proposed change, and retain the decision trail. A prior demonstration of these interactions does not establish that the audience understood them. The Showroom deliberately includes them and makes their purpose and research basis explicit.

The microscope scenario is illustrative. Research findings motivate the maintenance need and interaction choices; they do not validate this particular interface. The presentation distinguishes a finding, a design response, and a boundary or question to test. Narrowing the story does not settle the product's overall scope.

## Presentation and ownership
The [seven-frame tour](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1646-2046&scaling=contain&hide-ui=1) lives on Showroom. Each 1920×1080 frame carries a Workshop-proposal label and a source-screen link. Frames run left to right, with Previous/Next and Restart navigation. The shared Cover links to its start.

Screens are editable presentation snapshots of Aslan's Workshop, not promoted Prototype screens. Their source geometry, wording and rich-text diff styling are preserved in cropped copies. The presentation header and research panel are local shared components; navigation uses the existing compact-button component. Snapshot interactions are removed so clicking through a presentation does not accidentally change the example's state. Source-frame links lead to the working design for exploration.

Erwin's pages and shared Component Library definitions remain outside the write scope. The final frame acknowledges the separate maintenance exploration and future integration; it does not claim that work has been merged or accepted. Team reconciliation still belongs in Staging before Prototype promotion.

## Presentation treatment
The presentation uses a cool neutral backdrop, green headings and a pale green notes column to separate narration from the white wireframe examples. Research quotations appear in inset blocks with a left rule, quotation mark, italic text and attribution. Paraphrased findings are labelled as interview summaries. Two local shared annotation components carry those treatments; they do not alter the product's Component Library.

Research links are blue, underlined and paired with a link icon. The icon and attribution row open the same original source. Narration uses direct, everyday language and separates what participants said, why a feature is proposed, and what still needs testing. Quoted words and source destinations remain unchanged.

## Narrative map
| Frame | Purpose | Workshop source |
| --- | --- | --- |
| [S01](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-2046) | Establish the maintenance gap | `509:299` |
| [S02](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-2228) | Capture a recording and inspect its transcript/audio | Dialog on `509:482`; reviewed recording `509:747` |
| [S03](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-2570) | Make a bounded agent request in an anchored comment | `511:454` |
| [S04](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-2893) | Inspect the cited source and distinguish support from inference | Preview `781:27591`, opened from `511:513` |
| [S05](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-2959) | Review an inline proposal and choose a response | `511:513` |
| [S06](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-3375) | Inspect the accepted change and associated conversation | `994:20991` |
| [S07](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1646-3647) | Identify evaluation questions and the next integration work | Citation context from `512:299` |

## Research handling
The user authorized research annotations in the existing team design file. The annotations use participant codes, short checked excerpts or explicit paraphrases, and links to the existing restricted research sources. Original transcripts, participant identities, private meeting commentary and email text are not copied into this public repository. The presentation's source links preserve existing access restrictions; no sharing permissions are changed.

## Rehearsal and limits
Rehearse the Showroom separately from the native Workshop route. Presentation Next/Previous links prove linear navigation; they do not prove the underlying product interactions. Record actual playback outcomes and any remaining limitations in the [task record](showroom-research-tour.todo.md). The prototype simulates file selection, transcription, audio states, text entry and agent responses; it does not perform those operations.

Broader Workshop work is deferred in the existing [Workshop task record](wireframe-workshop.todo.md#remaining-work-after-the-showroom--2026-11-02), not silently completed or erased by this presentation.
