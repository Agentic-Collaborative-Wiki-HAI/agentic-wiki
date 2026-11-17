---
attention: live
---
# app-agent-runtime tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-agent-runtime.md).

## Current scope
M0 foundation and feasibility under approved D1–D5. D6–D8 remain deferred. No publication, Figma edits or standing policy changes are part of this work.

## To Do
- [ ] Inspect the installed Codex app-server schema and establish an isolated, tool-disabled request configuration.
- [ ] Verify subscription-backed generation, cancellation and failure behavior; record any unavailable capability without weakening D5.

The configuration is not proven isolated. Continue by testing a supported app-owned session/auth boundary or a supported isolated configuration. Do not copy auth tokens, change the user’s global instructions, silently enable inherited tools, or switch to billed API access. This does not require revisiting the approved D5 boundary.

## Ready for Human QA
No separate manual check requested by this record; app-level visual QA is tracked in S01/S12.

## Done
- [x] Implement a provider-neutral request/result boundary with a bounded synthetic context and no canonical write command.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
The implemented request/candidate types are provider-independent, size-bounded and reject unknown fields or invented source IDs. They cannot write canonical records. The Rust readiness CLI confirms ChatGPT account type and reports one inherited instruction source. It never calls turn/start. Missing-binary and unresponsive-child cleanup tests pass. The separate loopback-only request experiment saw global instructions and functions/clock tool namespaces; an empty MCP table initially inherited configured servers, while explicit per-server disables removed their tools. No private prompt body, credentials or real model response is saved in this repository. Streaming, quota handling, generation and cancellation of an actual model turn remain unverified.

## Next bounded investigation
Evaluate an app-owned Sign in with ChatGPT transport, if supported for this account/app, so the Rust adapter constructs an explicit inference request without loading a general-purpose coding session's global instructions or tool catalog. Retain the current provider-neutral request/candidate boundary. This is a proposed alternative, not a verified entitlement or implemented integration; it must pass outgoing-context inspection and real generation/cancellation checks before M1 uses it. A supported isolated app-server mode remains an alternative if it can pass the same checks. Neither path authorizes credential copying, API billing or relaxing the approved context boundary.
