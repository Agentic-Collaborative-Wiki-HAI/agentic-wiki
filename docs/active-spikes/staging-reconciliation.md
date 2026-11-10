---
attention: live
---
# Staging reconciliation
Opened 2026-11-09. Slug: `staging-reconciliation`. [Tasks and review status](staging-reconciliation.todo.md) · [Staging board map](../design/staging-board-map.json) · [QA record](../design/staging-qa-2026-11-09.md).

## Purpose and authority
Combine Aslan’s and Erwin’s working designs into one connected Staging prototype. Aslan reported their joint decisions while discussing the work with Erwin on 2026-11-09, then authorized implementation. Both people review Staging before approving promotion to Prototype.

Workshops retain their source work. Staging contains the reconciliation draft. Prototype’s existing Erwin copies are misplaced material, not accepted work. This pass does not modify Prototype, either Workshop, Showroom, or shared Component Library definitions.

Related work: [Wireframe Workshop](wireframe-workshop.md), [Workshop UI System](workshop-ui-system.md), [component audit](../design/component-audit.md), [agent settings #34](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/34), [Workshop QA #35](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/35), and [maintenance integration #37](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/37). The user separately authorized committing and pushing the six reconciliation records on 2026-11-09. This does not authorize promotion or external messages.

## Agreed basis applied in Staging
| Area | Joint decision and implementation |
| --- | --- |
| Shared tree and corpus | Getting started and Using the microscope remain distinct documentation pages. The orientation recording and training recording are distinct children of Getting started. Search, tree links, breadcrumbs, and source links expose those objects. |
| Documentation type | Current pages use Documentation. Procedure is retained as an attributed future exploration in Archive. |
| Metadata and actions · S1/S2 | Aslan’s integrated metadata and placement: page Edit/History with the title, creation actions above the reader, transcript actions within the transcript, and appropriate omissions on history/settings. |
| References and chat · S3 | Aslan’s References/Comments margin and collapsed endnotes. Conversation citations use descriptive links. Chat is a separate full-height drawer with a bottom composer; opening a page does not generate a message. |
| Authored recording notes · S4 | Erwin’s contributed notes become attributed margin notes associated with recording timestamps. They are separate from transcript text. The example includes an off-record recollection at the recording’s end, explicitly attributed to its contributor. |
| Recording review | Whole-transcript editing and review, with timestamps and audio below. Review attribution links to history. The summary identifies itself as AI-generated and unreviewed separately from the transcript. Editing title/description and editing the transcript are separate actions. |
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

The canvas contains 233 native editable top-level frames: 178 Aslan source copies and 55 reconciled states/previews. The launcher retains the eighteen existing activities and adds six entries into the shared tree:

| Entry | Main contribution | Start |
| --- | --- | --- |
| Find current microscope instructions | Erwin F2 search, changed-source review, shared documentation and citations | [Search](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-24863&scaling=scale-down-width&hide-ui=1) |
| Contribute a recording note | Erwin F5 summary, transcript and attributed notes, using Aslan’s recording/annotation structure | [Training recording](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-25765&scaling=scale-down-width&hide-ui=1) |
| Check maintenance findings | Erwin F9 dashboard, checks, dismissed findings and shared suggestion review | [Maintenance](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-26351&scaling=scale-down-width&hide-ui=1) |
| Prepare a handover | Erwin F10 draft/publication, reviewer invitation, accept/decline and onboarding | [Handover](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=1939-26631&scaling=scale-down-width&hide-ui=1) |
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
| Getting started versus Using the microscope content | Agreed difference | S6 preserves distinct documents and distinct recording objects. |
| Maintenance, handover, onboarding and delegation | Distinct scenario contribution | Added to the shared shell/tree rather than replacing existing recording/editing/intake routes. |
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

The latest inspected Prototype contains 25 misplaced screen copies, including the separately held F13 additions. It is unchanged by this reconciliation. Shared components are reused through instances, properties and slots; their masters were not edited. Showroom remains the research presentation of earlier Workshop proposals.

## Limits and review priorities
This is a reviewable integration draft, not a fully accepted or exhaustively tested prototype. The [QA record](../design/staging-qa-2026-11-09.md) distinguishes graph checks, screenshots and actual playback.

- The new documentation is a compact shared example corpus. Its exact source excerpts, citation-to-claim scope, omitted source details and handover wording need comparison with Erwin’s originals before promotion. Do not treat a simplified example as proof that every source-content detail survived.
- The original Aslan intake, note/link/citation editing and long-page routes remain accessible. Context preservation when starting those auxiliary flows from every newly added document, recording and chat caller is not fully reconciled or rehearsed. Visible reference kebabs and alternate editing paths need a targeted pass.
- The recording uses a transcript excerpt from a longer recording. Audio controls, text entry, file selection and agent replies are scripted; they do not prove real transcription, playback, search, permission enforcement or agent behavior.
- Whole-transcript review and artifact editing have distinct routes. A richer edited-transcript example and exact revision diff still need coverage for the new training object.
- History examples cover accepted/rejected decisions, pending resolved discussions, reviews and undo. Cross-event return combinations and the consistency of timestamps, original thread wording and alternative review branches need further QA.
- The global tree includes the combined objects. Search/account/dashboard layouts, note alignment after scrolling, and every collapsed/expanded combination need joint visual review.
- Neither graph verification nor this implementation grants human acceptance. F13 and Prototype promotion remain separate decisions.

