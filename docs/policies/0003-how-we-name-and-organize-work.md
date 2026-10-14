# 0003 How We Name And Organize Work
## Current Approach
**Four terms, at different resolutions.**

**Epic** — a GitHub issue type. A durable topic that accumulates work over time and stays open. *Design the interface.* An epic is a thin organizing shell; it has no natural end and may sit open while children close beneath it.

**Issue** — a piece of work someone owns and others need visibility into. **A plain issue is a task**, so there is no `Task` type — the type would only restate what the issue already is. Types exist for the two cases that carry real information: `Epic` and `Bug`.

**Spike** — a **bounded chunk of work carrying its own docs**: a concept doc plus a `.todo.md` in `docs/active-spikes/`. A spike produces real work, not throwaway code.

**task** — lowercase. A checkbox inside a spike's `.todo.md`. Never a GitHub object.

**Spikes end. Epics do not.** When related work comes around again, open a successor spike rather than reopening the original — `wcag-seo`, then `wcag-seo2`. The concept doc's whole value is that someone can read it and understand the shape of the work; reopening indefinitely makes it sprawl past the point of being followable.

**Issues and `.todo.md` files do not compete, because they are not at the same resolution.** An issue is a unit of human-visible ownership. A checkbox is a step. You do not file issues for checkboxes.

**Not every issue has a spike.** Design and research work performed in Figma or elsewhere is tracked here for visibility and co-location, with no repo docs behind it at all.

## Rationale
GitHub issues and the project board coordinate human ownership and visible work. Spike conceptual docs and `.todo.md` files preserve agent reasoning and granular implementation state. They serve different readers at different resolutions. Here a spike means a bounded body of real work, rather than an XP-style throwaway investigation; clear vocabulary prevents the two systems from competing.

## Operating Notes
**Easier:** no argument about whether something is a spike or a task, because they are not alternatives — they are different resolutions. No duplicated task tracking. An agent reading a fresh clone finds the concept doc; a person reading the board finds the issue.

**Harder:** two places to look when reconstructing what happened. Mitigated by issues linking their spike doc and spike docs recording their issue number.

**Deliberately kept:** archived spikes are not deleted. They are how a future agent learns what was already tried and rejected.

**Out of scope:** unifying the two systems into one. They serve different readers at different resolutions, and collapsing them would make one of them worse.

## Links
- [AGENTS.md](../../AGENTS.md) · [TODO.md](../../TODO.md) · [skills/run-project-spike/SKILL.md](../../skills/run-project-spike/SKILL.md)
