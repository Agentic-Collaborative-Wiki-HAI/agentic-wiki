# Staging QA · 9 Nov 2026
[Joint reconciliation](../active-spikes/staging-reconciliation.md) · [Remaining tasks](../active-spikes/staging-reconciliation.todo.md) · [Board map](staging-board-map.json).

## Scope
Agent verification of the joint Staging draft in Figma file `fm5zX32mbG13hahShVgJwd`, page `1469:31745`. Human acceptance and Prototype promotion remain outstanding. This report separates native playback from structural and visual inspection.

## Structural results
- 233 native editable top-level frames: 178 copied Aslan boards and 55 added reconciliation states/previews.
- Recursive reaction scan, including hidden descendants and conditional actions: 12,779 reactions; no missing destinations, no non-variant NODE destinations outside Staging, and no SCROLL_TO targeting another frame.
- The full scan ran after the training-tree selection, title sizing, editor caller and scrolling repairs. It found 233 frames and 12,779 reactions with no missing/outside-Staging NODE destinations or foreign SCROLL_TO targets. A later launcher repair restored omitted navigation actions on eighteen existing controls; its persisted action lists and all twenty-four entry destinations were checked separately.
- Shared library definitions, both Workshops, Showroom and Prototype were not mutated. Incoming Staging sections were preserved in Archive with their IDs.
- Representative reader, training-note, history, settings and artifact-editor trees contain native TEXT/INSTANCE/FRAME/SLOT/VECTOR layers and no screenshot-filled UI. Their text uses Roboto. This is a five-screen composition check, not a file-wide raster/font audit.
- The Cover Staging hyperlink targets `1934:4118`. Its owner links and other destinations were preserved.

A valid destination does not prove the correct caller, visible hit target, state preservation, semantic content or complete route coverage.

## Native playback
Playback ran in a dedicated Chrome tab with the user-authorized debugging connection. The viewport was 1728 × 866 CSS pixels at device scale 2; screenshots are 3456 × 1732. The Figma connector was used for authoring and inspection, not to substitute a reaction listing for playback.

| Path | Result |
| --- | --- |
| Launcher `1934:4118` → Search `1939:24863` → Using the microscope `1939:24893` | Initial rehearsal covered this new entry only; it did not establish preservation of the eighteen copied entries. |
| All twenty-four launcher entry controls → their expected Staging screens | After the reported failure and repair below, clicked every entry in native playback and checked its resulting frame ID. Browser Back returned to the launcher between cases. The board map records each control, destination and result. |
| Using the microscope → Chat → Send | Full-height drawer and bottom composer appeared; Send produced the scripted answer while the underlying page stayed unchanged. Human bubble styling and citation-preview wiring received a later local update, not a second complete chat rehearsal. |
| Suggestion `1939:25105` → Accept → `1939:25214` | Published example switches the booking requirement to two working days. The outcome retains request, agent explanation and human decision. |
| Accepted `1939:25214` → Resolve → `1956:32706` | Thread disappears from Comments; resolved-thread history link appears. Resolve is visible below the sticky tabs. |
| Resolved `1956:32706` → history `1939:25488` → Back | Returned to exactly `1956:32706`. Before/after PNG files were byte-identical with `cmp`. History shows the inline diff and associated discussion together. |
| Wrong-source suggestion `1939:27311` → citation preview `1939:27420` → Close | Preview exposes the rotor-model mismatch without replacing the page. Close returned to the same frame; before/after PNGs were byte-identical. |
| Training `1939:25765` → Mark transcript as reviewed → `1939:25938` | The whole-transcript action disappears and Reviewed by Mei Chen appears. Summary provenance remains separately unreviewed. |
| Reviewed training → review attribution → history `1939:26281` → Back | Returned to reviewed state `1939:25938`. |
| Reviewed training → Edit transcript `1939:25848` → scroll → Cancel | Scrolling reaches the lower transcript and audio. The shared editing bar, wiki tree, page contents, nav and rail tabs remain available. Cancel returns to reviewed state `1939:25938`; before/after PNGs were byte-identical with `cmp`. |

## Launcher follow-up · 9 Nov 2026
The user reported that multiple launcher hotspots did nothing. All eighteen copied controls retained variable-reset actions but lacked navigation; the six newly composed entries retained their navigation. The original Workshop launcher still had the expected ordered action lists. This differs from the earlier conflicting Chat toggle defect. The exact migration operation that removed navigation has not been established.

Restored the eighteen controls from the original ordered actions, translating their destinations to Staging while preserving initialization and reset behavior. Persisted readback shows one click reaction and one navigation destination per entry. All twenty-four entry clicks passed in native playback. The Chat destination visibly retains its full-height drawer and bottom composer. No source Workshop, shared component or Prototype nodes changed.

The earlier destination scan could detect invalid links but not omitted navigation, and the new-entry playback sample did not cover copied controls. This follow-up establishes launcher activation; it does not replace downstream route QA.

## Defects found and repaired during this pass
- Copied hidden reactions and scroll anchors escaped to Workshop/old history states; remapped to Staging.
- The expanded wiki tree was clipped inside its inherited slot; its inner layout now hugs the entries.
- Populated chat Send used the disabled variant; it now uses the default compact action.
- Resolving a pending discussion led to a rejected event. Resolution now leaves the suggestion pending and exposes its archived discussion separately.
- Replying with a reason could imply rejection. A separate reply-posted pending state preserves independent decisions.
- Edit recording artifact led to transcript editing. A separate metadata editor uses the shared bar and title/description fields.
- New history return controls lost their callers. Entry actions now capture the originating state.
- The metadata editor’s copied title text had a one-pixel height. It now hugs its text and labels the editing scope correctly.
- Training views inherited orientation’s selected tree row. Only training remains selected.
- Cancelling a training editor could discard the caller’s reviewed/annotated state. Both editor modes now capture their callers.
- Several added recording views had vertical overflow disabled and clipped the lower transcript/audio. The added full-page states now allow vertical scrolling and their content ancestors hug rather than clip their contents. The training editor’s scroll-to-audio and sticky controls were checked in playback.

## Coverage limits
- All 24 launcher entry controls were exercised after repair. This is not an exhaustive traversal of 233 destination flows or every variable combination.
- Auxiliary intake, annotation editing, source-card menus, alternate caller contexts and the larger changed-transcript example remain in the spike’s task list.
- Pending-resolution histories, reply-posted outcomes, all maintenance/handover/delegation branches and every new history return combination need additional native tests.
- Direct frame URLs may bypass launcher initialization. History defaults and state reset across independent review sessions need checking.
- File choice, typing, playback and agent replies are scripted examples. No real agent, transcription service or permissions engine was tested.
- Source-content equivalence, citation scope, chronology and branch semantics require joint review. Structural and visual checks cannot settle those design/content questions.
- F13 Agents overview and Needs your input are excluded, not silently accepted or rejected.

## Evidence retention
The named screenshots were produced in the task’s temporary directory. They are not committed into the public repository. The durable record is the exact path/IDs and result descriptions above, plus the Staging map. Original source boards remain available for comparison.

