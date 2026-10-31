# Agent Instructions
Instructions for AI agents working in this repository. Humans should read [README.md](README.md) first.

## What This Repo Is
The term project for INF 385T.13 Human-AI Interaction (UT Austin iSchool, Fall 2026): a collaborative knowledge base where an AI agent proposes changes that humans review before they enter the shared record.

**This is a shared student team repository, not one person's project.** Multiple people with different skill levels and different amounts of time will work here. Optimize for legibility to the next person, not for cleverness.

## Public Repository — Privacy Rules
This repo is public under AGPL-3.0. Everything committed here is visible to anyone.

- **Never commit API keys, tokens, or credentials.** Model provider keys are the specific risk on this project. Use `.env` locally and repository secrets in CI. `.gitignore` covers the common patterns, but it is a safety net, not a substitute for care.
- **Never commit classmates' personal information** — no phone numbers, home addresses, private messages, or anything from Canvas that is not already public.
- **Do not commit research participant data.** If user evaluation produces transcripts, notes, or recordings, they stay out of this repo. Consent and IRB questions are unresolved; ask before writing any participant data anywhere.
- Do not commit unreleased course material, other students' work, or anything from the instructor that was not shared publicly.

## Skills Are Repo Authority
The complete Project Workflows collection is adopted at `project-workflows/`. Read [workflow.toml](project-workflows/workflow.toml), then load `project-workflows/<skill-name>/SKILL.md` before global seeds. The child uses `project/agentic-wiki`; this repository keeps its own branch and exact gitlink. Project rules here override shared defaults, and actual project documents stay outside the collection.

The eight procedures are run-project-spike, coordinate-project-contribution, commit-work, triage-project-misc, capture-for-later, draft-approval-slate, setup-project-workflows, and update-project-workflows. The independent `skills/write-skills/` procedure and its client metadata stay local. Existing notes and deferred work are preserved. Use capture-for-later for new small individual scratch notes; update related existing material without bulk migration or creating tasks.

Use [Coordinate Project Contribution](project-workflows/coordinate-project-contribution/SKILL.md) to start contribution and review work under this project’s existing human direction. Each agent keeps its own spike; a branch may carry several spikes/issues and later reviews. Existing privacy, validation and publication rules remain authoritative.

## Where Things Go
| Path | What belongs there |
|---|---|
| `docs/active-spikes/` | Current spike work — a concept doc and a `.todo.md` per spike |
| `docs/policies/` | Current rules, defaults, procedures, and useful rationale, numbered `0001-title.md` |
| `docs/scratch/` | Rough thinking, references, early notes. Not authoritative |
| `docs/archive/` | Closed or superseded spikes. History, not current rules |
| `TODO.md` | The spike index and coordination map. Keep it scannable |
| `project-workflows/` | Adopted project procedures and configuration |
| `skills/` | Independent skill-authoring guidance |

**Do not create a `README.md` inside `docs/` or its subfolders.** `TODO.md` is the index; `ls` handles the rest.

## How Work Is Organized
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

When an approach becomes durable project guidance, create or update its policy in `docs/policies/`. Policies describe current rules, defaults, procedures, exceptions, and useful rationale. Revise them directly; Git preserves their evolution. Keep development chronology and abandoned alternatives in Git or archived spike notes rather than accumulating immutable ADRs. No fixed Context / Decision / Consequences format is required.

## Working With The Team's Code
- **Do not force-push shared branches.** Do not rewrite history that others may have pulled.
- **Do not commit on someone's behalf** or amend their commits.
- If the worktree is dirty with changes you did not make, **stop and ask.** Someone else may be mid-task.
- Commit by explicit pathspec. Never `git add -A` in a shared repo — see `project-workflows/commit-work/`.
- **Do not add `Co-Authored-By:` trailers.**

## Scope Discipline
This project is at real risk of being too large for one semester. The out-of-scope list in [README.md](README.md) is load-bearing.

**If a change requires real-time collaboration, CRDTs, multi-agent orchestration, or a permissions matrix, it is almost certainly out of scope.** Say so rather than building it.

The minimum vertical slice is in the README. Work that does not serve it needs a reason.

## Markdown And Prose Style
Do not hard-wrap prose in Markdown, comments, docs, or examples. Let editors handle soft wrapping. Preserve paragraphs as single lines unless line breaks carry meaning, such as lists, tables, code blocks, quoted text, frontmatter, or an existing semantic-line-break style.

Avoid reflow-only diffs. When editing prose, change the smallest relevant span instead of rewrapping neighboring paragraphs.

When touching existing Markdown or prose, apply this preferred style to the paragraph, section, or example being edited so files converge over time. Do not mass-reformat untouched sections just to normalize style unless asked for a cleanup pass.

Prefer compact Markdown heading spacing in hand-authored docs: do not add blank lines only to separate adjacent headings from each other. Follow existing file style, and let explicit project tooling win when a formatter or linter requires a different layout.

## Validation
No build or test tooling exists yet — the stack is not chosen. When it is, document how to validate changes here.

Until then: check that Markdown links resolve and that nothing secret is staged before committing.

## CLAUDE.md
`CLAUDE.md` is a symlink to this file. Edit `AGENTS.md`.

## Wireframe Workflows
The project adopts `wireframe-workflows/` on `project/agentic-wiki`. Its exact gitlink is authoritative. Read [design-files.json](design-files.json), [the project conventions](docs/design/wireframe-conventions.md), and the collection's `policies/` before design work. Use its `<skill-name>/SKILL.md` procedures instead of global seeds. Project planning and records still use this repository's top-level `project-workflows/`, not the workflow collection's internal contributor tooling.

Use the connected Figma capability and its tool-specific skills for this file. Ask which owner's pages a request concerns when that is unclear. Aslan's numbering and Archive migration do not authorize changes to Erwin's pages. The Component Library remains shared and requires its own explicit task. Global discovery is optional and does not override this project context.
