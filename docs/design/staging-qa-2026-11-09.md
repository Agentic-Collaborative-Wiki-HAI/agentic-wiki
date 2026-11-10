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

## Interaction and layout repair batch · 9 Nov 2026
Scope: the user authorized Staging repairs, returning misplaced Prototype material to Erwin’s Workshop, and keeping only reviewer transfer from the handover flow. Original Workshop material, Showroom and shared Component Library definitions remain unchanged. This batch does not promote Staging or settle the proposed Home/Maintenance consolidation.

### Implementation
- A Staging-local header component is used by 209 instances. Search is centered within the screen, breadcrumbs remain one line with truncation when necessary, and Maintenance joins the right-hand navigation controls. Existing route actions and visibility bindings were preserved. Page-contents toggle padding aligns the graphic with its labels.
- References/Comments content begins below the sticky tab controls. This repairs both the empty hint bleeding behind the tabs and the tracked-edit thread header being covered. The reported fragment was rail content, not an intentional comment-count badge.
- Staging-local conversation/composer components and variables isolate global chat from inherited scripted-state overrides. The same composition is used on 198 screens; the explicit timestamped-note drafting route retains its dedicated conversation. Launcher entries initialize a fresh activity. Existing conversation persists while moving around within an activity rather than being reset whenever the drawer closes.
- Chat Finder selection attaches the MP3; Send begins classification and confirmation. View artifact closes chat and navigates to the canonical recording. Its missing navigation was discovered and repaired during this rehearsal.
- Empty artifact title fields update only their title variable. The upload control’s outer container no longer advances the route. Both recording and audio-note preparation states have completion transitions; the recording transition was exercised below.
- Seven rich recording states have a 360-pixel scrolling transcript window with audio immediately below. This includes the reader, reviewed state, editor, timestamped-note states and artifact editor. It is not a claim that every older passage-focused fixture has the same layout.
- Endnote filters use dedicated Staging components with a full switch and label. The first attempted icon substitution retained the compact icon geometry in playback; the dedicated components resolve that sizing problem. Their host reactions retain the existing filter variables.
- Centrifuge reply fields target the Centrifuge reply draft. Planning a microscope session has normal incoming tree links. Handover tree rows are removed; Settings opens reviewer transfer. The three handover-writing states are preserved in Archive `2043:77430`.
- The recording shortcuts are labelled Related page: Using the microscope and Ask agent to draft a timestamped note. They were integration shortcuts, not new recording controls or verbatim imported source labels.
- Seven misplaced Prototype sections containing 25 screens were moved intact to a separate area in Erwin’s Workshop. Prototype is empty. F13 remains held for joint review; moving those copies does not integrate or accept F13.

### Native playback
Used a dedicated native Figma tab at 1920 × 842 CSS pixels, device scale 1. The selected checks establish the behavior below, not exhaustive coverage of every state or caller.

| Exercised path | Result |
| --- | --- |
| Guide `1934:4394` → Chat → Add file to chat → Finder MP3 → Open → Send → Primary recording → Add artifact | Chat initially opens empty; attachment and classification happen in the drawer. The underlying guide remains unchanged through confirmation. |
| Planning page `1934:37358` → the same chat intake → View artifact | After repairing the missing navigation, reaches canonical recording `1939:25765` with chat closed. The Planning article remains unchanged before explicit View artifact navigation. |
| Guide → Add artifact → click title | Title fills with Microscope orientation recording; the modal stays open, no file is selected, and Add remains disabled. |
| That modal → Upload file → Finder MP3 → Primary recording → Add and transcribe | Shows Creating the transcript, then automatically displays the populated canonical recording. This is a timed fixture, not real transcription. |
| Recording → outer scroll to transcript → wheel inside transcript | Transcript advances from introductory to training passages while the audio remains in the same visible position beneath the window. |
| Recording → Edit transcript `1939:25848` → Audio contents link | Brings the player and transcript window into view; editing controls remain available above. |
| Guide references `1934:4549` → expand Endnotes → Citations only switch | Full switch is visible. Turning it on leaves the citation and hides the ordinary endnote; the margin cards remain unchanged. |
| Centrifuge pending `1939:27311` → reply text field | Reaches populated Centrifuge draft `1939:27433`, retaining the Centrifuge article and suggestion. This does not test every subsequent decision branch. |
| Tracked edit `1934:13583` | Native screenshot shows the author, draft label and full reply area below the References/Comments tabs. |
| Tracked edit’s wiki tree → Planning a microscope session | Opens `1934:37358`; it is reachable through ordinary navigation. |
| Recording header → Settings → Transfer a reviewer role | Opens `1939:26631` with Settings / Page reviewers breadcrumbs and no wiki tree, page contents or chat. Invitation acceptance/refusal was not re-rehearsed in this batch. |

The final graph scan covers 230 Staging frames and 12,143 reactions, with no missing/outside-Staging NODE destinations and no foreign scroll targets. It cannot detect omitted interactions or prove context preservation. The board map records the five local component definitions, retired states and preserved Prototype section identities.

Temporary screenshots include `staging-batch-title-only.png`, `staging-batch-transcribing.png`, `staging-batch-transcript-completed.png`, `staging-batch-transcript-inner-scroll.png`, `staging-batch-editor-audio.png`, `staging-batch-citations-only.png`, `staging-batch-centrifuge-reply.png`, `staging-batch-edit-thread.png`, `staging-batch-planning-entry.png`, `staging-batch-chat-view-artifact.png`, and `staging-batch-reviewer-transfer.png`. They are not committed to this public repository.

### Remaining boundaries
The audio-note completion transition received structural verification, not a new full upload rehearsal. Global chat uses shared scripted examples; context-specific answers on every newly integrated page, reset behavior across all caller combinations, non-audio intake branches and alternate source/annotation editing routes remain bounded follow-up work. No real file upload, audio playback, transcription, agent execution or permission enforcement is claimed. Home Needs attention versus Maintenance remains a design discussion. Joint acceptance and promotion remain separate decisions.
