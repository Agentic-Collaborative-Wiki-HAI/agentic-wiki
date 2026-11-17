# Native passage review verification — 2026-11-17
This record covers the M1 implementation checkpoint in `app/`, after the foundation was committed. It does not claim that live ChatGPT inference or the full Figma prototype has been completed.

## Implemented route
Getting started → select one paragraph’s passage → add an arbitrary comment → explicitly ask the agent using selected source excerpts → inspect a proposed replacement and its evidence → reply or revise → accept/reject → inspect the saved revision and conversation → restart.

The app owns its discussion and proposal records. Model output cannot publish a revision. Accept writes the reviewed replacement, pinned citations and human decision in one SQLite transaction. Reject records the decision without changing the page or citations. Resolving the conversation is independent of accepting its suggestion. An intervening page revision blocks acceptance and requires an explicitly refreshed candidate.

## Automated evidence
| Check | Outcome |
| --- | --- |
| Frontend editor | Four tests pass: supported JSON/formatting round trip, split block identities, rejection of unsupported pasted structures, and citation numbering by first appearance with repeated references sharing a number. |
| Rust review engine | Nine behavior tests pass: atomic acceptance, idempotent repeated acceptance, rejection and independent resolution, explicit refresh after a stale base, human revisions with original provenance, transaction rollback, pinned evidence after the source changes, invalid/dangling evidence rejection, concurrent repeated acceptance, and source selection after an empty editor paragraph. Some tests cover several related assertions. |
| Subscription boundary | Eight tests pass: exact explicit outbound context, no advertised tools, callback state/client/consent validation, endpoint restrictions, complete streamed output and cited-source validation, scope/refresh handling, owner-only storage and stable host identity, cancellation of pending sign-in, and signed identity validation including issuer/audience/expiry/nonce and tampered signatures. |
| Cross-runtime editor contract | Ten valid and four invalid ProseMirror/Rust fixtures pass. |
| Native foundation walkthrough | Real WKWebView/Tauri/SQLite checks pass for navigation, independent contents controls, tabs, empty Chat, draft recovery, Save/Cancel, history, process restart and compact layout. |
| Native review walkthrough | Real selection/comment/reply controls, explicit source picker, sign-in requirement, evidence preview and return, human proposal revision, independent Resolve/Reopen, acceptance, inline citation, Cancel after acceptance, original/current revision history, rejection, restart and compact layout pass. |
| Sticky rail | References/Comments remain visible when a long discussion is independently scrolled. The page does not jump when scrolling the rail. |

The native review test injects an explicitly labeled **“Test fixture · no model call”** candidate through a command compiled only under `native-e2e`. It exercises the real persistence and decision logic; it is not evidence of live inference. The ordinary app has no candidate-injection command or fallback response. Screenshots and machine results are ignored local artifacts under `app/.artifacts/native-review/` and `app/.artifacts/native-smoke/`; each suite uses a temporary database. Synthetic identity test fixtures contain a public test key and signatures with no usable account credentials.

## Model connection and remaining gate
The installed Codex app-server inherited global instructions and additional tools in the foundation experiment. Its generation path remains disabled. The implemented alternative is an app-owned Rust adapter for the documented [Sign in with ChatGPT flow](https://developers.openai.com/siwc/token-sharing-open-source/sign-in), using the user’s explicit browser consent and subscription entitlement. It does not read Codex credentials, global instructions, skills, connectors or API-key environment variables. Inference uses only the selected passage, request and source excerpts; requests advertise no tools and have no paid fallback.

The local route is ready for human interaction QA. Browser sign-in, this account’s entitlement/model availability, live generation and live request cancellation remain unverified until Aslan completes the app sign-in. A request failure is shown as a failure; it never becomes fixture output. Partial model output cannot be accepted or silently published.

## Limits
- One local operator; no shared identity, permission enforcement or concurrent team editing.
- An old draft is preserved if acceptance advances its base revision. It cannot overwrite the accepted page; a dedicated stale-draft recovery/discard interface remains future work.
- Passage anchors cover a selection inside one paragraph. Changed historical quotes are retained and marked detached; complete anchor migration is not implemented.
- The recording is a seeded transcript excerpt, without real media import/playback or transcription.
- Search, artifact intake, maintenance execution, portable export/restore and the rest of the prototype are later workstreams. D6–D8 remain deferred.
- No Figma writes or remote publication are part of this checkpoint. Unrelated design records are excluded from its commit scope. `TODO.md` also contains a pre-existing Showroom change and is left uncommitted.
