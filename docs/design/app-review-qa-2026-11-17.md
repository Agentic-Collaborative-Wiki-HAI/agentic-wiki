# Native passage review verification — 2026-11-17
This record covers the M1 implementation checkpoint in `app/`, after the foundation was committed. It covers deterministic native checks and a separate live ChatGPT run. It does not claim that the full Figma prototype is implemented.

## Implemented route
Getting started → select one paragraph’s passage → add an arbitrary comment → explicitly ask the agent using selected source excerpts → inspect a proposed replacement and its evidence → reply or revise → accept/reject → inspect the saved revision and conversation → restart.

The app owns its discussion and proposal records. Model output cannot publish a revision. Accept writes the reviewed replacement, pinned citations and human decision in one SQLite transaction. Reject records the decision without changing the page or citations. Resolving the conversation is independent of accepting its suggestion. An intervening page revision blocks acceptance and requires an explicitly refreshed candidate.

## Automated evidence
| Check | Outcome |
| --- | --- |
| Frontend editor | Four tests pass: supported JSON/formatting round trip, split block identities, rejection of unsupported pasted structures, and citation numbering by first appearance with repeated references sharing a number. |
| Rust review engine | Ten behavior tests pass: atomic acceptance, idempotent repeated acceptance, rejection and independent resolution, explicit refresh after a stale base, human revisions with original provenance, transaction rollback, pinned evidence after the source changes, invalid/dangling evidence rejection, concurrent repeated acceptance, source selection after an empty editor paragraph, and exact prior-proposal context with obsolete/tampered versions rejected. Some tests cover several related assertions. |
| Subscription boundary | Eight tests pass: exact explicit outbound context, no advertised tools, callback state/client/consent validation, endpoint restrictions, complete streamed output and cited-source validation, scope/refresh handling, owner-only storage and stable host identity, cancellation of pending sign-in, and signed identity validation including issuer/audience/expiry/nonce and tampered signatures. |
| Cross-runtime editor contract | Ten valid and four invalid ProseMirror/Rust fixtures pass. |
| Native foundation walkthrough | Real WKWebView/Tauri/SQLite checks pass for navigation, independent contents controls, tabs, empty Chat, draft recovery, Save/Cancel, history, process restart and compact layout. |
| Native review walkthrough | Real selection/comment/reply controls, explicit source picker, sign-in requirement, evidence preview and return, human proposal revision, independent Resolve/Reopen, acceptance, inline citation, Cancel after acceptance, original/current revision history, rejection, pending/decided-proposal restart, stale acceptance blocking with explicit refresh/reselection, prior-version disclosure with the request field brought into view, and compact layout pass. |
| Sticky rail | References/Comments remain visible when a long discussion is independently scrolled. The page does not jump when scrolling the rail. |

The native review test injects an explicitly labeled **“Test fixture · no model call”** candidate through a command compiled only under `native-e2e`. It exercises the real persistence and decision logic; it is not evidence of live inference. The ordinary app has no candidate-injection command or fallback response. Screenshots and machine results are ignored local artifacts under `app/.artifacts/native-review/` and `app/.artifacts/native-smoke/`; each suite uses a temporary database. Synthetic identity test fixtures contain a public test key and signatures with no usable account credentials.

## Model connection and live verification
The installed Codex app-server inherited global instructions and additional tools in the foundation experiment. Its generation path remains disabled. The implemented alternative is an app-owned Rust adapter for the documented [Sign in with ChatGPT flow](https://developers.openai.com/siwc/token-sharing-open-source/sign-in), using the user’s explicit browser consent and subscription entitlement. It does not read Codex credentials, global instructions, skills, connectors or API-key environment variables. Inference uses only the selected passage, request, exact prior draft when revising, and chosen source excerpts; requests advertise no tools and have no paid fallback.

Aslan completed the app-owned browser sign-in. The separate opt-in native live suite passed with `gpt-6.1-sol`: live model discovery, cancellation with no saved proposal, successful retry, a source-grounded suggestion, follow-up revision of the disclosed draft, rejection without a page revision, acceptance with the citation, history and process restart. This test uses real native controls and the production Rust adapter; it does not invoke candidate injection. The working wiki’s table contents were unchanged across the successful live run.

The initial result directed readers to the guide and booking calendar linked from the equipment page. Its follow-up shortened that to checking the booking calendar on the equipment page. Both cited the selected recording excerpt at revision 1. The final accepted result preserved the same supported relationship. These fictional outputs were inspected for source support; this is one narrow example, not a broad model-quality evaluation.

A request failure is shown as a failure; it never becomes fixture output. Partial model output cannot be accepted or silently published. Cancellation was verified at the app boundary: no candidate was saved and retry worked. The test does not establish whether provider-side computation stops immediately or how cancellation affects plan usage. Live quota exhaustion, token refresh after expiry and service outages were not forced. The local route remains ready for the requested hands-on interaction review.

## Limits
- One local operator; no shared identity, permission enforcement or concurrent team editing.
- An old draft is preserved if acceptance advances its base revision. It cannot overwrite the accepted page; a dedicated stale-draft recovery/discard interface remains future work.
- Passage anchors cover a selection inside one paragraph. Changed historical quotes are retained and marked detached; complete anchor migration is not implemented.
- The recording is a seeded transcript excerpt, without real media import/playback or transcription.
- Search, artifact intake, maintenance execution, portable export/restore and the rest of the prototype are later workstreams. D6–D8 remain deferred.
- No Figma writes or remote publication are part of this checkpoint. Unrelated design records are excluded from its commit scope. `TODO.md` also contains a pre-existing Showroom change and is left uncommitted.

## Follow-up context audit
A revision request includes the exact pending proposal’s wording and explanation, disclosed in the request panel. The backend rejects a different prior ID or changed prior text before saving, and rejects an obsolete parent before starting inference. A previous proposal is a draft to revise, not evidence; the new result may cite only the source excerpts selected for the current request. Pinned prior excerpts remain visible choices if the source has since advanced.

## Native test startup
The WebDriver request timeout is longer than the driver’s script timeout. Initial page readiness may retry a timed-out read against the same still-running process within a bounded window; interaction commands are not replayed automatically. Both native suites pass after this change, and the ordinary desktop build is restored with the test feature absent.

## Reproduction and data isolation
Run `WIKI_RUN_LIVE_CHECK=1 npm run test:review:live` from `app/` only when live subscription verification is authorized. The native test build uses a temporary wiki and the signed-in app’s protected connection in place. It does not export credentials, capture account screenshots or copy working page content into the test. Test results and fictional model wording are ignored local artifacts. Ordinary builds ignore both test environment switches and have no WebDriver or candidate-injection command.
