# Agentic Wiki wireframe conventions
The [file manifest](../../design-files.json) maps page roles and ownership. The shared collection's [numbering policy](../../wireframe-workflows/docs/agent-policies/ARRANGE-BOARDS-board-numbering-and-layout.md) and [Scratchpad/Archive policy](../../wireframe-workflows/docs/agent-policies/PRESERVE-EXPLORATION-scratchpad-and-archive.md) govern Aslan's migrated work. Erwin's pages retain their existing conventions until he chooses to adopt them.

## File orientation and page order
The shared [Cover](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1507-2) links to the current Workshop routes and repository setup. Staging, Prototype and Showroom are unpopulated; no settled team prototype is presented there. Keep each person’s Workshop immediately followed by their Scratchpad, preserving the established owner order. Component Library remains shared and Archive stays last.

## Scope and addresses
Navigation addresses follow the existing wiki hierarchy. They locate the screen, not its place in a route walkthrough. Screen states, editing modes and history views retain the owning object's address and use descriptive names. This is the current prototype's address map, not a proposed permissions or routing implementation.

| Address | Screen or context |
| --- | --- |
| `1` | The Lab home |
| `1.1` | Equipment |
| `1.1.1` | Microscope |
| `1.1.1.1` | Getting started with the microscope |
| `1.1.1.1.1` | Microscope orientation recording, transcript and their history views |
| `1.1.1.1.2` | Microscope orientation audio note and its transcript/history |
| `1.1.1.1.3` | Microscope care, created from Getting started |
| `1.1.1.2` | Planning a microscope session |
| `2` | Profile |
| `3` | Personal settings |
| `P0` | Workshop launcher, outside the product hierarchy |
| `O1`–`O10` | Standalone overlay/partial demonstrations, identified individually in the board lookup |

The `O` family is local to this project; it is not a universal convention. Full-screen host states showing a modal keep the host's address. Viewport is an optional qualifier, not a hierarchy level. Intermediate hierarchy entries need not have a separate frame.

The [board lookup](figma-board-map.json) maps former route names to current unique names, optional short codes and unchanged Figma node links. Native names carry the address, descriptive screen and state. Old codes remain aliases in the lookup for existing discussion and spike records. Historical records retain their original identifiers. Do not use a global rename across another owner's pages or shared components.

## Scratchpad and Archive
Aslan's Scratchpad retains Parked navigation alternatives and future ideas. The shared Archive is last and contains historical exploration groups labelled with their original owner. Product version-history screens remain in Workshop because they are working prototype routes, not obsolete design work.

Archived exploration preserves legacy names and node identities. It is browsable historical evidence, not a working current prototype: some old interactions already target other pages or shared source components. The current Workshop has no interaction destinations in the archived groups. Current decisions, remaining work and checks belong in the repository rather than old canvas review guides.

## Working with the collections
Use the top-level `project-workflows/` for this project's planning and records, and the pinned `wireframe-workflows/` for wireframe procedures. Its nested Project Workflows dependency governs contributions to that collection, not this project's records. Use the actual selected checkout when following relative skill links; keep full folders and supporting policies together.

Figma access is provided by the user's configured connector. Global skill links do not install or authenticate it. Creating shared components, changing other owners' work or publishing commits requires the corresponding authorization.

## Workshop launcher
[Open Aslan’s Workshop launcher](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=513-299&scaling=scale-down-width&hide-ui=1). The [board lookup](figma-board-map.json) is also the entry-point record; the file manifest references it through `local.routes`. Keep its action nodes, destination boards, prototype URLs, starting conditions and scenario references synchronized with the launcher.

Workshop contains individual screen/state boards directly on the connected canvas. Journeys progress from left to right; alternate decisions and states branch vertically beside their callers. Recording, transcript editing, citation authoring and review history sit near the relevant entry and outcome screens. Short forward transitions have labelled connectors; long jumps and returns use linked destination names below the originating board. Dashed connectors identify a state change within the same displayed screen, not navigation to a separate product page. These annotations explain selected relationships rather than reproducing every header, tab or return interaction. Scenario and development-batch sections do not partition Workshop or add a level to navigation addresses. Scratchpad and Archive retain their separate exploration/history grouping conventions.

The launcher exposes eighteen activities, including the original F1 recording, F4 review and F8 comment scenarios from the [user-flow record](https://www.figma.com/board/zWKFoZeOtzEHpC25y2mg98/User-Flow-and-Scenarios?node-id=88-1085). Other activities have descriptive names without invented scenario identifiers. Entries can share a starting board; they do not own isolated copies of the product. The file Cover continues to link to this launcher and selected starts.

Restart the prototype before choosing an entry to reset shared state. Launcher actions reset scroll and interactive component state, close or open chat as appropriate, and navigate to the named screen. Chat entries also initialize the conversation/composer variables; direct frame URLs do not execute launcher actions, so open Chat when entering that way. File selection, typing, audio and agent replies are scripted examples. The launcher states these limits and links to the detailed records rather than repeating the project task list.

## Convention application verification · 2026-11-01
- Structural inspection: removed twenty-six scenario/development sections, including one empty legacy section. All 178 screen IDs, dimensions and absolute positions were preserved. Descendant identity, geometry, visibility, text and reaction fingerprints matched immediately after reparenting; the launcher was then deliberately updated. The connector does not offer named-version creation; a pre-change geometry/signature snapshot was retained for this operation.
- Entry inspection: all fifteen distinct destinations used by the eighteen launcher actions resolve. Replaced copied flow-start labels with the launcher plus those fifteen distinct starts. Added missing chat-citation/endnotes actions and navigation after chat intake initialization.
- Visual inspection: rendered the launcher, normalized secondary entry controls and aligned the three primary cards. Product screen contents were not redesigned.
- Ownership: Erwin’s Workshop and Scratchpad matched their before/after identity, geometry, visibility, text and reaction fingerprints. No write targeted his pages, shared component definitions, Scratchpad, Archive or Cover.
- Remaining verification: this pass does not establish native playback, shared-state reset behavior, every route’s reachability, native-comment attachment, or the accuracy of every historical canvas connector. Continue the existing [Workshop QA](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/35) with the consistency audit and bug-fix skills. Structural inspection, visual inspection, prototype playback and human acceptance remain separate evidence.

## Canvas layout and native playback · 2026-11-01
- Canvas: rearranged all 178 Workshop boards into left-to-right progression with vertical branches. Replaced 60 obsolete flow annotations with 131 routed connectors, 126 action labels and 42 linked cross-references. No board, annotation-label or label-to-board overlaps remain in the geometric check; inspected the full rendered canvas after correcting label collisions.
- Preservation: screen identity, dimensions, descendant text, relative geometry, visibility and reaction fingerprints matched the baseline for all 178 boards after the layout pass. Erwin’s Workshop and Scratchpad matched their before/after fingerprints. The subsequent audio-note repair changes only launcher action `1543:24268`, its context label `1543:24271`, and Aslan Workshop’s flow-start list. Shared components, Erwin’s pages, Cover, Scratchpad and Archive were not edited.
- Entry playback: clicked all eighteen launcher actions in Figma’s native prototype player and inspected their resulting screenshots. Rechecked the repaired audio-note entry through its complete upload-to-dialog path. Entry arrival does not establish completion of every route.
- Audio-note context: **Add an audio note** starts on Getting started (`509:299`), with the instruction **Add artifact → choose an audio file → choose Audio note.** The modal has an originating page; Close and Cancel retain their reusable overlay actions. Verified both returns with transcription enabled and Close with transcription disabled. The overlay itself is not a standalone flow start.
- Other sampled behavior: artifact picker opening and cancellation back through the upload modal to the guide; page-editor cancellation; citation cancellation retaining the editing selection; References/Comments switching while preserving the pending inline suggestion and restoring its thread; chat close/reopen and a scripted follow-up; endnote expansion and citation-only filtering.
- Transcript playback: `950:15620` → Cancel → `509:589`; reopen → scripted correction `950:16426` → Save → `950:17232`. The changed wording is visible after Save. Play/pause state and seeking to 00:48 work while editing, with audio below the transcript. Revised attribution opens `684:6928`; its return inconsistency is recorded below.
- Review playback: F4 `510:299` → Accept → `510:709` updates the guide; Reject → `510:574` retains its original wording. Their Page history actions open `994:20991` and `991:22802` respectively, and Back returns to the corresponding outcome. Successful navigation does not establish accurate history content; see the accepted-diff finding below.
- F8 playback: `511:299` → text selection `707:7431` → Add comment `511:346` → Reply `511:400` → @agent request `511:454` → agent suggestion `511:513`. The suggestion’s citation opens the transcript preview and Close preserves the pending discussion. Accept reaches the accepted guide `510:709`. A separate run verified Resolve thread removes the pending discussion from Comments and exposes the resolved-thread history link; that history path has the defects below.
- Long-page playback: reader `1079:24424` and editor `1162:154668` scroll while retaining nav, wiki contents, page contents and References/Comments tabs. The editor’s formatting, tracking and compact actions remain accessible after scrolling. Clicking **Arrange access** scrolls to that heading and highlights its item. Cancel from the scrolled editor returns to the reader. Automatic heading tracking is not implemented, as noted below.
- Limits: these are named route samples, not exhaustive coverage of all 178 boards or every state combination. Earlier hidden-tab navigation and wheel attempts were inconclusive; follow-up playback used a visible, focused native player at 1920×1000 and completed the checks above. Native Figma comments were not reopened. File selection, typing, audio and agent responses remain scripted. Human acceptance remains pending.

### Walkthrough findings and disposition
The bounded repairs below are complete in Aslan’s Workshop and remain part of [Workshop QA](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/35). Human acceptance is separate from the recorded agent checks.

| Finding | Current behavior and evidence | Status |
| --- | --- | --- |
| Transcript-history return context | Revised attribution captures its recording caller. Back to artifact returns to that exact state. Native round trips from `950:17232` and `684:6009` restore screenshots pixel for pixel; six attribution controls cover five recording states structurally. | Repaired · 2026-11-02 |
| Accepted-history diff wording | `994:20991` shows the exact old/new instruction accepted into `510:709`, with the associated conversation. Text equality passed and the rendered diff was inspected through Page history; Back restores the accepted guide exactly. | Repaired · 2026-11-02 |
| Resolved history visibility and return | Alternate archived conversations occupy the same display position instead of falling below a clipped container. F8 `511:513` → Resolve → resolved-thread history → Back restores the same resolved page. The neighboring transcript route `950:21262` → `1109:140330` → Back also preserves its resolved state. Both return screenshots match their originating states. | Repaired · 2026-11-02 |
| Resolve obscured by sticky tabs | Eighteen affected Comments contexts reserve space below the opaque sticky strip. Resolve is visible and clickable on pending, accepted and rejected guide states and the transcript comment state. | Repaired · 2026-11-02 |
| Long-page active heading | Clicking a heading scrolls to it and updates its selected state. Manual scrolling does not update that selection. The available native prototype trigger API has no scroll-position event; no simulated substitute was added. | Prototype fidelity limitation |

## Chat toggle repair · 2026-11-02
- Report and reproduction: native Figma comment #5 identifies the transcript-checked recording `509:747`. Clicking Chat in the native player did not open the drawer.
- Cause: local navigation instances contained duplicate click reactions that toggled the same Boolean twice. Drawer-header instances also carried conflicting Close/toggle reactions. The shared compact-button source had no reactions; the repair belongs to the local prototype wiring.
- Repair: updated 310 instance controls across the 155 Aslan Workshop screens with active Chat drawers. Navigation sets the drawer’s visibility Boolean to true; its header control sets that same Boolean to false. The two existing global/route-local bindings are retained, as are non-click interactions. Profile and Settings have intentionally unavailable drawers and were excluded. No shared component definition or Erwin-owned node was edited.
- Structural evidence: every active Chat host has exactly one matching Open and one matching Close action; the 310-control check reports no mismatches.
- Native playback at 1920×1000: checked `509:747`, pending review `510:299`, dedicated chat `939:13562`, and long reader `1079:24424` after scrolling. Open shows the full-height drawer and bottom composer; Close restores the underlying page. The review’s Comments tab/thread and the long reader’s scroll position match their before-opening screenshots pixel for pixel. Reopening was also checked on the reported recording and dedicated chat screen.
- Limits: native playback samples four screens; all other active hosts have structural coverage. Conversation content, scripted message behavior and unrelated history/Resolve findings were not changed. The native review comment was not replied to or resolved. Human confirmation is pending.

## History and Resolve repairs · 2026-11-02
- Scope: Aslan’s Workshop only. Local instance padding, history content, visibility-state layout and prototype actions were edited. Erwin’s pages, shared component definitions, screen identities, launcher entries and canvas arrangement were not changed.
- Resolve layout: corrected eighteen Comments-context instances with 56 px of top clearance, leaving the existing opaque sticky tabs intact. The affected guide controls begin at y=178 below tabs ending at y=149; transcript controls begin at y=138 below tabs ending at y=109. Native clicks on `510:299`, `510:709`, `510:574` and `950:21262` remove the thread and reveal its history link.
- Caller state: six transcript-history attribution controls across five recording states set a dedicated Workshop origin variable. Twenty-seven resolved-history entry controls set the existing page-origin variable. Their return controls use separate supported conditional actions; a multi-branch conditional was truncated by the connector during an intermediate attempt, so persisted readback and native playback were required. Final readback confirms five transcript destinations and twenty-eight page destinations, including the existing direct-entry default.
- Archived conversations: repaired thirty-one page-thread states and two transcript-thread states inside their existing history-layout slots. Each alternative shares the same top position and retains its own selection binding. The containing detail region accommodates the longest state; source components are unchanged. Archived copies omit live Edit suggestion/Accept/Reject controls while retaining their discussion text and evidence links.
- Diff: the accepted insertion in `994:20991` equals the published instruction in `510:709`: “Use the booking calendar linked from the equipment page. Contact the equipment lead for hands-on training before your first session.” The removed spreadsheet instruction and surrounding context remain visible.
- Verification: native Figma playback at 1920×1000 covered both transcript-history callers, pending/accepted/rejected Resolve actions, F8 resolved history and return, accepted revision content and return, and the neighboring resolved-transcript history. The named return screenshots match their origins pixel for pixel. Structural checks cover all eighteen padding fixes and all thirty-three history-entry controls. This is bounded route coverage, not exhaustive replay of all Workshop state combinations.
- Remaining limit: scroll-driven active-heading selection is not implemented. The existing click-to-heading behavior and sticky controls remain. Human review is pending; no public issue update was performed.
