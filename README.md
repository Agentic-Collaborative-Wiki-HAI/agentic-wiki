# Agentic Collaborative Wiki

A shared knowledge base where an AI agent is a participant rather than a private assistant — and where everything it proposes goes through human review before it becomes part of the record.

Term project for **INF 385T.13 Human-AI Interaction**, UT Austin iSchool, Fall 2026.

## The problem

Agentic knowledge tools are getting good, but they are almost all **personal**. They assume you know Markdown, run Obsidian or VS Code, are comfortable in a terminal, and can drive a coding agent against local files. That rules out most people.

Meanwhile, mature wikis solved something those tools never attempted: **the social layer.** Revision history. Rollback. Proposed and contested edits. Discussion attached to a change. Identifiable contributors. Ways to handle a bad edit.

Put those together and you get the question this project is actually about:

> **What happens when an AI agent becomes an actor inside a shared, governed knowledge system rather than a private assistant working for one person?**

Not "a chatbot next to a wiki." The interesting part is governance — who reviews what the agent proposes, how they see what would change and why, and how that decision enters the record.

## Who it's for

**Small academic and research teams**, to start. Collaboration around accumulated knowledge is native to how they work, they are reachable for evaluation, and it keeps us honest about scope. We are not trying to solve collaborative knowledge management in general.

## What we're building first

The smallest thing that demonstrates the idea:

> A nontechnical member of a shared workspace asks an agent to work with shared knowledge. The agent reads and searches the corpus and produces a **transparent proposed change**. Another human can understand and review that proposal before it becomes part of the shared record.

The loop:

1. Agent reads and searches the shared corpus
2. Agent creates a **proposed** edit — never a silent one
3. Humans see what would change, and why
4. Humans accept, reject, or revise
5. Only accepted changes become canonical
6. The decision enters revision history

If that works well, the idea is demonstrated. Everything else is supporting infrastructure.

## Deliberately out of scope

Real-time multi-cursor collaboration · CRDTs · multi-agent orchestration · autonomous background agents · MediaWiki feature parity · elaborate permissions · federation · enterprise RAG infrastructure · local model hosting to save a few dollars.

This is a semester project. Scope discipline is the main risk.

## Open architecture questions

Nothing here is decided. These are the things the first spikes exist to answer.

- **Extend an existing wiki, or build small from scratch?** Wiki.js is the leading candidate for the first path; MediaWiki has a richer AI/MCP ecosystem but a heavier architecture.
- **Which model is good enough?** We want the *cheapest* model that reliably does the real tasks, not the most capable one. This is a benchmark question, not an opinion.
- **What agent harness, if any?** A thin vendor-neutral tool-calling loop keeps models swappable.
- **Where does it run**, and who owns the API keys and billing?

## Design commitments

Two things we are not planning to relitigate:

- **The model stays swappable.** No architecting around a single vendor.
- **The agent proposes; humans dispose.** Agent output creates reviewable proposals. It does not silently mutate shared knowledge. This is the whole point of the project.

## Repository layout

```text
docs/
  active-spikes/   current spike work — a concept doc plus a .todo.md each
  policies/        current project guidance, numbered
  scratch/         rough thinking, not authoritative
  archive/         closed or superseded spikes
project-workflows/ adopted project workflow collection
wireframe-workflows/ adopted wireframe procedures and policies
design-files.json   design file and page-owner mapping
skills/            independent skill-authoring guidance
TODO.md            spike index and coordination map
AGENTS.md          instructions for AI agents working in this repo
```

## Getting started

Nothing to run yet — the stack is not chosen. See [TODO.md](TODO.md) for open spikes and pick one that interests you.

**If you are working with an AI agent in this repo, read [AGENTS.md](AGENTS.md) first.**

## How we work

**People pick up work rather than being assigned it.** Issues and the project board are the coordination surface; current project guidance lives in `docs/policies/`, revised directly while Git preserves its history.

A few words mean specific things here — **epic**, **spike**, **task** — and they are defined in [docs/policies/0003](docs/policies/0003-how-we-name-and-organize-work.md). The short version: an epic is an open-ended topic, a spike is a bounded chunk of work with its own docs, and a plain issue is just a piece of work someone owns.

## License

[AGPL-3.0](LICENSE). Chosen deliberately: if this becomes something people run as a service, the improvements should come back.

## Project Workflows adoption
Use the pinned `project-workflows/` collection on `project/agentic-wiki` with [AGENTS.md](AGENTS.md) as project authority. [Reconciliation record](docs/active-spikes/project-workflows-adoption.md) · [validation and remaining steps](docs/active-spikes/project-workflows-adoption.todo.md). Future updates follow the [adopted update procedure](project-workflows/update-project-workflows/SKILL.md).

The pinned workflow revisions are published. A fresh recursive clone retrieves the project collections and their dependencies. In an existing checkout, use `git submodule update --init --recursive` after checking for local work; do not reset dirty submodules or substitute remote tips for the recorded pins.

## Wireframes: start with your agent
The [Figma file](https://www.figma.com/design/fm5zX32mbG13hahShVgJwd) has individually owned Workshop/Scratchpad pages and a shared Archive. [Project conventions](docs/design/wireframe-conventions.md) and the [board lookup](docs/design/figma-board-map.json) explain the current organization. Erwin's existing work has not been migrated; adoption of the conventions on his pages is his choice.

**Paste this into your coding agent from this repository:**

```text
Set me up to work on this Agentic Collaborative Wiki repository's wireframes. Read README.md and AGENTS.md first, and inspect the Git/submodule state without overwriting any work. Initialize missing pinned submodules recursively; do not update them to remote tips or reset dirty checkouts. Read the top-level project-workflows/workflow.toml and use that collection for project records, then read design-files.json, docs/design/wireframe-conventions.md and wireframe-workflows/setup-wireframe-workflows/SKILL.md. Use the project's pinned policies and skills rather than global versions. Ask which owner's Figma pages I will work on, verify Figma access through my existing connector, and leave other owners' pages unchanged. Offer optional global links for my chosen local agent clients with wireframe-workflows/setup-wireframe-workflows/scripts/link-skills.py: preview first, preserve existing installations, and verify targets after execution. Do not automatically install service integrations, modify the shared Component Library, move design work, publish branches or push commits. Report any missing published revision or connector access instead of substituting a different version.
```

For a fresh clone:
```sh
git clone --recurse-submodules https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki.git
```

For an existing clean clone, initialize missing pinned dependencies with `git submodule update --init --recursive`; inspect dirty submodules before doing so. The parent pin selects the version. `.gitmodules` records `project/agentic-wiki` for future updates, but setup must not use `--remote` to substitute a newer revision.

Global links are optional convenience, not the source of project authority. The installer supports Codex and Claude Code and refuses conflicts. Other agents can read the same `SKILL.md` files directly. See the [adoption tasks](docs/active-spikes/wireframe-workflow-adoption.todo.md) for publication and fresh-clone verification status before sending these instructions to teammates.
