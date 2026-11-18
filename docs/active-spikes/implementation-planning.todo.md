---
attention: live
---
# Implementation planning tasks
Opened 2026-11-17. Slug: `implementation-planning`. [Concept, map and drafts](implementation-planning.md).

## Background and scope
The planning pass produced thirteen workstream briefs and an approval slate. Aslan approved D1–D5 and authorized the foundation plus editor/Rust and Codex feasibility checks on 2026-11-17. This coordinating record tracks those decisions and the child evidence. D6–D8 remain deferred. Figma changes, publication and standing-policy amendments are outside this continuation.

## Current state
D1–D5 approved on 2026-11-17. Nine workstreams are active: S01–S07, S09 and S12 cover the foundation, passage review, source imports, local authoring, search and recovery; four remain scratch drafts. D6–D8 are deferred. Existing local design edits are preserved; the published release and live Prototype metadata supply the current baseline. The full approval slate belongs in chat, not a file.

## To Do
N1–N6 implementation and bounded native/live checks are committed. Human review of the new Chat/draft interaction remains open. The approved U1–U3 interface corrections and accompanying authoring fixes are implemented and checked; [interface QA](../design/app-interface-qa-2026-11-17.md) records evidence and remaining limits. Human interaction review remains open. Transcription, semantic retrieval, automatic maintenance and shared operation remain deferred. Preserve D5's explicit context boundary.

## Ready for Human QA
- Review the passage comment → `@agent` reply → inline proposed wording and evidence → accept/reject → history journey, including Settings and returning to the unfinished request. Foundation appearance is approved. The live model check passes; D1–D5 do not need renewed approval.

## Done
- [x] Establish the first runnable foundation and record both feasibility outcomes in the child spikes. Native editing/storage/recovery checks pass. The editor/Rust contract passes ten valid and four invalid cases. The Codex readiness check confirms subscription sign-in but rejects the inherited-context boundary before model generation; M1 is not complete.
- [x] Record Aslan’s approval of D1–D5 and deferral of D6–D8; promote the five foundation/verification briefs and repair their inbound links.
- [x] Verify every requested planning deliverable against the actual generated files, including draft content, dependency ownership and route references. Checked all 15 documents, all required sections in 13 draft briefs, 28 relative links, 54 distinct linked Prototype frame IDs and the acyclic dependency graph.
- [x] Verify that unrelated dirty files and existing TODO content are unchanged, and that no app files, standing policies, submodule pins or Git history changed. SHA-256 comparison preserved all ten pre-existing file contents, except the exact additive TODO entry. HEAD and both submodule pins are unchanged; no app directory was created. git diff --check passed; new files also passed whitespace checks.
- [x] Present the spike sequence, first milestone and complete approval slate directly in chat, distinguishing now/later decisions and evidence limits. D1–D5 are approved; D6–D8 remain deferred.
- [x] Read the goal objective and establish its planning-only boundary. The user’s attached objective authorizes local planning artifacts and requires stopping for review before implementation.
- [x] Read current repository instructions, adopted policies, configuration and the selected spike/contribution/slate skills. Standing rules were not amended.
- [x] Inspect local Git/work and preserve a pre-write snapshot. Ten modified files existed; only the authorized additive TODO entry belongs to this planning scope.
- [x] Locate current release, design records and issues rather than relying on stale notes. GitHub main fda5d752 and the November 13 release establish promotion; live Prototype metadata confirms the 246 frame identities and the current Maintenance table.
- [x] Investigate primary technical sources for the document/Rust boundary, model integration, local transcription, retrieval, backup and native testing. ProseMirror reference access returned 403; its upstream mapping source was read instead. No model call, dependency install or application build occurred.
- [x] Create the coordinating conceptual/task pair and separate draft workstreams. The map includes dependencies, contract ownership, phased acceptance, later collaboration and the distinction between Getting started’s transcript example and the Booking sheet r4/r5 example.

## Verification record
Final readback and preservation checks passed on 2026-11-17. All 246 top-level live Prototype frame IDs match the published map. The new scratch briefs are ignored by the existing scratch/ rule and remain local; no Git staging, commit or push occurred. Structural Figma metadata inspection is not new playback; released playback evidence is linked in the conceptual doc. Planning decisions D1–D5 are approved; the foundation evidence and its limits are recorded in the active child records. No commit, push or external design change occurred during this continuation.

## Human feedback — 2026-11-17
Aslan launched the native foundation and reported that it looks good. This confirms the initial visual check; the full review journey and its edge cases remain the next batch’s QA.

## Review implementation checkpoint — 2026-11-17
The foundation checkpoint excludes the pre-existing design edits and mixed TODO index. The next batch adds persistent comments, source evidence, human-revised candidates, atomic accept/reject decisions and history. The native route passes with labeled test candidates. The app-owned subscription adapter passes local boundary tests; real access remains pending user sign-in. See [QA evidence and limits](../design/app-review-qa-2026-11-17.md). The native app is open for the sign-in and review check; no paid fallback, Figma write or push is authorized.

## Live review checkpoint — 2026-11-17
App-owned sign-in, live model discovery and the native passage-review journey pass. The check covers cancellation/retry, initial and revised model proposals, rejection, atomic wording/citation acceptance, history and restart in a temporary workspace. The working wiki and unrelated design edits are preserved. M1 awaits the requested hands-on interaction review; M2 and the deferred decisions remain outside this implementation batch.

## Interaction alignment — 2026-11-17
The user’s hands-on check confirmed the live mechanics but requested Prototype-aligned interaction. The app uses `@agent` in normal passage comments/replies, inline proposed wording, and Settings for the ChatGPT connection/model. S05 and S06 retain the implementation detail; the QA record distinguishes automated verification from the remaining human review. No later milestone is activated.

## Source-import checkpoint — 2026-11-17
The approved S07 slice imports existing Meet documents through Agent Hands and UTF-8 exports through a local source-folder setting. The [active intake spike](app-artifact-intake.md) owns implementation and the [import QA record](../design/app-import-qa-2026-11-17.md) separates fixture UI checks from real provider verification. Chat and Add artifact share explicit preview/confirmation. Import does not invoke a model, ingest the private vault, or activate the deferred transcription and retrieval decisions. The mixed TODO index remains untouched.

## Approved Chat and fidelity continuation — 2026-11-17
- [x] N4: distinguish proposed source markers from the article's existing citation numbering (S04).
- [x] N5: implement sourced global Chat with exact evidence inspection, missing-evidence answers, explicit context selection and no request on opening Chat (S06/S09).
- [x] N6: save an answer as an editable draft with citations and AI provenance; require explicit Save to publish, and verify restart persistence (S02/S03/S04/S09).
- [x] Audit implemented routes against live released Prototype: rendered layout, interactions, state/return paths and built-product accessibility; separate missing capabilities, implementation mismatches and decisions. Record evidence and unreached states (S01/S12).

The mixed root TODO index and the nine unrelated design files remain outside this continuation. Detailed child records and this coordinating overview carry the app's current status.

## Chat and fidelity checkpoint — 2026-11-17
N4 is committed as `0fe2fa9`; sourced Chat and editable unpublished drafts are committed as `f2519f4`. The [Chat QA record](../design/app-chat-qa-2026-11-17.md) covers fixture/live answers, missing evidence, draft recovery, publication, citations and history after restart. The [Prototype audit](../design/app-prototype-audit-2026-11-17.md) records twelve implementation/accessibility findings, deliberate local-app differences and unimplemented routes. These findings are proposals for the next batch, not silently approved redesigns. Human QA of Chat/draft usefulness and presentation remains open.

The normal desktop build without test instrumentation is reopened for the user. The root index and nine unrelated design records remain excluded; no push or Figma mutation occurred. This checkpoint does not close the broader child spikes.

## Approved interface correction batch — 2026-11-17
U1–U3 and the accompanying interaction corrections are approved. Work stays in the app, with local commits and no Figma edits or push.
- [x] S01/S12: persistent breadcrumbs, responsive shell/Chat, scalable typography, keyboard citations/tabs and named dialogs.
- [x] S04/S05: real margin notes, passage highlights for comments, passage-aligned reference/discussion cards, no thread-switching tabs, and hide resolved threads from the active rail.
- [x] S03/S04: edit titles within page editing; author citations and margin notes in both read/edit states; preserve Save/Cancel and draft recovery. Remove Rename / move UI; defer moving.
- [x] S05: make Page history a page with associated discussion and clear return context.
- [x] S06/S09: support editing instructions without mandatory external sources, keep revision evidence in the discussion rather than creating article references, and use message-oriented Chat copy with an inspectable Context disclosure.
- [x] S06: expose editable application task instructions in Settings, retaining explicit human publication and bounded-source/tool enforcement in code.
- [x] S12: native functional, accessibility, persistence and visual checks; update the owning records and commit only this scope.

The [interface QA record](../design/app-interface-qa-2026-11-17.md) separates fixture, live-subscription and native keyboard/visual evidence. Remaining history comparison, media, search and sharing work stays in its owning spikes. This checkpoint does not close them.
