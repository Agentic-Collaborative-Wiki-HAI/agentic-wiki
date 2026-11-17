---
attention: live
---
# app-release-quality tasks
Opened 2026-11-17. Owner: Aslan’s agent in this foundation conversation. [Concept](app-release-quality.md).

## Current scope
M0 foundation and the authorized M1 passage-review batch under approved D1–D5. D6–D8 remain deferred. No publication, Figma edits or standing policy changes are part of this work.

## To Do
- [ ] Finish live subscription generation/cancellation verification after app sign-in.
- [ ] Record human review of the complete passage-to-decision journey.

## Ready for Human QA
- Review the passage discussion, source preview, proposed wording, accept/reject and history interactions. Start with `npm run desktop` and use fictional content. The foundation appearance has been approved; the new review interactions have not.

## Done
- [x] Establish repeatable frontend, Rust and cross-boundary checks for the foundation.
- [x] Probe native macOS test automation and exercise the compiled app.
- [x] Record exact tested behavior and remaining M1 work without claiming later milestones complete.
- [x] Promote the reviewed scratch brief into an active concept/task pair for the authorized foundation slice. Broader milestone criteria remain planned.

## Foundation result — 2026-11-17
The foundation has repeatable frontend, Rust, cross-runtime and native-window checks. Aslan approved the foundation appearance; later feature/release acceptance criteria are not complete.

## Foundation verification — 2026-11-17
Tested environment: Apple Silicon macOS 15.5, Command Line Tools, Node 24.13.0, npm 11.6.2, Rust 1.100.0. Dependency locks record the exact packages. Tauri CLI/API 2.12.5, Rust Tauri 2.12.6, Vue 3.5.45, Tiptap 3.34.0, TypeScript 5.9.3 and SQLx 0.8.6 were used. TypeScript 7 was not compatible with the installed vue-tsc entrypoint; the supported 5.9 version is pinned. Rust was installed through rustup without changing shell profiles.

| Check | Result and limit |
| --- | --- |
| `npm run check` | Three frontend behavior tests, eight Rust behavior tests and nine generated-type export tests passed; ten valid and four invalid real ProseMirror/Rust fixtures passed; type checking and frontend build passed. Generated export tests are not counted as product behavior tests. |
| Native build and `npm run test:native` | Actual WKWebView/Tauri/Rust/SQLite route checks passed. No mocked IPC replaces the backend. |
| Native navigation | Getting started ↔ recording excerpt; separate wiki/page contents controls; tabs and Chat preserve the document; selected passage appears in the rail. |
| Native editing/recovery | Cancel retains saved content; an autosaved draft survives process restart without changing the published page; saving creates revision 2 and history; another process restart retains it. |
| Native layout | Screenshots inspected at 1440×932 and 980×732 webview dimensions. Compact layout has no horizontal document overflow; References/Comments remain visible while the page scrolls. This is foundation visual QA, not full Prototype parity. |
| Codex readiness | Existing ChatGPT sign-in recognized. The probe reports inherited instructions and keeps generation disabled. No live inference, model streaming, quota exhaustion or model cancellation is claimed. Process timeout/cleanup and missing executable errors are tested. |
| Local request inspection | A synthetic request sent only to a loopback mock retained global instructions and additional tool namespaces. Per-server MCP overrides removed inherited MCP tools; disabling the project-document budget did not remove the global instruction provider. The inspected CLI was 0.169.0-alpha.11.1. |

Screenshots and machine results are under ignored `app/.artifacts/native-smoke/`. Test databases are temporary and separate from the user's application-data directory. The native WebDriver feature is off by default and has a compile-time release guard. The ordinary app exposes six specific document/draft/history commands, with no general shell, filesystem or SQL command.

The core document, proposal-context and native tests are reproducible through [app/README.md](../../app/README.md). No fixtures are substituted for a failed model response. The next integrated M1 work still includes persisted comments, citation/proposal state, review decisions, evidence previews and the history discussion view. Export/restore, real media, transcription, semantic retrieval, maintenance and shared operation remain unimplemented. D6–D8 remain deferred.

## Final preservation and build check
The ordinary desktop debug build passes with the native test feature absent; its dependency graph excludes the WebDriver plugin. Clippy with warnings denied, Rust formatting, and the pinned frontend formatter pass. All nine pre-existing unrelated dirty files retain their original hashes, and the original TODO content remains byte-for-byte intact beneath the six scoped entries. All 22 app/planning records checked have valid local links. Git HEAD and submodule pins are unchanged. No files were staged or committed, and no push or Figma write occurred. Forty-five application text files were checked for private absolute paths, credential-like prefixes and trailing whitespace, with no findings.

## Human feedback — 2026-11-17
Aslan launched the native foundation and reported that it looks good. This confirms the initial visual check; the full review journey and its edge cases remain the next batch’s QA.

## M1 review checkpoint — 2026-11-17
The [native review QA record](../design/app-review-qa-2026-11-17.md) covers the persistent discussion, evidence, human revision, acceptance/rejection, original/current history and restart checks. Four frontend tests, 26 Rust behavior tests and 20 type-export checks pass, plus ten valid/four invalid cross-runtime patch fixtures. Native tests use actual Rust/SQLite and clearly labeled candidate injection in the debug-only test build; live model access is a separate pending check. Ordinary builds exclude WebDriver and candidate injection.

Long-discussion screenshots revealed that sticking the whole rail was insufficient when its content exceeded the window. The rail now scrolls independently beneath sticky opaque tabs; the native review suite checks this populated state. A full history view displays the saved document next to its conversation. No screenshots, test databases or real connection credentials enter the repository.

## Revision-context audit
The native review suite additionally verifies pending-proposal persistence after process restart, disclosure of the draft being revised, disabled acceptance after an intervening page revision, explicit passage reselection and acceptance only after a refreshed proposal is reviewed. The Rust context test confirms that a request such as “Make that shorter” includes the actual prior draft and rejects a mismatched or obsolete version. These tests remain separate from the pending live inference check.

The follow-up native review and foundation suites pass. The request field receives focus and is brought into view when requesting another version. The test client now waits longer than its configured WebDriver script timeout and retries only the initial read-only readiness observation against the same live process. Frontend formatting, Rust formatting and the ordinary desktop debug build pass.
