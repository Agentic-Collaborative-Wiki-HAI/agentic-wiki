---
attention: live
status: active
opened: "2026-11-17"
---
# S05 · Suggestions, discussion, decisions and history
Proposed spike: `app-proposal-review`. Activated for the authorized M1 implementation batch on 2026-11-17; it does not amend standing policies. [Build map, evidence and decisions](../active-spikes/implementation-planning.md). Intended delivery: M1 first integrated milestone; M3 full review variants. Implementation owner: Aslan’s agent in the current build conversation.

## Outcome
Discuss a passage, request agent help, inspect the proposed text and evidence, decide what becomes canonical, and inspect or undo the resulting revision.

## Scope
Own the proposal lifecycle, inline diff, reply/request-revision/edit-proposal controls, Accept/Reject, independent Resolve thread, history with associated conversation and revert-as-new-revision. Support human tracked proposals using the same decision mechanism. Integrate S06 after deterministic fixture behavior is proven; both the Getting started and source-driven booking examples use the same engine.

Out of scope: No acceptance from merely resolving a thread, agent-controlled canonical writes, automatic stale patch rebasing, or a history view that loses the surrounding conversation. A seeded response is not a live-agent success claim.

## Design basis
[Comment posted](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89035&scaling=scale-down-width&hide-ui=1) → [Explicit agent request](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89154&scaling=scale-down-width&hide-ui=1) → [Proposal](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-89285&scaling=scale-down-width&hide-ui=1) → [History](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-98958&scaling=scale-down-width&hide-ui=1). A separate [Using the microscope booking proposal](https://www.figma.com/proto/fm5zX32mbG13hahShVgJwd?node-id=2621-100896&scaling=scale-down-width&hide-ui=1) updates the cited Booking sheet r4/r5 requirement; do not conflate this with the Getting started transcript/calendar example.

## Dependencies and interface ownership
Completion prerequisites: S01, S02, S03, S04. These are completion dependencies, not a ban on early contract/design work. Read the coordinating map for the smaller M0/M1 handoffs.

Own proposal states (pending, accepted, rejected, superseded/stale), independent thread state, candidate patch/evidence validation, decision command and decision-history queries. S02 atomically commits document/citation/review-event changes; S03 supplies normalized operations and anchors; S04 resolves evidence; S06 may submit candidates, never decisions. S11 supplies eligibility checks at the same domain boundary.

## Rough task sequence
1. Specify state transitions and build both fictional booking cases, including rejected and manually revised alternatives.
2. Implement the deterministic first route and transaction/error cases; expose the same proposal ID in page, history and later Maintenance.
3. Connect a live S06 candidate, allow a narrower revision and human editing, and record generation/decision attribution separately.
4. Add safe revert, wrong-source rejection, resolved-pending discussion, and fuller tracked-human-edit variants.

## Acceptance criteria
- Accept publishes the exact reviewed wording and evidence references together; Reject leaves both unchanged and records the decision without a content revision.
- Reply, revise, reject, accept and resolve are separate actions. A resolved thread can still contain a pending proposal.
- Acceptance is idempotent and checks the current base revision, evidence identities and actor eligibility. A stale proposal remains inspectable and cannot overwrite intervening work.
- History retains the request, proposed alternatives, evidence and final decision. Revert creates a new revision and cannot silently erase later edits.
- Both explicit-request and source-change cases pass, including a deliberately irrelevant centrifuge source.

## Automated and integration checks
Rust transition/transaction tests cover repeated decisions, stale document/evidence, unauthorized actor and failures between writes. UI and desktop routes cover evidence Close, request revision, edit proposal, accept, reject, resolve, history return and Cancel after acceptance.

## Human QA
Aslan pressure-tests the full first milestone and distinguishes a rejected proposal, resolved discussion and changed document. Verify that the evidence actually supports the example; schema checks do not decide that.

## Risks, choices and recommendation
Recommend blocking acceptance whenever the proposal’s document base is stale, even after an unrelated edit, until an explicit refreshed proposal is reviewed. This trades occasional extra review for an auditable initial implementation. Non-overlapping rebase can be a later extension, not a hidden convenience.

## Independent work and gates
The complete deterministic route can be built before live model access. Live end-to-end acceptance requires S06. Full-text/vector search, recording import and Maintenance are not prerequisites for the seeded evidence route.


Continues from: docs/active-spikes/implementation-planning.md
