---
attention: live
status: active
opened: "2026-11-17"
---
# S12 · Continuous verification, recovery and desktop release
Active spike: `app-release-quality`. Aslan authorized foundation work and feasibility checks on 2026-11-17 after approving D1–D5. Later milestone scope remains planned, not implemented. [Build map, evidence and decisions](implementation-planning.md). Intended delivery: M0 harness; checks at every milestone; M5 release. Owner: Aslan’s agent in the current foundation conversation. Contract ownership below belongs to this workstream; no teammate is assigned.

## Outcome
Use a dependable local app whose behavior survives failures, can be backed up/restored, and has honest evidence for the routes claimed to work.

## Scope
Own the integration harness, public-safe fixtures, route/coverage ledger, accessibility and visual checks, recovery scenarios, full-fidelity export/restore, readable document export, migration release checks and a local macOS package. Feature owners remain responsible for their own invariants; this spike owns combined behavior and release evidence.

Out of scope: No last-minute testing phase that excuses earlier defects, private source data in CI, production telemetry by default, public distribution/signing spend, or claims of Windows/Linux support from a successful Mac build.

## Design basis
[Prototype launcher](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-85795&scaling=scale-down-width&hide-ui=1) and the route table in the coordinating spike. [#39](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/39) identifies unfinished failure-state design; [#43](https://github.com/Agentic-Collaborative-Wiki-HAI/agentic-wiki/issues/43) is a concrete regression. Preserve content on returns and cancellation instead of reproducing scripted-state mistakes.

## Dependencies and interface ownership
Completion prerequisites: S01, S02. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own cross-feature acceptance fixtures, the export manifest contract with S02/S07, reproducible verification reports and release checklist. S01 supplies the test bridge; S02 supplies consistent snapshots; S07 supplies blob checksums; all feature owners provide failure states and test hooks. Exclude credentials, provider session tokens and private debug traces from portable exports.

## Rough task sequence
1. Establish Rust/unit and UI/contract test execution at M0; probe the currently documented WDIO embedded Tauri driver on macOS in a test-only build.
2. At each milestone, run deterministic tests and a real desktop walkthrough with explicit fixture/live distinctions; record visual, behavioral and human QA separately.
3. Implement crash/disk-full/restart recovery and a portable snapshot-plus-media manifest; restore into a fresh application directory.
4. Package the Mac build, verify runtime dependencies and disabled test capabilities, and document startup, backup, known limits and the exact tested release.

## Acceptance criteria
- Both main journeys work in the actual Tauri app, including evidence returns, acceptance/rejection and restart; browser mocks alone are insufficient.
- Backup/restore preserves object/revision/anchor identity and attachments, excludes secrets, and detects missing/corrupt members.
- Application upgrade failure leaves a recoverable workspace; closing during a job has a defined resume/cancel outcome.
- Keyboard, focus, text scaling and VoiceOver are checked on the actual platform for the critical routes.
- The distributable build does not expose embedded test drivers or broad debug capabilities.

## Automated and integration checks
Use current Tauri/WebdriverIO documentation rather than assuming macOS is unsupported. Validate the embedded route experimentally; fall back to explicitly recorded native manual checks if unavailable. Test exports using a consistent SQLite snapshot and immutable media copies, not a blind copy of a live database/WAL. Expand integration coverage when new features land.

## Human QA
Aslan tests the first milestone and subsequent releases before feature expansion is declared accepted. Any future teammates test on their own supported environments before those platforms are advertised.

## Risks, choices and recommendation
The embedded macOS WebDriver path passes native foundation, deterministic review and opt-in live review checks. Accessibility and native media behavior still require real-device checks. Recommend a local unsigned/dev build first and a deliberate later decision on signed distribution and automatic updates.

## Independent work and gates
Harness and failure catalog start at M0. A milestone is not blocked by unbuilt future features; release claims are explicitly scoped. Full backup/media tests wait for S07, and final integration waits for the features included in the chosen release.


## Current activation
The M0 foundation and bounded M1 review implementation are checked in the companion task records and [native review QA](../design/app-review-qa-2026-11-17.md). The full passage-to-decision route passes deterministic and live subscription checks. The Codex coding-session path remains disabled; product inference uses the isolated app-owned connection. Hands-on M1 review remains open. Broader milestones and D6–D8 remain deferred.

Continues from: docs/active-spikes/implementation-planning.md

## Current interface and authoring boundary
The interface correction batch covers explicit note/citation authoring, source-optional review, title drafts/history, open discussion cards, custom task instructions, responsive shell and keyboard controls. The dedicated authoring suite uses a disposable database and labeled candidates; it checks Cancel/Save, restart, source-free acceptance, resolved-thread hiding and Chat persistence. Broader Prototype parity, VoiceOver, media timing and cross-platform behavior are not inferred from these checks.

See [interface QA](../design/app-interface-qa-2026-11-17.md) for checked cases and remaining limits.
