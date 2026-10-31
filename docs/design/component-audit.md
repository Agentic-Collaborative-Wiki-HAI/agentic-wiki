# Component usage audit
Snapshot: 2026-10-31. Read-only inspection of the shared [Component Library](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=593-3103), both owners’ Workshops and Scratchpads, and Archive. No component sources or teammate pages were changed.

## Inventory
| Area | Instances | Missing main-component references |
| --- | ---: | ---: |
| Aslan’s Workshop | 8,890 | 0 |
| Erwin’s Workshop | 1,514 | 0 |
| Component Library | 956 | 0 |
| Aslan’s Scratchpad | 0 | 0 |
| Erwin’s Scratchpad | 0 | 0 |
| Archive | 1,287 | 0 |

Counts include nested and hidden instances, not just visible controls. The library contains 356 component masters in 115 families (sets or standalone components). All main components referenced by Aslan’s Workshop resolve to that library inventory. Erwin’s Workshop also contains a local Lab page tree source.

## Findings
- **Shared impact:** 26 library families have consumers in Erwin’s Workshop. Changes to shared masters require checking those consumers even when initiated from Aslan’s work. This audit does not authorize changing them.
- **Editing bar is shared:** [Workshop/Editing bar](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1226-1569) has five variants and 66 instances in Aslan’s Workshop: 41 page, 1 tracked, 19 transcript, 2 artifact and 3 transcript-view. The audit establishes reuse, not that every route-specific action has been replayed.
- **Evidence-role remnants are hidden:** [Workshop/Evidence role](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=987-1624) has 23 Aslan instances: 1 Primary and 22 Secondary. Each has an invisible ancestor or is itself invisible. These do not establish visible tier labels contrary to the current direction; removing unused structure is a separate cleanup.
- **Endnotes have two generations:** [Contextual endnotes](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1260-145893) has 21 current Aslan instances. [Workshop/Endnotes](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1024-1315) has no Workshop placements but has 3 Archive instances. Deleting the older family would affect preserved history.
- **Thread actions are shared:** [Workshop/Thread actions](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1107-2453) has 26 Aslan instances and 2 variants. Its behavioral consistency needs route playback, beyond this reference inventory.

## Families without placements in either Workshop
These are investigation candidates, not safe-deletion findings. Library specimens, nested dependencies, Archive consumers and interaction targets can still require them.

| Family | Library instances | Archive instances |
| --- | ---: | ---: |
| [Workshop/Icon/Chevron right](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=574-326) | 2 | 1 |
| [Workshop/Icon/Pause](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=574-334) | 3 | 0 |
| [Workshop/Status](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=574-433) | 5 | 0 |
| [Workshop/Thread card](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=576-374) | 1 | 0 |
| [Workshop/Message](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=576-376) | 2 | 0 |
| [Workshop/Reply composer](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=576-389) | 1 | 0 |
| [Workshop/Suggestion thread](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=576-398) | 1 | 0 |
| [Workshop/Context notice](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=580-441) | 0 | 0 |
| [Workshop/Discussion content](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=580-443) | 0 | 0 |
| [Workshop/Icon/Upload](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=628-751) | 0 | 0 |
| [Workshop/Thread entry](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=664-4378) | 0 | 0 |
| [Workshop/Icon/Text](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=757-1025) | 0 | 0 |
| [Workshop/Notifications panel](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=804-1048) | 1 | 0 |
| [Icon/Bold](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=925-51207) | 0 | 0 |
| [Icon/Italic](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=925-51209) | 0 | 0 |
| [Icon/Quote](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=925-51213) | 0 | 0 |
| [Icon/Code](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=925-51215) | 0 | 0 |
| [Workshop/Endnotes](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1024-1315) | 0 | 3 |
| [Workshop/Inline comment editor](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd?node-id=1159-1453) | 0 | 0 |

## Recommended follow-up
Check variant-change targets, nested dependencies and all consumers before consolidating or retiring any family. Prioritize hidden evidence-role remnants and overlapping legacy thread/endnotes patterns, while preserving Archive and Erwin’s current consumers. Make lifecycle and replacement guidance explicit in component descriptions before removal.

This audit does not detect every detached visual copy, establish semantic equivalence between components, exhaustively verify style/token bindings, or test every override, prototype action and viewport. Those require a separate visual and interaction consistency pass. No library cleanup was performed.

