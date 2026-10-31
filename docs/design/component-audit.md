# Component audit
Snapshot: 2026-10-31. Focused checks of editing bars, thread actions and retirement candidates in the [Component Library](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=593-3103). Dependency verification explicitly includes invisible instance descendants. Instance counts in this report describe the named families, not a complete file inventory.

## Editing bars
[Workshop/Editing bar](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1226-1569) has five semantic variants: Page, Tracked page, Transcript, Artifact and Transcript view. Aslan’s Workshop has 66 instances; the library has 5 specimens and Archive has 1 instance. Erwin’s Workshop has none, including invisible descendants.

The inspected overrides are dimensions, names, reactions and variable bindings. No text, font, paint or visibility overrides were reported. Existing variants already encode the main editing contexts. Show return action is a component property; nested formatting and compact actions remain shared instances. Destination-specific navigation belongs on host instances, not in a variant for every route. Document-specific endnotes already use an Entries slot.

The [saved transcript correction](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=950-17232) uses Transcript view and Edit transcript returns to the correction editor at `950:16426`. The instance retains its host wrapper and dimensions. All 165 directly referenced destinations inspected across editing bars resolved before the repair; the repaired destination also resolves. Sticky wrappers were preserved, but runtime sticky behavior was not replayed in this pass.

## Thread actions
[Workshop/Thread actions](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1107-2453) has 68 Aslan instances when hidden descendants are included, 1 library specimen and 6 Archive instances. Erwin’s Workshop has none.

Of the 68 Aslan instances, 29 have resolution bindings. All 29 hide the associated active thread and expose a resolved-history link, with destinations `1109:139382` or `1109:140330`. The remaining 39 controls are hidden and have no active Resolve action, including draft and historical compositions. This is structural action verification, not a complete prototype playback test.

The family description documents the direct compact Resolve thread action and separate acceptance/rejection semantics. The Closed variant is the current direct action; Menu is retained for compatibility. No thread behavior or shared visual source was changed.

## Component lifecycle descriptions
Descriptions distinguish current Editing bar, Thread actions and Contextual endnotes from legacy Endnotes, Thread card, Message, Reply composer and Suggestion thread specimens. The legacy endnotes source remains because Archive uses it. Descriptions are maintenance guidance. The three separately approved retirements are recorded below; shared visual foundations remain unchanged.

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
