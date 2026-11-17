---
attention: live
status: active
opened: "2026-11-17"
---
# S06 · Subscription-backed agent runtime and context boundary
Active spike: `app-agent-runtime`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. Later milestone scope remains planned, not implemented. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0 feasibility; M1 live proposal; later retrieval integration. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Ask the agent for help from the app, see its progress and evidence, cancel it, and recover from account limits without losing work or starting an unexpected paid request.

## Scope
A replaceable Rust adapter to a supported local Codex app-server process, with version/capability checks, explicit app session mapping, request IDs, streaming, cancellation, rate-limit state and validated candidate responses. Use a small application tool surface for permitted reads/search and proposal submission. Keep credentials and Codex runtime state outside the frontend, repository and portable exports.

The authorized feasibility work permits a bounded synthetic model request only after D5 isolation is verified. Out of scope: no scraping subscription cookies/tokens, inherited unrestricted shell/filesystem access, silent API billing fallback, multi-agent orchestration, external connector activation, or assumption that this chat’s tools/permissions become the product’s.

## Design basis
[Ask within a passage](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89154&scaling=scale-down-width&hide-ui=1) · [Global chat opened](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-93882&scaling=scale-down-width&hide-ui=1) · [Cited answer](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-94018&scaling=scale-down-width&hide-ui=1). Ordinary navigation waits for a human prompt. Attaching a file starts the intended intake path only after the relevant user action; chat does not begin in a pre-filled halfway state.

## Dependencies and interface ownership
Completion prerequisites: No completed feature prerequisite; contracts can be drafted against fixtures. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own AgentRun, streamed status/result events, cancellation and a provider-neutral candidate/result envelope. S05 owns proposals and decisions; S09 owns retrieval; S07 owns file intake. Model-readable context is a bounded snapshot identified by object/revision IDs. Enforce authorization in Rust handlers, not prompts. App conversations are independent of provider thread IDs so provider replacement does not erase the record.

## Rough task sequence
1. Read the installed protocol schema and prove supported auth/capability configuration without assuming current documentation matches the binary.
2. Demonstrate in a loopback request inspector that ambient instructions and unrelated tools are absent; current readiness is blocked here. Account metadata alone is insufficient.
3. Once the boundary passes, run a tiny synthetic request through supported subscription access; test streaming, cancellation and unavailable-account behavior. Compare schema-constrained output with scoped tools before selecting either.
4. Connect proposal generation, follow-up discussion and context disclosure; add S09 retrieval and file-intake orchestration later.

## Acceptance criteria
- A real user-requested run returns a validated candidate; the app shows the actual provider outcome rather than swapping in a fixture on failure.
- Cancel, timeout, logout, process exit and quota exhaustion leave the editor/draft intact; retry does not duplicate a proposal.
- No model response can invoke Accept, write SQL, modify local files or expand tool scope. Test this boundary rather than relying on instructions.
- Only the intended context is sent. The app remains usable for reading/editing when model access is unavailable.
- A fake provider passes the same interface tests without requiring the live service.

## Automated and integration checks
Protocol transcript tests cover interleaved stream events, malformed candidates, process crashes and duplicate notifications. A later live smoke test proves account access; documentation and CLI help alone do not. Security tests use untrusted source text that requests unrelated actions. Record what token/limit data is actually available; do not invent per-call dollar costs for a subscription.

## Human QA
Review the proposed outgoing-context disclosure, streaming/cancellation behavior and quota messaging. Confirm that prompt, retrieved evidence and generated claims remain distinguishable.

## Risks, choices and recommendation
The read-only probe confirms ChatGPT sign-in on the tested installation. The model-generation path is gated: global instructions and additional tool namespaces survived the attempted isolation settings, so no real inference was attempted. Dynamic tools are documented as experimental. Recommend a minimal constrained request first and no ambient inherited workspace tools; if that boundary cannot be verified, stop this integration rather than broaden access. App-owned Sign in with ChatGPT is an alternative for later teammate distribution; direct API billing is not preapproved.

## Independent work and gates
Auth/protocol feasibility uses synthetic context and can run before S02–S05 are implemented. The live product route waits for S02–S05. It does not need embeddings or transcription to start.


## Current activation
The first M0 foundation/check slice is implemented and documented in the companion task record. The native shell, persistence and bounded editor contract have passing evidence; Codex generation remains disabled because isolation was not proven. M1 and later acceptance criteria remain future scope. D6–D8 are deferred.

Continues from: docs/active-spikes/implementation-planning.md
