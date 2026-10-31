# Agentic Wiki wireframe conventions
The [file manifest](../../design-files.json) maps page roles and ownership. The shared collection's [numbering policy](../../wireframe-workflows/policies/board-numbering-and-layout.md) and [Scratchpad/Archive policy](../../wireframe-workflows/policies/scratchpad-and-archive.md) govern Aslan's migrated work. Erwin's pages retain their existing conventions until he chooses to adopt them.

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
| `P0` | Prototype routing frame, outside the product hierarchy |
| `O1`–`O10` | Standalone overlay/partial demonstrations, identified individually in the board lookup |

The `O` family is local to this project; it is not a universal convention. Full-screen host states showing a modal keep the host's address. Viewport is an optional qualifier, not a hierarchy level. Intermediate hierarchy entries need not have a separate frame.

The [board lookup](figma-board-map.json) maps former route names to current unique names, optional short codes and unchanged Figma node links. Native names carry the address, descriptive screen and state. Old codes remain aliases in the lookup for existing discussion and spike records. Historical records retain their original identifiers. Do not use a global rename across another owner's pages or shared components.

## Scratchpad and Archive
Aslan's Scratchpad retains Parked navigation alternatives and future ideas. The shared Archive is last and contains historical exploration groups labelled with their original owner. Product version-history screens remain in Workshop because they are working prototype routes, not obsolete design work.

Archived exploration preserves legacy names and node identities. It is browsable historical evidence, not a working current prototype: some old interactions already target other pages or shared source components. The current Workshop has no interaction destinations in the archived groups. Current decisions, remaining work and checks belong in the repository rather than old canvas review guides.

## Working with the collections
Use the top-level `project-workflows/` for this project's planning and records, and the pinned `wireframe-workflows/` for wireframe procedures. Its nested Project Workflows dependency governs contributions to that collection, not this project's records. Use the actual selected checkout when following relative skill links; keep full folders and supporting policies together.

Figma access is provided by the user's configured connector. Global skill links do not install or authenticate it. Creating shared components, changing other owners' work or publishing commits requires the corresponding authorization.
