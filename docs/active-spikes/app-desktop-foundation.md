---
attention: live
status: active
opened: "2026-11-17"
---
# S01 · Desktop foundation and shared interface
Active spike: `app-desktop-foundation`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. Later milestone scope remains planned, not implemented. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0–M1; extend through M4. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Open a real desktop application with the familiar wiki shell, navigate between objects, and return without losing unrelated reading or editing context.

## Scope
Create app/ with Tauri, Vue, TypeScript and Vite; establish a small shared component layer and extracted design values. Implement the header, wiki tree, page contents, centered reading area, sticky References/Comments rail, global chat drawer and settings navigation. The first milestone needs only the routes it exercises; extend the same shell instead of cloning Figma states into separate app screens.

Out of scope: No new Figma styling, framework migration, comprehensive token-sync pipeline, HTTP server, mobile build, or simultaneous editing. No automatic reopening of the closed course token-integration issue.

## Design basis
[Getting started](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-85970&scaling=scale-down-width&hide-ui=1) · [Long planning page](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-99503&scaling=scale-down-width&hide-ui=1) · [Settings](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-91324&scaling=scale-down-width&hide-ui=1). The tree icon controls the wiki hierarchy; the list icon controls this page’s contents. Collapsing either preserves centered, width-limited text. Header and sticky tabs have opaque backgrounds. Search is centered; links look like links.

## Dependencies and interface ownership
Completion prerequisites: No completed feature prerequisite; contracts can be drafted against fixtures. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own the desktop invocation adapter, navigation context (object ID, optional revision/passage/thread, return location), window lifecycle and shared UI components. S02 supplies objects; S03–S11 supply feature views. Export typed command/event contracts with S02 rather than independently hand-maintaining two definitions. Keep domain rules out of Vue and Tauri window handlers.

## Rough task sequence
1. Refresh the implementation baseline from published records without resetting dirty work; pin the chosen toolchain and record startup instructions.
2. Build the window and reusable shell against small synthetic records; verify scroll, focus, modal dismissal and chat close/return.
3. Connect S02 persistence and the first S05 route; add a debug/test-only bridge for S12 desktop verification.
4. Extend the shared navigation to equipment overviews, recording, search and settings as their owners deliver them.

## Acceptance criteria
- The app starts on the target Mac and navigates from Getting started to its source and back with the expected context.
- Tree ordering and recording identity are consistent across pages; no in-progress chat example appears merely because Chat was opened.
- References/Comments remain switchable while scrolling, and content does not bleed behind fixed controls. Collapsing navigation does not make body text unbounded.
- All visible controls in a completed route either work or clearly explain their unavailable state; no decorative dead hotspots.

## Automated and integration checks
Component tests cover route-context preservation and focus return. Desktop checks cover real window resizing, WKWebView scrolling, keyboard selection, file dialogs when available, and reopening. Cross-check layout against selected Prototype screens at both the design width and the actual laptop window.

## Human QA
Aslan reviews shell fidelity, text width, drawer behavior, keyboard flow and the two independent navigation toggles. A screenshot check cannot substitute for a scroll/click walkthrough.

## Risks, choices and recommendation
Figma frames use wide fixed examples; the app needs resizing behavior. Recommend keeping the established layout while adapting column widths, then bringing any material interaction change for review. Routine CSS/component choices are reversible; do not turn them into policy decisions.

## Independent work and gates
Shell work can start against fixtures while S02 and S06 investigate independently. Persistent navigation depends on S02. A working shell alone is not completion of the first product milestone.


## Current activation
The first M0 foundation/check slice is implemented and documented in the companion task record. The native shell, persistence and bounded editor contract have passing evidence; Codex generation remains disabled because isolation was not proven. M1 and later acceptance criteria remain future scope. D6–D8 are deferred.

Continues from: docs/active-spikes/implementation-planning.md
