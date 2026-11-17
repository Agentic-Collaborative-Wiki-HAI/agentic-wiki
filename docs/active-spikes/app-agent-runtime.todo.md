---
attention: live
---
# app-agent-runtime tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-agent-runtime.md).

## Current scope
M0 foundation and feasibility under approved D1–D5. D6–D8 remain deferred. No publication, Figma edits or standing policy changes are part of this work.

## To Do
- [ ] Complete the app-owned browser sign-in and verify this account’s model access.
- [ ] Verify real generation, cancellation and failure/retry behavior without weakening D5 or enabling API billing.

## Ready for Human QA
- In the native app, select a passage → Comment → Ask agent → Continue with ChatGPT. The app is waiting for Aslan to complete the browser flow. Do not copy existing Codex credentials as a shortcut.

## Done
- [x] Inspect the installed Codex app-server and retain the generation gate because inherited instructions/tools remain.
- [x] Implement a provider-neutral, explicit request/candidate boundary with no model write capability.
- [x] Implement the documented app-owned OAuth and inference adapter in Rust.
- [x] Pass eight subscription boundary tests, including signed identity validation, precise outbound context, protected storage and incomplete-response rejection.
- [x] Integrate explicit requests, source selection, human review and honest failure states into the native app.

## Foundation result — 2026-11-17
The implemented request/candidate types are provider-independent, size-bounded and reject unknown fields or invented source IDs. They cannot write canonical records. The Rust readiness CLI confirms ChatGPT account type and reports one inherited instruction source. It never calls turn/start. Missing-binary and unresponsive-child cleanup tests pass. The separate loopback-only request experiment saw global instructions and functions/clock tool namespaces; an empty MCP table initially inherited configured servers, while explicit per-server disables removed their tools. No private prompt body, credentials or real model response is saved in this repository. Streaming, quota handling, generation and cancellation of an actual model turn remain unverified.

## Current verification gate
The app-owned alternative is implemented and its local boundary tests pass. The [M1 QA record](../design/app-review-qa-2026-11-17.md) distinguishes real native review checks from live model checks. Sign-in, entitlement, live inference and actual provider cancellation remain pending. No live result has been substituted with a test fixture.
