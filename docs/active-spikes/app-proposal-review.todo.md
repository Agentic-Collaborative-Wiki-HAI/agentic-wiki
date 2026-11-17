---
attention: live
---
# Passage discussion and review tasks
Opened 2026-11-17. Owner: Aslan’s agent in this implementation conversation. [Concept](app-proposal-review.md).

## Scope
Authorized M1 batch under D1–D5. Transcription, semantic retrieval, maintenance execution, shared operation and Figma writes remain outside this batch.

## To Do
The bounded M1 interaction uses thread mentions and inline suggestions. Human acceptance of this interaction remains open; broader review variants remain later work.

## Ready for Human QA
- Review passage selection → comment/reply with `@agent` → inline suggestion and evidence → revise/accept/reject → history in the native app. Also check Settings sign-in and returning to an unfinished reply. Automated checks are recorded in [M1 verification](../design/app-review-qa-2026-11-17.md); they do not replace human QA.

## Done
- [x] Align requests with the Prototype comment thread and show pending edits inline in the article. Keep auth/model preferences in Settings and preserve unfinished requests across that round trip.
- [x] Complete the live subscription-backed request and follow-up check after the user signs in. The native live suite also passed cancellation/retry, rejection, acceptance, history and restart.
- [x] Persist anchored comments and replies, with independent discussion resolution.
- [x] Implement proposed wording, exact evidence, human revisions and accept/reject decisions.
- [x] Show saved revisions beside their conversation and preserve evidence return paths.
- [x] Verify stale/repeated decisions and rollback in Rust; verify pending/decided restart, rejection, explicit stale refresh/reselection and Cancel after acceptance in the native app.
- [x] Include the exact prior version in revision requests and disclose it before sending; reject obsolete or mismatched prior context.
- [x] Activate the reviewed brief for the explicitly authorized review journey.

## Verification boundary
The deterministic native suite exercises Tauri/SQLite persistence with labeled test candidates. A separate opt-in native suite passes with real subscription-generated proposals and their pinned evidence. No failed request is replaced with fixture output; the live check leaves the working wiki unchanged.
