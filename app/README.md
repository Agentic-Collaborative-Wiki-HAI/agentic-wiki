# Agentic Wiki desktop app
The local application uses Tauri 2, Vue 3/TypeScript and Rust. Its design baseline is the released Figma Prototype. The current checkpoint implements local editing, passage review and transcript/notes import. App-owned ChatGPT sign-in and the live passage-review journey have been verified on the development Mac. The complete wiki and both presentation demo routes are not implemented.

## Run locally
Prerequisites: Node 24.13 or newer, npm, Rust installed through rustup, and the [Tauri prerequisites for your operating system](https://v2.tauri.app/start/prerequisites/). Development has been checked on Apple Silicon macOS 15.5 with Command Line Tools. The Rust toolchain is pinned in `rust-toolchain.toml`; npm and Cargo dependencies have lockfiles. Other platforms are not verified.

From this directory:
```sh
npm ci
npm run desktop
```
If rustup is installed but `cargo` is not on your PATH, load the environment script rustup supplies before running these commands. `npm run dev` alone starts only the frontend; persistence requires the Tauri application. The browser view reports that limitation rather than substituting an in-memory database.

## What works
- Getting started and an orientation-transcript excerpt are seeded as explicitly fictional example records. Switching pages, the independent wiki/page contents controls, References/Comments tabs and an empty Chat drawer work.
- The bounded editor supports paragraphs, headings, basic inline formatting, links and citation identities. Save publishes a revision; Cancel discards the draft and preserves the saved document. Unsupported pasted structures are rejected with an explanation.
- Drafts autosave after a 250 ms idle period. Wait for “Draft saved on this device” before closing if you need to recover that draft. Restarting restores the saved page and offers the saved draft when editing resumes.
- Passage comments and replies persist. Select a passage, click Comment, and add your note. Mention `@agent` in that comment or a later reply to request a suggestion; ordinary comments do not invoke the agent. Expand Context sent to the agent to inspect the passage, linked source excerpts and prior draft before sending.
- Proposed wording appears inline in the article as deleted and inserted text, while remaining separate from the saved document. The thread holds the explanation, pinned evidence and decision controls. You can reply with `@agent` for another version, edit the proposed wording inline, accept or reject; resolving the thread is a separate action. Accept publishes wording and citations together. A stale suggestion cannot be accepted until it is refreshed and reviewed.
- History shows saved page revisions alongside associated discussions and decisions. Comments, original and revised candidates, citations and decisions survive restart. References/Comments tabs remain visible while scrolling a long discussion.
- Add artifact and Chat share a source importer: browse local text exports or Meet documents through Agent Hands, preview, choose transcript versus notes, optionally link to the current page, and confirm. Sources remain editable and revisioned; unchanged imports open the existing copy and changed imports require confirmation.
- SQLite transactions enforce document validation, revision preconditions, idempotent decisions and rollback.

Search, page creation, binary media intake/playback, natural-language global agent chat and the remaining routes are not implemented. Their controls are disabled or explicitly describe the current limit. The recording fixture contains an excerpt, not a playable imported recording. Citations created by accepted suggestions work; a general manual citation-authoring interface is not built yet.

## Data and boundaries
The database lives in the operating system's application-data directory for `org.agentic-wiki.desktop`, outside the repository. On macOS this is `~/Library/Application Support/org.agentic-wiki.desktop/wiki.sqlite`. The first launch creates example content; subsequent launches preserve saved revisions. This is one local operator's workspace, without shared authentication or a reviewer-permission system.

Document schema version 1 is validated in Rust before storage. Rust owns the types; `ts-rs` generates the TypeScript contracts in `src/generated/`. The prototype's screen addresses are not database identities. The initial passage operation addresses one paragraph by block ID and UTF-16 offsets with an exact-quote precondition; it preserves surrounding formatting and citation atoms. Mapping an overlapping historical anchor detaches it rather than pretending it still locates the same text. Arbitrary cross-block edits and complete anchor migration remain future work.

Original text import bytes are stored in SQLite with their source identity, checksum and revision. Portable workspace export/restore is not built yet; keep the external originals and do not use this development app as the only copy of important work. Application databases, test artifacts, dependency caches and credentials are not repository content.

## Connect your subscription
1. Open Settings and use the ChatGPT connection section. Click Continue with ChatGPT, complete browser sign-in and choose an available model. The app uses its own registration and consent; it does not reuse or copy Codex credentials.
2. Back on Getting started, select the booking sentence, click Comment and add a note.
3. Reply in that thread with `@agent Compare this passage with the orientation transcript and suggest a revision.` You can also mention `@agent` in the initial comment. Inspect Context sent to the agent before sending if needed.
4. Review the inline replacement, explanation and source evidence. Ask for another version in the same reply composer, or use Edit suggested text to revise the proposed wording in the article. Accept or Accept edited publishes wording and citations together; Reject preserves the saved page. Cancel edits discards only the unsaved proposed wording.

If a request needs sign-in, its Settings link preserves the comment draft so you can return and send it. Cancel request stops waiting for a pending result; a completed suggestion still needs your decision.

The [documented app-owned sign-in flow](https://developers.openai.com/siwc/token-sharing-open-source/sign-in) uses ChatGPT plan access. The native live check verifies model access, generation, follow-up revision, cancellation and retry using fictional content; other accounts and plan tiers are not thereby verified. Credentials live in a separate protected `chatgpt/connection.json` beneath the app-data directory, with owner-only Unix permissions. They stay out of the frontend, repository, logs and export data. Reconnect ChatGPT is available if sign-in needs renewal. There is no API-key or paid fallback.

The Rust adapter constructs one request from the selected passage, human request, exact previous draft when revising, and bounded linked source excerpts. Context comes from that page’s pinned citations and directly attached recordings/imported sources, not an all-workspace source picker. Follow-up requests retain the pending suggestion’s pinned evidence. The thread discloses the exact context before sending; it does not silently include unrelated conversation or other files. It advertises no tools and never loads global Codex instructions, skills, connectors or ambient files. Only complete, validated model output becomes a pending suggestion. Network errors, usage limits and malformed output are shown as errors, without substitute fixtures.

The separate Codex readiness probe remains available:
```sh
npm run probe:codex -- /absolute/path/to/codex
```
The argument is optional when `codex` is on PATH. The tested Codex `0.169.0-alpha.11.1` inherited global instructions and additional tools; that generation path stays disabled. The probe checks metadata in its own temporary app-server session, never calls `turn/start`, and does not alter global configuration or read authentication files.

## Import Meet transcripts or local exports
1. Open **Settings → Sources**. For local sync, set an absolute folder path containing UTF-8 `.txt`, `.md`, `.srt` or `.vtt` exports. Only files directly inside that folder are listed; `.gdoc` files are shortcuts and need the Drive path instead.
2. For Meet, use **Find installed tool** or enter the installed Agent Hands `use-google-drive/scripts/drive` path, plus its existing Google account alias. Optionally set the Meet folder link/ID; imports will be restricted to that folder and its subfolders. Agent Hands owns the account access; the app does not copy its credentials. If account access is missing, use Agent Hands’ `setup-google-services` guidance to establish it first.
3. Save settings, return to the page, and choose **Add artifact**, or **Chat → Import a transcript or notes**. Select the source, browse into a meeting folder, or paste a specific Google Doc link. Folder lists disclose when Agent Hands returned only one page of results.
4. Inspect the complete text preview, title and content type. Choose Transcript only for spoken content; mixed notes/summary/transcript documents stay together under Notes or summary. The app exports the complete Google Doc as plain text rather than selecting individual tabs. Speaker labels and timestamps remain source text, not verified speaker identities or playable audio anchors.
5. Optionally link the source to the current page, then confirm. The Sources navigation lists the saved copy. History retains prior versions; the original Google Doc remains linked. Importing never invokes a model or writes back to Drive.

The first import creates one document and stores its exact source bytes. Repeating the same provider/source/content opens the existing copy without overwriting local edits. A changed export offers **Update imported copy**: it creates a revision, replaces the current text with the preview and preserves history and pinned older citations. A stale preview is rejected. Cancel creates no document. Sources are limited to 1 MB, with explicit errors for missing, inaccessible, unsupported or malformed inputs. Local files and Google Docs are separate source identities even when their text matches.

This is an explicit import, without background folder watching or “a meeting happened” discovery through natural-language Chat. Transcript generation, DOCX/PDF/media imports, per-tab extraction and timestamp-to-audio linking remain later work. Linked source context still has the passage-review limit of eight excerpts and 16,000 characters per excerpt; oversized contexts fail explicitly until bounded retrieval is implemented.

## Verify changes
```sh
npm run check
npm run test:native
npm run test:review:native
npm run test:import:native
```
`check` runs frontend tests, Rust behavior/type-export tests, real ProseMirror-versus-Rust patch comparisons, TypeScript checks and the frontend production build. `test:native` compiles a debug-only WebDriver build and tests the actual native window and Rust/SQLite boundary. It uses its own temporary database and writes screenshots/results under ignored `.artifacts/native-smoke/`. The review suite uses port 4447 and `.artifacts/native-review/`; the foundation suite uses port 4446. The import suite uses port 4448 and `.artifacts/native-import/`, with local exports and a labeled Agent Hands fixture. Run native suites sequentially; each port must be free. The review suite injects explicitly labeled test candidates through a debug-only command, never through the product UI or as a fallback for failed inference. No browser-only IPC mock substitutes for these native checks.

The live review suite is opt-in and consumes ChatGPT plan usage:
```sh
WIKI_RUN_LIVE_CHECK=1 npm run test:review:live
```
Complete the app-owned sign-in first and run this while the ordinary app is not making model requests. The suite uses a disposable wiki on port 4448 and the existing app-owned connection in place; it does not copy credentials or edit the working wiki. It cancels one request, retries, revises a real result, rejects it, then accepts a fresh result and checks history after restart. It records only fictional candidate output and test outcomes under ignored `.artifacts/native-live-review/`, without screenshots, account identity or credential logs. This command is not part of `npm run check` and must not be run without explicit live-verification authorization.

For an explicitly authorized live Drive check, set `WIKI_LIVE_IMPORT_DOC`, `WIKI_LIVE_IMPORT_ACCOUNT` and `WIKI_LIVE_IMPORT_TOOL` when running `test:import:native`; optionally set `WIKI_LIVE_IMPORT_FOLDER` to test ancestry restrictions. `WIKI_LIVE_IMPORT_KIND` defaults to `notes`; use `transcript` only for a transcript-only source. The live import uses the existing Agent Hands account in place, stores the document only in a private temporary database, verifies the complete text and restart, then removes that test directory. It does not call a model. Public QA records contain outcomes, not document titles, IDs or meeting content. The fixture UI journey and the live adapter/persistence check are separately identified in the [import QA record](../docs/design/app-import-qa-2026-11-17.md).

The native test feature is off by default and refuses release compilation. Its server binds only to loopback. `WIKI_E2E_DATA_DIR` and the live suite’s `WIKI_E2E_LIVE_SUBSCRIPTION=1` switch are honored only in that feature build. After running native tests, rebuild without instrumentation before using or distributing that binary:
```sh
npm run tauri -- build --debug --no-bundle
```
No installer, signing, updater or cross-platform release is configured. [Review verification and limits](../docs/design/app-review-qa-2026-11-17.md) records the exact checked behavior. [Implementation planning](../docs/active-spikes/implementation-planning.md) links the thirteen workstreams and the deferred decisions.

## Source layout
```text
src/                 Vue shell, bounded editor and Tauri command client
src/generated/       Rust-derived TypeScript contracts
src-tauri/src/       Domain validation, SQLite, readiness probe and desktop commands
src-tauri/migrations/ Versioned storage migrations
public/assets/       Icons supplied by the approved Figma baseline
scripts/             Cross-runtime contract and native-window checks
tests/               Frontend editor checks
```
Keep product capabilities behind specific Rust commands. The frontend has no general filesystem, shell or SQL command. A model candidate remains data requiring application validation and human review; it does not gain write authority by arriving through a model adapter.
