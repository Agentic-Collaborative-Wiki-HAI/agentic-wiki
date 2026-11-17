---
attention: live
status: active
opened: "2026-11-17"
---
# S06 · Subscription-backed agent runtime and context boundary
Active spike: `app-agent-runtime`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. The M1 adapter and native review integration pass both deterministic and live subscription checks; hands-on interaction review remains open. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0 feasibility; M1 live proposal; later retrieval integration. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Ask the agent for help from the app, see its progress and evidence, cancel it, and recover from account limits without losing work or starting an unexpected paid request.

## Scope
A replaceable Rust adapter using the documented app-owned Sign in with ChatGPT flow and Responses API. The app constructs explicit, tool-free requests from selected passage/source data and validates candidate output before S05 stores it. OAuth uses PKCE, state, nonce and signed identity validation; credentials stay in protected application storage outside the frontend, repository and portable exports. The installed Codex app-server remains a read-only readiness probe because its inherited instructions/tools failed D5 isolation.

The authorized work permits a bounded synthetic model request after isolation verification and user sign-in. Out of scope: credential copying, inherited shell/filesystem access, API billing fallback, multi-agent orchestration and external connectors. Account entitlement and live generation must be demonstrated; a documented protocol alone does not prove them.

## Design basis
[Ask within a passage](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89154&scaling=scale-down-width&hide-ui=1) · [Global chat opened](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-93882&scaling=scale-down-width&hide-ui=1) · [Cited answer](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-94018&scaling=scale-down-width&hide-ui=1). Ordinary navigation waits for a human prompt. Attaching a file starts the intended intake path only after the relevant user action; chat does not begin in a pre-filled halfway state.

## Dependencies and interface ownership
Completion prerequisites: No completed feature prerequisite; contracts can be drafted against fixtures. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own AgentRun, streamed status/result events, cancellation and a provider-neutral candidate/result envelope. S05 owns proposals and decisions; S09 owns retrieval; S07 owns file intake. Model-readable context is a bounded snapshot identified by object/revision IDs. Enforce authorization in Rust handlers, not prompts. App conversations are independent of provider thread IDs so provider replacement does not erase the record.

## Rough task sequence
1. Inspect the installed Codex boundary and retain its generation gate when inherited context cannot be excluded.
2. Implement the supported app-owned OAuth and inference boundary, with explicit request inspection and identity/candidate validation tests.
3. Complete sign-in in the native app; verify available models, a real synthetic suggestion, cancellation and recovery without a paid fallback.
4. Connect follow-up proposals to the persistent review journey. Retrieval and artifact orchestration remain later workstreams.

## Acceptance criteria
- A real user-requested run returns a validated candidate; the app shows the actual provider outcome rather than swapping in a fixture on failure.
- Cancel, timeout, logout, process exit and quota exhaustion leave the editor/draft intact; retry does not duplicate a proposal.
- No model response can invoke Accept, write SQL, modify local files or expand tool scope. Test this boundary rather than relying on instructions.
- Only the intended context is sent. The app remains usable for reading/editing when model access is unavailable.
- A fake provider passes the same interface tests without requiring the live service.

## Automated and integration checks
Protocol transcript tests cover interleaved stream events, malformed candidates, process crashes and duplicate notifications. The opt-in native live suite proves the exercised account/model access; documentation and CLI help alone do not. Security tests use untrusted source text that requests unrelated actions. Record what token/limit data is actually available; do not invent per-call dollar costs for a subscription.

## Human QA
Review the proposed outgoing-context disclosure, streaming/cancellation behavior and quota messaging. Confirm that prompt, retrieved evidence and generated claims remain distinguishable.

## Risks, choices and recommendation
The app-owned adapter avoids the general-purpose coding session’s ambient instructions and tools. Eight boundary tests cover context, credentials, identity and incomplete output. Live access, generation, follow-up requests and app-side cancellation/retry pass. Provider-side cancellation effects, actual quota exhaustion and expired-token refresh remain unverified against the service. No access token is copied from Codex; no API billing is configured.

## Independent work and gates
Auth/protocol feasibility uses synthetic context and can run before S02–S05 are implemented. The live product route waits for S02–S05. It does not need embeddings or transcription to start.


## Current activation
The local review journey and app-owned subscription adapter are implemented. Deterministic checks use clearly labeled test candidates, and a separate opt-in native suite verifies live inference, model availability and the complete decision/history route. See [M1 verification](../design/app-review-qa-2026-11-17.md). D6–D8 remain deferred.

Continues from: docs/active-spikes/implementation-planning.md
