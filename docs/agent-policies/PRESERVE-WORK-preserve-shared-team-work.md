---
id: PRESERVE-WORK
type: operational
scope: "Agent changes and Git operations in this shared checkout."
status: adopted
authority: "Aslan approved D2 (the eleven-policy adoption and associated routing) in the active Agentic Wiki migration chat on 2026-11-02."
revised: "2026-11-02"
depends_on: ["RESOLVE-DEPENDENCIES"]
related_to: []
---
# Preserve shared team work
- **Do not force-push shared branches.** Do not rewrite history that others may have pulled.
- **Do not commit on someone's behalf** or amend their commits.
- If the worktree is dirty with changes you did not make, **stop and ask.** Someone else may be mid-task.
- Commit by explicit pathspec. Never `git add -A` in a shared repo — see `project-workflows/commit-work/`.
- **Do not add `Co-Authored-By:` trailers.**
