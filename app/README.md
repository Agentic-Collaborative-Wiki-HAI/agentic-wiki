# Agentic Wiki desktop foundation
The local application uses Tauri 2, Vue 3/TypeScript and Rust. Its design baseline is the released Figma Prototype. This is the initial implementation foundation, not the complete wiki or the two presentation demo routes.

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
- SQLite transactions enforce document validation, revision preconditions and rollback. History lists persisted revisions. It does not yet render revision diffs or discussion.

Search, page creation, artifact intake, comments, proposed edits and the remaining routes are not implemented. Their controls are disabled or explicitly describe the current limit. The recording fixture contains an excerpt, not a playable imported recording. Citation identity and anchor helpers are contract groundwork, not a finished citation-authoring interface.

## Data and boundaries
The database lives in the operating system's application-data directory for `org.agentic-wiki.desktop`, outside the repository. On macOS this is `~/Library/Application Support/org.agentic-wiki.desktop/wiki.sqlite`. The first launch creates example content; subsequent launches preserve saved revisions. This is one local operator's workspace, without shared authentication or a reviewer-permission system.

Document schema version 1 is validated in Rust before storage. Rust owns the types; `ts-rs` generates the TypeScript contracts in `src/generated/`. The prototype's screen addresses are not database identities. The initial passage operation addresses one paragraph by block ID and UTF-16 offsets with an exact-quote precondition; it preserves surrounding formatting and citation atoms. Mapping an overlapping historical anchor detaches it rather than pretending it still locates the same text. Arbitrary cross-block edits and complete anchor migration remain future work.

Portable export/restore and managed artifact import are approved directions but are not built yet. Use only disposable example content until the release/recovery work is complete. Application databases, test artifacts, dependency caches and credentials are not repository content.

## Codex feasibility
The existing Codex sign-in can be checked without a model request:
```sh
npm run probe:codex -- /absolute/path/to/codex
```
The executable argument is optional when `codex` is on PATH. The probe starts its own temporary app-server session, reads account/configuration metadata, checks inherited instruction sources, and stops the child. It does not alter global Codex configuration, read authentication files, start a model turn, or fall back to API billing. A successful probe execution can report `generationEnabled: false`; this is a readiness result, not a successful model-generation test.

With the tested Codex `0.169.0-alpha.11.1` build, ChatGPT sign-in is available but global instructions remain inherited despite an empty working directory and disabled project-document budget. A separate local mock request also retained additional tool namespaces. Therefore the D5 context boundary is not proven and model generation is disconnected. Streaming, model cancellation, quota handling and real proposal output remain unverified. The app-owned request/candidate types deliberately contain only explicit page/passage/source data and no canonical-write command.

## Verify changes
```sh
npm run check
npm run test:native
```
`check` runs frontend tests, Rust behavior/type-export tests, real ProseMirror-versus-Rust patch comparisons, TypeScript checks and the frontend production build. `test:native` compiles a debug-only WebDriver build and tests the actual native window and Rust/SQLite boundary. It uses its own temporary database and writes screenshots/results under ignored `.artifacts/native-smoke/`. Port 4446 must be free. No browser-only IPC mock substitutes for these native checks.

The native test feature is off by default and refuses release compilation. Its server binds only to loopback. `WIKI_E2E_DATA_DIR` is honored only in that feature build. After running native tests, rebuild without instrumentation before using or distributing that binary:
```sh
npm run tauri -- build --debug --no-bundle
```
No installer, signing, updater or cross-platform release is configured. [Foundation verification and remaining work](../docs/active-spikes/app-release-quality.todo.md) records the exact checked behavior. [Implementation planning](../docs/active-spikes/implementation-planning.md) links the thirteen workstreams and the deferred decisions.

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
