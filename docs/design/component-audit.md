# Component audit
Snapshot: 2026-10-31. Focused checks of editing bars, thread actions and retirement candidates in the [Component Library](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=593-3103). Dependency verification explicitly includes invisible instance descendants. Instance counts in this report describe the named families, not a complete file inventory.

## Editing bars
[Workshop/Editing bar](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1226-1569) has five semantic variants: Page, Tracked page, Transcript, Artifact and Transcript view. Aslan’s Workshop has 66 instances; the library has 5 specimens and Archive has 1 instance. Erwin’s Workshop has none, including invisible descendants.

The inspected overrides are dimensions, names, reactions and variable bindings. No text, font, paint or visibility overrides were reported. Existing variants already encode the main editing contexts. Show return action is a component property; nested formatting and compact actions remain shared instances. Destination-specific navigation belongs on host instances, not in a variant for every route. Document-specific endnotes already use an Entries slot.

The [saved transcript correction](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=950-17232) uses Transcript view and Edit transcript returns to the correction editor at `950:16426`. The instance retains its host wrapper and dimensions. All 165 directly referenced destinations inspected across editing bars resolved before the repair; the repaired destination also resolves. Sticky wrappers were preserved, but runtime sticky behavior was not replayed in this pass.

## Thread and comment composition
[Workshop/Comment thread](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1523-1503) has Open, Draft and Resolved variants with Messages and Reply slots. The 74 migrated Workshop instances comprise 29 Open, 9 Draft and 36 Resolved threads. Draft and Resolved variants omit resolution controls; Resolved also omits the reply composer. [Open thread actions](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1523-1504) contains the direct compact Resolve action, without a hidden kebab alternative. Resolution and acceptance/rejection remain separate operations.

[Workshop/Comment body](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1528-243962) uses Reading and Editing variants with content/editor slots. Fourteen inline editors use this family. Suggestion status stays outside editable prose. Existing editing eligibility and route-specific actions remain host responsibilities.

## Component lifecycle descriptions
Descriptions distinguish current Editing bar, Thread actions and Contextual endnotes from legacy Endnotes, Thread card, Message, Reply composer and Suggestion thread specimens. Legacy source families remain for historical consumers, including Archive. Current Workshop uses the successors listed below. Descriptions are maintenance guidance. The three separately approved retirements are recorded below; shared visual foundations remain unchanged.

## Approved retirements
The user approved C1–C3. Their instance-consumer checks were repeated before deletion; the preceding full-file scan found no prototype or component-property references, including hidden descendants. All three are absent from the Component Library after deletion.

| Approval | Retired component | Former node ID |
| --- | --- | --- |
| C1 | Workshop/Context notice | `580:441` |
| C2 | Workshop/Discussion content | `580:443` |
| C3 | Workshop/Thread entry | `664:4378` |

### Keep these hidden-state dependencies
| Family | Aslan Workshop | Library | Archive |
| --- | ---: | ---: | ---: |
| Workshop/Icon/Upload | 157 | 2 | 26 |
| Workshop/Icon/Text | 680 | 110 | 10 |
| Icon/Bold | 68 | 11 | 1 |
| Icon/Italic | 68 | 11 | 1 |
| Icon/Quote | 68 | 11 | 1 |
| Icon/Code | 136 | 22 | 2 |
| Workshop/Inline comment editor | 14 | 0 | 6 |

None of these candidate families has an Erwin Workshop consumer in this scan. That does not authorize changing other shared primitives or tokens, which can have cross-owner dependencies.

## Evidence labels and limits
All 67 obsolete evidence-role instances were deleted from Aslan’s Workshop, including nested metadata and transcript-slot content. A follow-up scan with invisible descendants included found no remaining instances of the evidence-role family there. Visible Primary recording metadata remains. The source family and historical Archive consumers are retained.

The first slot badge was removed through its live instance handle; the remaining 66 were removed in a batch whose before/after visible-text and prototype-reaction checksums matched. No active visibility bindings or reactions were found on the removed badges. A representative recording view was rendered after cleanup.

No Erwin page or shared visual foundation was changed. Checks do not establish external-file consumers, arbitrary detached visual copies, full override correctness outside the named families, or runtime scrolling and state persistence. The saved-transcript repair was visually checked.

## Approved H1–H6 cleanup
The implementation is scoped to Aslan’s Workshop and isolated successor components. Erwin’s pages, existing shared visual foundations, and historical Archive compositions were not edited. Old source components are retained for historical consumers; current successors do not require obsolete hidden UI.

| Area | Current composition | Migration |
| --- | --- | --- |
| Header and metadata | [Wiki header · current](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1522-149119) and [Page metadata](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1522-149155), with metadata slot | 293 instances; no Shared with lab or obsolete metadata actions |
| Document margin | [Document margin body](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1523-1476), References/Comments variants with Content slot | 146 instances; sticky host tabs own navigation |
| Chat | [Chat drawer](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1525-2664) with Conversation slot and [conversation states](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1525-2576) | 157 drawers; one selected conversation stage |
| Chat input | [Composer host](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1525-3189) with Composer slot and [Empty/Draft/Attachment draft](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1525-2663) variants | 155 viewport composer hosts |
| Formatting | [Semantic formatting toolbar · current](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1526-1850) with shared anchored Paragraph, Lists and Insert popovers | 66 nested toolbars; sticky wrappers retained |
| Lightweight links | [Link destination popover](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1530-25336) with Content slot | Two instances omit obsolete hidden close control |
| Citation authoring | [Unified source flow](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=988-16150&scaling=scale-down-width) | Old entry points lead to wiki search/URL/DOI and Enter source details; whole-source default and optional passage selection |

The initial removal batch deleted 205 obsolete layers, including superseded page contents, old status badges and link fields. Current sticky contents and optional icon properties remain. Notification overlays and contextual endnotes were already shared and were retained.

Chat stage, composer state and draft text use string variables; legacy routing booleans remain where existing routes still require them. Route-specific Finder and evidence-preview destinations belong on persistent Workshop hosts. Evidence preview currently has a conversation-level click target in answer states rather than a text-range-only target; that prototype affordance needs playback review. Do not assume editing a dynamically substituted variant descendant preserves an instance override after another state switch.

## Verification and remaining QA
Rendered the guide, active comment thread, chat drawer, agent-answer state and unified citation dialog. Checked dynamic variant switching and restoration of initial variable defaults. All 567 unique destinations referenced by Workshop prototype reactions resolve. This verifies destination existence, not full behavioral correctness. Full native prototype playback, sticky scrolling, typing simulation, and every route-state combination remain QA work under issue #35.

One visible legacy “Checked against audio” text remains at node `512:548`; it was outside the approved hidden-layer removal and should be reviewed for current attribution wording. Historical and unrelated shared components were not globally purged. No application code, backend evidence enforcement, or published team library changed.
