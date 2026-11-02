---
id: FOLLOW-WORKFLOWS
type: operational
scope: "Agent selection of project procedures, configuration and authority."
status: adopted
authority: "Aslan approved D2 (the eleven-policy adoption and associated routing) in the active Agentic Wiki migration chat on 2026-11-02."
revised: "2026-11-02"
depends_on: ["RESOLVE-DEPENDENCIES"]
related_to: []
---
# Follow project-owned workflows
The complete Project Workflows collection is adopted at `project-workflows/`. Read [workflow.toml](../../project-workflows/workflow.toml), then load `project-workflows/<skill-name>/SKILL.md` before global seeds. The child uses `project/agentic-wiki`; this repository keeps its own branch and exact gitlink. Project rules here override shared defaults, and actual project documents stay outside the collection.

The existing procedures include run-project-spike, coordinate-project-contribution, commit-work, triage-project-misc, capture-for-later, draft-approval-slate, setup-project-workflows, and update-project-workflows. The independent `skills/write-skills/` procedure and its client metadata stay local. Existing notes and deferred work are preserved. Use capture-for-later for new small individual scratch notes; update related existing material without bulk migration or creating tasks.

Use [Coordinate Project Contribution](../../project-workflows/coordinate-project-contribution/SKILL.md) to start contribution and review work under this project’s existing human direction. Each agent keeps its own spike; a branch may carry several spikes/issues and later reviews. Existing privacy, validation and publication rules remain authoritative.

Use the selected collection’s Create, Edit, Review, Adopt and Retire Agent Policies skills for their respective policy tasks. Package delivery supplies candidate source; it does not adopt project policies or change tool permissions.
