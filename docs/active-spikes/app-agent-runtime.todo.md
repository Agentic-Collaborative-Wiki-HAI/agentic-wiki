---
attention: live
---
# app-agent-runtime tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-agent-runtime.md).

## Current scope
The bounded subscription-backed passage-review runtime and approved N2 source-selection integration. Linked sources are the default; other saved sources require explicit selection. No ambient file access, additional provider or publication is enabled.

## To Do
The implemented M1 runtime has passed its bounded live check. Expiry/refresh, quota-exhaustion and service-outage cases remain release-hardening work; they were not provoked against the real account.

## Ready for Human QA
- Review the live suggestion, its disclosed source context, follow-up revision and accept/reject interaction. Browser sign-in is complete.

## Done
- [x] Complete the app-owned browser sign-in and verify this account’s model access. Aslan completed sign-in; the live catalog and native run passed.
- [x] Verify real generation, cancellation and failure/retry behavior without weakening D5 or enabling API billing. Cancellation/retry, initial/follow-up suggestions and native decisions passed; malformed/incomplete-response and context boundaries remain covered by deterministic tests. Live quota exhaustion and expired-token refresh were not forced.
- [x] Inspect the installed Codex app-server and retain the generation gate because inherited instructions/tools remain.
- [x] Implement a provider-neutral, explicit request/candidate boundary with no model write capability.
- [x] Implement the documented app-owned OAuth and inference adapter in Rust.
- [x] Pass eight subscription boundary tests, including signed identity validation, precise outbound context, protected storage and incomplete-response rejection.
- [x] Integrate `@agent` thread requests, inspectable linked source context, inline human review and honest failure states into the native app. ChatGPT connection and model preference live in Settings.

## Foundation result — 2026-11-17
The implemented request/candidate types are provider-independent, size-bounded and reject unknown fields or invented source IDs. They cannot write canonical records. The Rust readiness CLI confirms ChatGPT account type and reports one inherited instruction source. It never calls turn/start. Missing-binary and unresponsive-child cleanup tests pass. The separate loopback-only request experiment saw global instructions and functions/clock tool namespaces; an empty MCP table initially inherited configured servers, while explicit per-server disables removed their tools. No private prompt body, credentials or real model response is saved in this repository. Streaming, quota handling, generation and cancellation of an actual model turn remain unverified.

## Current verification
The app-owned connection passed a real native review loop with `gpt-6.2-sol` using `@agent` comments/replies and inline proposals. The [M1 QA record](../design/app-review-qa-2026-11-17.md) distinguishes deterministic fixtures from actual generation and records cancellation limits. The user’s working wiki was unchanged, and no fixture replaced a live result. Hands-on interaction review remains open.

## Approved N2 source context — 2026-11-17
Requests use bounded excerpts from linked sources by default. The thread’s context disclosure offers Choose sources, with other workspace pages available only through explicit opt-in. Pending-suggestion follow-ups preserve their pinned evidence unless the reviewer changes it. The Rust adapter still accepts only explicit revision-bound selections, advertises no tools and rejects oversized or malformed requests. No provider, external search or standing policy changed.
