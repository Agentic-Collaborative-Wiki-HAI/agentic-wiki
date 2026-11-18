# Sourced Chat and editable page drafts — QA
Checked 2026-11-17 on Apple Silicon macOS, using the compiled Tauri/WKWebView app and real Rust/SQLite commands. Scope: approved N4–N6. Owning records: [S09](../active-spikes/app-search-retrieval.md), [S06](../active-spikes/app-agent-runtime.todo.md), [S02](../active-spikes/app-knowledge-store.todo.md), [S03](../active-spikes/app-editor-anchors.todo.md), [S04](../active-spikes/app-evidence-citations.todo.md), [S12](../active-spikes/app-release-quality.todo.md).

## Result
The approved functional journey passes with labeled test answers and separately with real `gpt-6.2-sol` subscription output. Chat does not invoke the model on opening. A supported answer retains exact evidence; creating a draft does not publish; explicit Save publishes reviewed wording and citations with a recoverable AI-origin link. This is a local implementation checkpoint, not full Prototype acceptance or a factual-quality benchmark.

## Evidence
| Check | Observed result |
| --- | --- |
| Quiet opening and context selection | The drawer opens without a model request. Current-page and linked-source excerpts are inspectable; other saved evidence requires explicit selection. |
| Real sourced answer | A deliberate fictional booking question produced a real answer citing the supplied recording excerpt. Opening the citation displayed the saved quote and revision; no fixture replaced the result. |
| Missing evidence | A separate unsupported question returned an explicit evidence gap. No unsupported page draft was offered. This proves the observed example, not universal model abstention. |
| Editable draft | Create draft page preserved citations and AI origin. Native title/body edits autosaved; the draft remained absent from the wiki tree before Save. |
| Recovery before publication | Keep draft and close, reopening through Chat, and process restart preserved the unpublished edited draft. |
| Explicit publication | Save page created the wiki page with the reviewed text, citations, source inspection, originating-answer link and first revision. Another restart retained them. |
| Original page isolation | The question/draft route did not mutate the source article. Tests used disposable workspaces, separate from the working wiki. |
| Proposal marker regression | Native workspace verification displayed `[proposed source 1]` alongside saved `[1]` before acceptance. Existing review/accept/reject/history checks still passed. |
| Backend integrity | Tests reject unknown/absent source IDs, stale draft saves and stale publication. Cancellation/interruption rejects late answers. Forced revision-insert failure rolls back publication while retaining the draft. |
| Backup recovery | A backend integration test exported/restored completed Chat and an unpublished draft, preserved citations/origin, then published only in the restored copy. |
| Regression suites | Native workspace, passage review and foundation suites passed against the integrated code. The import suite was not rerun in this batch. |

## Commands and artifacts
From `app/`, run `npm run check`, `npm run format:check`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, and `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --features native-e2e -- -D warnings`. The check command passed 11 frontend tests, 86 Rust tests (including generated-type export tests, not 86 behavior tests), ten valid/four invalid cross-runtime fixtures, TypeScript and Vite builds.

`npm run test:chat:native` builds instrumentation and runs the fictional native journey on port 4452. An explicitly authorized `WIKI_RUN_LIVE_CHECK=1 npm run test:chat:native` uses the existing app-owned subscription connection in place, with a disposable fictional wiki. Run native suites sequentially with an unlocked desktop. The live answer and unsupported-question calls consume subscription usage; they are not part of the default check command.

Screenshots and machine outcomes remain under ignored `app/.artifacts/native-chat/`; repeated runs replace this local artifact set. Native workspace/review/foundation evidence remains in each suite's ignored directory. Private credentials, account identity, source documents and databases are not committed. The ordinary `npm run tauri -- build --debug --no-bundle` build passed after testing; test instrumentation is absent from that binary.

## Limits and remaining review
- Human assessment of answer relevance, factual support and interface fit remains necessary. Citation validation establishes identity and saved quote integrity, not entailment.
- No semantic retrieval, background discovery, external browsing, raw-audio transcription, shared publication or automatic maintenance was added.
- Conversation history is bounded; only the latest 50 exchanges are listed. An origin link can reopen an older answer. There is no multi-conversation organization or deletion UI in this slice.
- Source selection can be explicit, but retrieval remains keyword-based and excerpt-limited. Missing evidence means insufficient supplied context, not that the whole workspace lacks an answer.
- Quota exhaustion, expired-token renewal, provider-side cancellation, other platforms and account tiers were not exercised. Chat cancellation/interruption is covered by backend tests; the existing native review suite covers the shared cancellation transport.
- The separate read-only Prototype audit found layout, authoring and accessibility differences. Passing this route does not close those differences or the broader spikes.
