# Staging QA · 9 Nov 2026
[Joint reconciliation](../active-spikes/staging-reconciliation.md) · [Remaining tasks](../active-spikes/staging-reconciliation.todo.md) · [Board map](staging-board-map.json).

## Scope
Agent verification of the joint Staging draft in Figma file `fm5zX32mbG13hahShVgJwd`, page `1469:31745`. Human acceptance and Prototype promotion remain outstanding. This report separates native playback from structural and visual inspection.

## Structural results
- 233 native editable top-level frames: 178 copied Aslan boards and 55 added reconciliation states/previews.
- Latest recursive reaction scan after the recording consolidation, including hidden descendants and conditional actions: 12,601 reactions; no missing destinations, no non-variant NODE destinations outside Staging, and no SCROLL_TO targeting another frame.
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


## Recording and contents follow-up · 9 Nov 2026
The user supplied a screenshot of comment #51, “@agent visual bug here,” on `1939:25765`. The supplied screenshot located the defect; the native review comment was not read, replied to or resolved by the agent.

Added recording pages contained direct contents labels without the visibility binding used by the existing contents toggle. Collapsing the column left those labels visible at a narrow width. Bound 213 direct contents labels to the existing expanded-state variable. Fifty compact Edit page/Edit recording artifact/Page history/Edit transcript/Mark transcript as reviewed actions had locally hidden icons; restored their pencil/history/check instances without modifying shared masters.

The user selected one recording object. Staging now uses Microscope orientation recording (`1.1.1.1.1`, entry `1939:25765`) for both introductory and training excerpts. Removed 188 duplicate tree rows; 194 recording tree instances link to the canonical reader. Review a recording, the home listing and ordinary search/review-notification entry paths converge there. Duplicate home/search recording links were removed. Existing passage-focused annotation states and comment-anchor frame identities remain. The combined reader/editor/review/note states share the filename and 18:04 duration; older introductory states identify their excerpt boundary rather than claiming that the recording ends at 01:12. Original Workshop source content is unchanged.

The post-change scan covers all 233 Staging frames: no duplicate recording rows within a tree, no old training-recording title/filename, no hidden icons on the specified actions, and no direct contents text missing its visibility binding. Reader and editor renders show one selected recording, visible action icons and both excerpt sections. This is structural/render evidence; the follow-up native toggle, review and return-path rehearsal could not run because the Chrome debugging connection did not open. Earlier playback results above predate this consolidation, including the changed Review a recording launcher destination.

## Native recording follow-up · 9 Nov 2026
After the user allowed the browser connection, exercised the selected route in a dedicated native Figma tab at 1920 × 842 CSS pixels, device scale 1. No source Workshop, shared component or Prototype nodes were changed.

Playback exposed a second contents defect: twenty-one direct labels still used Fill/Stretch sizing, unlike the established fixed-width contents labels. After collapse/re-expansion and navigation to the reviewed state, their text could wrap one character per line. Matched those labels to the existing 140-pixel label width with height auto-resizing, preserving visibility bindings and scroll reactions. Reloaded the prototype from the launcher and repeated the collapse/reopen/review sequence successfully.

| Exercised path | Result |
| --- | --- |
| Launcher Review a recording → `1939:25765` | Lands on the unified orientation recording; one recording tree row; Edit recording artifact, Page history, Edit transcript and Mark transcript as reviewed retain their icons. |
| Page contents collapse → wiki tree collapse → reopen wiki tree → reopen page contents → mark reviewed | Collapsed labels are hidden. After the sizing repair, labels remain readable in reviewed state `1939:25938`; the review CTA disappears and Reviewed by Mei Chen appears. |
| Reviewed recording → Page history `1939:26281` → Back to artifact | Returns to `1939:25938`; before/after PNGs are byte-identical. |
| Reviewed recording → Edit transcript `1939:25848` → wheel-scroll to audio → Cancel | Audio is below the transcript and reachable; sticky editing controls remain available. Returns to `1939:25938`; before/after PNGs are byte-identical. |
| Reviewed recording → Edit recording artifact `1974:33650` → Cancel | Returns to `1939:25938`; before/after PNGs are byte-identical. |
| Recording tree → Getting started `1934:4394` → Edit page `1934:13459` → Cancel | The page action icons render; edit opens correctly and cancel restores the same guide. Before/after PNGs are byte-identical. |

This rehearsal exposed two additional defects: clicking Audio in the transcript editor’s contents panel did not visibly scroll, although wheel scrolling reached the player; the right-hand review comment text in recording history clipped at the card edge. Their subsequent repair is recorded below. This rehearsal does not establish every contents anchor, Save outcome, source preview, annotation branch or audio playback simulation. No real audio playback was claimed or tested.

## Audio anchor and history comment repair · 9 Nov 2026
The user authorized these two follow-ups within Staging. No Workshop, Prototype, shared component master or reaction destination changed.

The transcript editor’s Main content frame was marked vertically scrollable even though it hugged its content and had no independent overflow. The nested Audio target therefore selected a non-scrolling ancestor. Set the redundant nested overflow to None, leaving vertical scrolling on the outer screen. The same condition existed in note draft `1939:26079` and saved-note state `1939:26184`; corrected those two local frames as well. The other inspected recording states already had no redundant Main content overflow.

The history comment’s Message slot was fixed at 350 pixels inside a 314-pixel message area. The slot and its text now fill the available width and hug text height, preserving the card’s clipping and border. Applied the same width rule to the local history-screen chat message slot. Shared message components are unchanged.

Native playback at 1920 × 842 CSS pixels, device scale 1:

- Started at the launcher, opened Review a recording, marked the transcript reviewed, and entered Edit transcript. Clicking Audio in `1939:25848` brought the player into view without wheel scrolling. The sticky editing controls remained available.
- Cancel returned to reviewed state `1939:25938`; its PNG was byte-identical to the reviewed baseline.
- Opened Page history `1939:26281`. The complete comment reads: “Checked the transcript against the audio. The speaker labels and instructions match.” Both lines fit inside the card.
- Back to artifact returned to `1939:25938`; its PNG was byte-identical to the reviewed baseline.
- Directly opened saved-note state `1939:26184` and clicked its Audio contents item. The player became visible while the note rail and collapsed endnotes remained present. This checks that anchor from a direct entry, not the complete note-creation sequence.

Screenshots: `staging-audio-anchor-repaired.png`, `staging-history-comment-repaired.png`, `staging-note-audio-anchor-repaired.png`; return comparisons used `staging-audio-anchor-cancel.png` and `staging-history-comment-return.png`. These temporary images are not committed. The note-draft state received the matching structural correction but was not separately rehearsed. No actual audio playback or exhaustive route verification is claimed.
