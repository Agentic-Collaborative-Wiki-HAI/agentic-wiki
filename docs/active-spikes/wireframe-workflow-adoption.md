---
attention: live
---
# Wireframe workflow adoption
Apply the approved naming and exploration conventions to Aslan's Figma work, preserve Erwin's pages, and make the workflow collection available through a project branch and a teammate setup prompt. The shared Archive retains original ownership at group level and sits last.

## Boundaries
Existing frame IDs and prototype interactions are preserved. Historical product snapshots remain in Workshop; superseded design explorations move to Archive. The Component Library and Erwin's pages are outside the mutation scope. Global links are optional per client; project-local skills take precedence.

## Deliverables
- [Project conventions](../design/wireframe-conventions.md), [board lookup](../design/figma-board-map.json) and [design file manifest](../../design-files.json).
- Shared collection on `project/agentic-wiki`, with the exact parent pin controlling adoption.
- README setup prompt that loads this project's instructions and both workflow collections before offering global discovery.
- Preview-first global-link installer with conflict protection and offline tests, developed in the shared collection.

## Publication
Local setup does not make unpublished commits available to teammates. Publish the reviewed shared workflow revision and project branch before the parent adoption commit, and verify a fresh recursive clone. Do not replace a missing pin with an older remote version. No pushes are part of this task.
