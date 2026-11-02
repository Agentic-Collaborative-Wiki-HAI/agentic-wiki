---
id: ORGANIZE-WORK
type: operational
scope: "Agent work records and coordination in this shared team project."
status: adopted
authority: "Aslan approved D2 (the eleven-policy adoption and associated routing) in the active Agentic Wiki migration chat on 2026-11-02."
revised: "2026-11-02"
depends_on: ["RESOLVE-DEPENDENCIES"]
related_to: []
---
# Follow shared work conventions
**People self-select work rather than being assigned roles.** The team's actual capacities are not yet known, and small real work reveals them better than role labels handed out up front.

### The vocabulary
**Epic** — an issue type. A durable topic that accumulates work over time and stays open. *Design the interface.*

**Issue** — a piece of work someone owns and others need visibility into. Most issues are plain issues; a plain issue **is** a task, so it needs no type saying so.

**Spike** — a **bounded chunk of work with its own docs**, living in `docs/active-spikes/` as a concept doc plus a `.todo.md`. **Not** an XP-style throwaway investigation — a spike here produces real work.

**task** — lowercase. A checkbox inside a spike's `.todo.md`. Never a GitHub object.

### Why spikes are bounded
A spike has an end. **When related work comes back around, open a successor rather than reopening the original** — `wcag-seo` then `wcag-seo2`. Reopening makes the doc sprawl until it is no longer followable, and the whole point of the concept doc is that someone can read it and understand the shape of the work.

An epic spans one or more spikes over time. **Not every issue has a spike** — design and research work done in Figma or elsewhere is tracked here for visibility with no repo docs at all, and that is fine.

### Where each audience reads
**Issues and the board are for humans.** The `.todo.md` is agent working memory and is far more granular. They do not compete because they are not at the same resolution.

Archived spikes are deliberately kept rather than deleted: they are how a future agent gets up to speed on what was already tried.

When an approach becomes durable agent guidance, keep its obligation in `docs/agent-policies/`; shared product and team guidance remains in `docs/policies/`. Apply ADOPT-POLICIES’ policy/skill distinction and amendment boundary. Preserve current guidance and useful rationale, with Git recording evolution. Keep development chronology and abandoned alternatives in Git or archived spike notes rather than accumulating immutable ADRs. No fixed Context / Decision / Consequences format is required.

The [shared work vocabulary](../../docs/policies/0003-how-we-name-and-organize-work.md) retains the team-facing conventions and rationale. This agent policy governs agent work; it does not change human governance.
