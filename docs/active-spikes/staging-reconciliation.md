---
attention: live
---
# Staging reconciliation
Opened 2026-11-09. Slug: `staging-reconciliation`. [Tasks and review status](staging-reconciliation.todo.md) · [Staging board map](../design/staging-board-map.json) · [QA record](../design/staging-qa-2026-11-09.md).

## Purpose and authority
Combine Aslan’s and Erwin’s working designs into one connected Staging prototype. Aslan reported their joint decisions while discussing the work with Erwin on 2026-11-09, then authorized implementation. Both people review Staging before approving promotion to Prototype.

Workshops retain their source work. Staging contains the reconciliation draft. At Aslan’s direction, the 25 misplaced Prototype copies have been returned to a separate area in Erwin’s Workshop, preserving their identities and the existing Workshop material. Prototype is empty pending joint acceptance. Showroom and shared Component Library definitions are unchanged; Staging-specific header, chat and endnote-filter components isolate the integration repairs.

Related work: [Wireframe Workshop](wireframe-workshop.md), [Workshop UI System](workshop-ui-system.md), [component audit](../design/component-audit.md), [agent settings #34](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/34), [Workshop QA #35](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/35), and [maintenance integration #37](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/37). The user separately authorized committing and pushing the six reconciliation records on 2026-11-09. This does not authorize promotion or external messages.

## Agreed basis applied in Staging
| Area | Joint decision and implementation |
| --- | --- |
| Shared tree and corpus | Getting started and Using the microscope remain distinct documentation pages. Microscope orientation recording is one child of Getting started, containing introductory and training excerpts. Search, tree links, breadcrumbs, and source links refer to that same recording. |
| Documentation type | Current pages use Documentation. Procedure is retained as an attributed future exploration in Archive. |
| Metadata and actions · S1/S2 | Aslan’s integrated metadata and placement: page Edit/History with the title, creation actions above the reader, transcript actions within the transcript, and appropriate omissions on history/settings. |
| References and chat · S3 | Aslan’s References/Comments margin and collapsed endnotes. Conversation citations use descriptive links. Chat is a separate full-height drawer with a bottom composer; opening a page does not generate a message. |
| Authored recording notes · S4 | Erwin’s contributed notes become attributed margin notes associated with recording timestamps. They are separate from transcript text. The example includes an off-record recollection at the recording’s end, explicitly attributed to its contributor. |
| Recording review | Whole-transcript editing and review, with timestamps in a bounded scrolling window and audio below. Review attribution links to history. The summary identifies itself as AI-generated from the transcript; no summary-review status is shown without a review interaction. Editing title/description and editing the transcript are separate actions. |
| Suggestions and history | Inline deletion/insertion, supporting citation, and the associated thread. Accept/Reject applies to an individual suggestion. Replying, deciding, and resolving remain separate actions. History combines the change, decision, and conversation. |
| Protected pages · S5 | Reviewer eligibility/assignment is independent of authorship and can be handed over. The bounded example uses lab-admin controls. Delegation never bypasses protected-page review. |
| Delegation | Settings retain task choices. An automatic repair on an unprotected booking source produces a notification linking to its exact history event; Undo is in history. |
| Source changes | Changed-source information belongs on the affected citation. It does not imply that the whole page is wrong or authorize automatic content replacement. |
| Ordinary chat | Wait for a human prompt. Scripted replies require Send. Maintenance notifications are a separate behavior. |
| Held work | F13 Agents overview and Needs your input remain for the next joint review, as explicitly requested. |
| Promotion | Aslan and Erwin review this draft before any promotion. |

The protected-page and delegation examples are interface models. They do not establish repository agent authority, a running permissions service, automatic learning after rejection, or a settled lab governance scheme.

## Integrated surfaces
[Open the Staging launcher](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1934-4118&scaling=scale-down-width&hide-ui=1).

The canvas contains 230 native editable top-level frames: 178 Aslan source copies and 52 reconciled states/previews. The launcher retains the eighteen existing activities and adds six entries into the shared tree:

| Entry | Main contribution | Start |
| --- | --- | --- |
| Find current microscope instructions | Erwin F2 search, changed-source review, shared documentation and citations | [Search](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-24863&scaling=scale-down-width&hide-ui=1) |
| Contribute a recording note | Erwin F5 summary, transcript and attributed notes, using Aslan’s recording/annotation structure | [Orientation recording](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-25765&scaling=scale-down-width&hide-ui=1) |
| Check maintenance findings | Erwin F9 dashboard, checks, dismissed findings and shared suggestion review | [Maintenance](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-26351&scaling=scale-down-width&hide-ui=1) |
| Transfer reviewer responsibility | Reviewer invitation and accept/decline under Settings; onboarding remains separately accessible | [Reviewer settings](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-26631&scaling=scale-down-width&hide-ui=1) |
| Catch a wrong-source suggestion | Erwin F11 wrong rotor model, source comparison, reply/rejection, acceptance and later undo | [Suggested change](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-27311&scaling=scale-down-width&hide-ui=1) |
| Set agent delegation | Erwin F12 choices reconciled with protected review, notifications and history | [Agent settings](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-27029&scaling=scale-down-width&hide-ui=1) |

Use the launcher to initialize a scripted activity. A direct screen link is useful for inspection but may not initialize every shared variable. The board map records the source-copy map, current addresses, named extensions, Archive preservation and held F13 work. Forward outcomes sit left to right; alternatives branch vertically. The cover’s Staging link opens the launcher, while both Workshop links remain.

## Divergence disposition
| Difference | Classification | Disposition |
| --- | --- | --- |
| Integrated metadata versus a generic metadata table | Agreed difference | S1 uses integrated metadata. |
| Title actions, creation actions and transcript controls | Agreed difference | S2 uses Aslan’s placement and separate scopes. |
| Notes in body, bottom cards or side areas | Agreed difference | S3/S4 use the margin/endnote pattern, with timestamp association for recording notes. |
| Passage checks versus whole-transcript review | Agreed difference | Whole-transcript review selected; summary and human notes retain separate provenance. |
| Creator ownership versus transferable reviewers | Agreed difference | S5 uses protected pages and independent reviewer assignments. |
| Getting started versus Using the microscope content | Agreed difference | S6 preserves distinct documentation pages. Their recording citations refer to one shared orientation recording. |
| Maintenance, reviewer transfer, onboarding and delegation | Distinct scenario contribution | Connected to the shared interface. Reviewer transfer lives under Settings; handover-writing states are preserved in Archive and omitted from the working routes. |
| Navigation-triggered greetings | Settled behavior | Chat waits for a human prompt. |
| Copied launcher controls lost navigation | Implementation drift | Restored the eighteen original ordered action lists with Staging destinations; all twenty-four launcher entry clicks passed. |
| Convenience links that changed unrelated state | Implementation drift | Remapped destinations to Staging; added separate reply-posted, resolved and artifact-editor states; history captures its caller. |
| Pending Resolve leading to rejection | Implementation drift | Resolution only closes the thread. The suggestion remains pending and its discussion is accessible in history. |
| Shortened conversation in outcome views | Implementation drift | The request and agent proposal remain before the human decision. |
| Source preview mistaken for a competing panel architecture | Assessment distinction | Erwin’s inspected source peek is a small popover. That does not require a unified chat/source/comment panel. Context-preserving previews coexist with the agreed rail and chat drawer. |
| F13 Agents overview / Needs your input | Not yet jointly reviewed | Excluded from this pass. |
| Fixture content and secondary controls | Review/verification remaining | See the bounded gaps below; successful links do not establish complete content equivalence or every caller combination. |

## Preservation
Original Workshop boards retain their identities. The six incoming Staging sections containing 23 Erwin copies were moved into [Erwin’s attributed Archive group](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1938-7238), preserving those section and board identities. The [Procedure exploration](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1938-7239) contains a copy of Workshop source `1425:4148`; its original remains.

The seven returned Prototype sections contain 25 screens, including the held F13 additions. They sit separately from Erwin’s original Workshop sections and retain their original node identities. The three unadopted handover-writing screens are preserved in the attributed [Staging Archive group](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=2043-77430). Shared Component Library masters were not edited. Showroom remains the research presentation of earlier Workshop proposals.

## Annotation entry
On Getting started, click the sentence beginning “Reserve the microscope” to open the simulated selection menu. Reading mode offers Add comment; Edit page followed by the same click offers link, citation, margin note and comment actions. The comment example continues through Reply → reply field → scripted @agent request → Reply → suggested revision. Exact playback coverage and limitations are in [the annotation-entry QA record](../design/staging-qa-2026-11-09.md#annotation-entry-repair-and-review-comment-resolution--10-nov-2026).

## Limits and review priorities
This is a reviewable integration draft, not a fully accepted or exhaustively tested prototype. The [QA record](../design/staging-qa-2026-11-09.md) distinguishes graph checks, screenshots and actual playback.

- The new documentation is a compact shared example corpus. Its exact source excerpts, citation-to-claim scope, omitted source details and reviewer-transfer wording need comparison with Erwin’s originals before promotion. Do not treat a simplified example as proof that every source-content detail survived.
- The original Aslan intake, note/link/citation editing and long-page routes remain accessible. Context preservation when starting those auxiliary flows from every newly added document, recording and chat caller is not fully reconciled or rehearsed. Visible reference kebabs and alternate editing paths need a targeted pass.
- The recording combines introductory and training excerpts from one 18:04 fixture; passage-focused annotation/editing states retain their narrower scope. Audio controls, text entry, file selection and agent replies are scripted; they do not prove real transcription, playback, search, permission enforcement or agent behavior.
- Whole-transcript review and artifact editing have distinct routes. A richer edited-transcript example and exact revision diff still need coverage for the training passages of the shared recording.
- History examples cover accepted/rejected decisions, pending resolved discussions, reviews and undo. Cross-event return combinations and the consistency of timestamps, original thread wording and alternative review branches need further QA.
- The global tree includes the combined objects, including Planning a microscope session; Handover is not a wiki page. Search/account/dashboard layouts, note alignment after scrolling, and every collapsed/expanded combination need joint visual review.
- Home’s Needs attention summary and the Maintenance queue still overlap. A short Home summary linking to a single detailed Maintenance queue is proposed for discussion, not implemented.
- Neither graph verification nor this implementation grants human acceptance. F13 and Prototype promotion remain separate decisions.

