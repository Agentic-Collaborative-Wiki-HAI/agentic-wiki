---
attention: live
---
# Passage discussion and review tasks
Opened 2026-11-17. Owner: Aslan’s agent in this implementation conversation. [Concept](app-proposal-review.md).

## Scope
Authorized M1 batch under D1–D5. Transcription, semantic retrieval, maintenance execution, shared operation and Figma writes remain outside this batch.

## To Do
Complete the live subscription-backed request and follow-up check after the user signs in.

## Ready for Human QA
- Review passage selection, the discussion and evidence presentation, and the accept/reject/history journey in the native app. Automated checks are recorded in [M1 verification](../design/app-review-qa-2026-11-17.md); they do not replace human QA.

## Done
- [x] Persist anchored comments and replies, with independent discussion resolution.
- [x] Implement proposed wording, exact evidence, human revisions and accept/reject decisions.
- [x] Show saved revisions beside their conversation and preserve evidence return paths.
- [x] Verify stale/repeated decisions and rollback in Rust; verify pending/decided restart, rejection, explicit stale refresh/reselection and Cancel after acceptance in the native app.
- [x] Include the exact prior version in revision requests and disclose it before sending; reject obsolete or mismatched prior context.
- [x] Activate the reviewed brief for the explicitly authorized review journey.

## Verification boundary
The native suite exercises actual Tauri/SQLite persistence with an explicitly labeled test candidate. Live model generation remains gated on the app-owned sign-in; no failed request is replaced with fixture output.
