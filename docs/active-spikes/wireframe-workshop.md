---
attention: live
---
# Wireframe Workshop Consolidation
Opened 2026-10-21. Slug: `wireframe-workshop`.

## Purpose and scope
Maintain Aslan’s connected working prototype for evidence intake, source-grounded suggestions and anchored correction, with the related editing, chat and history routes. These are working wireframes for design review, not a settled team specification or a functioning application. Erwin’s pages remain independently owned and unchanged by this work.

The [board lookup](../design/figma-board-map.json) identifies 178 Workshop boards by current name and unchanged node ID, retaining old route codes as aliases. The [Cover](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1507-2) links to the working routes and repository. Staging, Prototype and Showroom remain unpopulated; Workshop has not been promoted into them.

## Current design basis
- Recordings are typed objects with audio and an optional transcript presented together. Audio format permits Primary recording or Audio note; the type is selected before adding the artifact. Typed relationships and source links express provenance without standalone Primary/Secondary/Tertiary labels. Transcription is not automatically another evidence tier.
- Add artifact from a page defaults to a child of that page. Chat attachment, sending a message and adding a wiki artifact are distinct steps; structured intake proposes a destination before explicit confirmation. File selection uses simulated macOS-style picker routes. Format selection is not a claim that every format has a complete processing workflow.
- Chat is a full-height right drawer over the header and document margin, with a toggle in the same screen position when open or closed. It does not dim or reflow the document. Closing it restores the underlying context. Profile and Settings omit chat and wiki/page contents navigation.
- Navigation follows page hierarchy. Linking, citing, deriving from and organizing under remain different relationships. Both contents navigations can collapse and stay available during scrolling; long-form body and endnotes widths are bounded and centered. The header and References/Comments tabs stay available while their anchored cards follow the relevant content.
- Recording pages show type, uploader/date and description near the title. Transcript text retains timestamps and speakers; audio sits beneath it in reading and editing views. Transcript editing and marking reviewed are transcript-level actions. AI-generated, reviewed and revised attribution are distinct, with attribution links into history. Redundant Details metadata is not an active route requirement.
- Page editing includes the title. A shared editing bar starts below it and becomes sticky on scroll. Editing sits above the formatting row; Track changes follows formatting, with compact Cancel then Save/Submit actions on the right. Transcript editing uses a variant of the same component. Creation controls and routine page attribution are omitted in edit mode.
- Shared prose changes proposed by agents require review. Suggested edits use inline deletion/addition styling and per-suggestion Accept/Reject controls in the related conversation. A reply can request another suggestion with @agent; ordinary human replies do not trigger an agent response. Accepting a suggestion and resolving its thread are separate actions.
- Comments anchor to selected text, including selections within or across blocks. Each thread encloses its replies and composer, with a compact Resolve thread action at the top. Resolved threads leave the active margin and remain discoverable through history. Editing a comment changes its text, not a suggestion’s decision status; restrictions on editing older messages still need discussion.
- A citation is a structured note. Numbers identify notes, not sources: reusing a note reuses its number; distinct passage notes can have distinct numbers. Optional citation titles supplement the number. A unified wiki search or URL/DOI input identifies sources; manual source details cover material without those identifiers. Whole-page citation is the default, with optional passage selection in a preview rather than required timestamp entry.
- Margin notes can contain inline links. Lightweight selection-anchored link and note authoring preserve editing context. Conversation citations use descriptive links instead of document endnote numbers. Source previews preserve the conversation context.
- Numbered notes appear both in the margin and in shared, collapsed-by-default endnotes. Expanded endnotes use a continuous list with citation-only filtering and return links. References/Comments switching must preserve article content and editing state.
- History connects an event, its contextual inline diff and associated conversation on the right. It omits wiki/page contents and creation controls; Back to page/artifact aligns with the title. Accepted changes create content revisions; rejected suggestions remain review events. Resolving a conversation does not imply a content change.
- Reused text links to its source; source editing does not silently edit a local copy. Changed-source notices belong on affected citation cards. A source change does not authorize rewriting shared prose. Primary evidence replacement creates a different artifact identity; deletion and dependent reprocessing semantics remain unresolved.

## Open questions
- Scope and presentation when a suggestion affects text beyond its original comment anchor; superseding earlier suggestions and editing suggestions before acceptance.
- Versioned comment citations and editing restrictions for older messages; behavior when cited material is deleted or replaced.
- Broader file-type processing, images within documentation, narrow-screen behavior and private-space interiors.
- Arbitrary repeated actions, persistence and application behavior beyond the fixed example routes.

## Exploration and ownership
Keep each person’s Workshop immediately followed by their Scratchpad, with established owner order preserved. Aslan’s Scratchpad contains two Parked groups for navigation alternatives and future ideas. Shared Archive is last, with 12 historical groups attributed to Aslan. Product history screens remain in Workshop; they are active routes, not obsolete design work. The [project conventions](../design/wireframe-conventions.md) define addresses and preservation boundaries.

## Evidence and verification limits
Meeting records and earlier explorations are inputs, not automatic authority. The user’s current decisions govern this working design. Detailed private references and recovery material stay under ignored `reference-material/wireframe-workshop/`; do not copy private transcripts, participant information or teammate artifacts into tracked docs.

The prototype uses simulated example data, file selection, messages, transcription and playback states. It does not establish real upload, audio playback, arbitrary text entry or persistent application state. Historical checks in the [task record](wireframe-workshop.todo.md) describe the state tested on their dates; they are not current whole-route certification. The [component audit](../design/component-audit.md) is a structural reference check, separate from visual acceptance and end-to-end playback.
