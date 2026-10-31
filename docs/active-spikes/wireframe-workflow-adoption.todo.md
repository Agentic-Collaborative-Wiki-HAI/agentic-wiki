---
attention: live
---
# Wireframe workflow adoption tasks
## Implemented locally
- Renamed 178 Aslan Workshop boards while retaining frame IDs and prototype actions; meaningful context distinguishes acceptance-history variants.
- Moved 12 historical exploration/review groups into the shared Archive at the end, with Aslan identified as original owner. Two Parked exploration groups remain in Scratchpad.
- Added repository conventions, board lookup, manifest and README teammate prompt; routed project agents to both pinned workflow collections.
- Created local `project/agentic-wiki` branch in the adopted Wireframe Workflows submodule, pinned to shared installer revision `9f5e70e`.
- Prepared shared collection global-link installer and tested previews, repeat runs, conflict preflight and broken-link protection. Installed ten skills into each of the existing Codex and Claude Code discovery directories, with no conflicts.

## Verification and review
- Repository onboarding covers general contribution setup and introduces both workflow collections. Design contributors are guided through missing Figma/Penpot connection setup rather than stopped at a capability gap. Live teammate installation remains to be exercised.
- Initial and post-renaming Workshop interaction checksum: `3808836145`, covering 3,989 nodes with reactions. No Workshop targets point to archived work.
- Archive and Scratchpad orientation text visually checked. Archive is last; Erwin's pages and shared component sources were not mutated.
- Confirm the migrated layout and naming in Figma. Global discovery should be checked in fresh Codex/Claude sessions; filesystem links alone do not prove client discovery.

## Publication verified
- Published shared Wireframe Workflows `main` and `project/agentic-wiki` at `9f5e70e`, then wiki adoption `12e0f66` on `main`.
- A fresh recursive GitHub clone retrieved the exact pins: Project Workflows `2f20ab5`, Wireframe Workflows `9f5e70e`, and its contributor Project Workflows `ce2c9bc`.
- All four installer tests passed in the fresh clone. Temporary Codex and Claude installations each passed read-only preview, ten-link creation, target verification and idempotent repeat installation. The cloned worktree remained clean.
- Client UI discovery and teammate Figma access remain separate checks; temporary filesystem verification does not establish those.

## Onboarding follow-up publication
- Published Project Workflows dependency `c0e088d` on `project/wireframe-workflows`, Wireframe Workflows `78e2aad` on `main` and `project/agentic-wiki`, and the repository-wide setup and icon update `1322000` on wiki `main`.
- A new recursive GitHub clone on 2026-10-30 retrieved all exact pins, including top-level Project Workflows `2f20ab5`. All four installer tests passed. Codex and Claude each passed preview, ten-link creation, target verification and repeat installation in temporary directories. README local links and matching PNG header assets were verified; the cloned worktree remained clean.
- Direct frontmatter, relative-link and portability checks passed. The standard skill validator could not run because its Python environment lacks PyYAML. Live first-time connector authentication and client UI discovery remain unverified; the setup guidance now walks teammates through those steps.
