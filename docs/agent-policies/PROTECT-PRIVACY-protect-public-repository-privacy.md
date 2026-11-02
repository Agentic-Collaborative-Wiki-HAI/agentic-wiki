---
id: PROTECT-PRIVACY
type: operational
scope: "Agent handling of project content in this public repository."
status: adopted
authority: "Aslan approved D2 (the eleven-policy adoption and associated routing) in the active Agentic Wiki migration chat on 2026-11-02."
revised: "2026-11-02"
depends_on: ["RESOLVE-DEPENDENCIES"]
related_to: []
---
# Protect public repository privacy
This repo is public under AGPL-3.0. Everything committed here is visible to anyone.

- **Never commit API keys, tokens, or credentials.** Model provider keys are the specific risk on this project. Use `.env` locally and repository secrets in CI. `.gitignore` covers the common patterns, but it is a safety net, not a substitute for care.
- **Never commit classmates' personal information** — no phone numbers, home addresses, private messages, or anything from Canvas that is not already public.
- **Do not commit research participant data.** If user evaluation produces transcripts, notes, or recordings, they stay out of this repo. Consent and IRB questions are unresolved; ask before writing any participant data anywhere.
- Do not commit unreleased course material, other students' work, or anything from the instructor that was not shared publicly.
