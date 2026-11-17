# Source import verification — 2026-11-17
This checkpoint implements the authorized S07 text-import slice: existing Meet documents through Agent Hands, plus local UTF-8 transcript and note exports. It does not implement local transcription or the full recording demo. [Spike](../active-spikes/app-artifact-intake.md) · [Setup and limits](../../app/README.md#import-meet-transcripts-or-local-exports).

## Implemented route
Settings → Sources → save a local folder or Agent Hands tool/account/Meet folder → Add artifact or Chat → choose a source → browse a meeting subfolder or paste a Google Doc link → preview the complete text → choose transcript or notes → optionally link it to the current page → confirm → read/edit/history → restart.

The app copies original text bytes and source metadata into SQLite with the first revision. Cancel creates no object. Repeated provider/source/content opens the same object and preserves human edits. Changed source text requires confirmation, creates a new revision on the same object and preserves old content and pinned citations. An intervening human revision blocks a stale import preview. Failed transactions create no orphan artifact. Source import never invokes a model or writes to the external source.

## Evidence
| Check | Result |
| --- | --- |
| Frontend and editor contract | Eight frontend tests, TypeScript checks, the production frontend build and ten valid/four invalid ProseMirror/Rust patch cases pass. |
| Rust suite | 34 behavior tests and 27 generated-type checks pass. Seven import behavior tests cover plain Unicode text/line breaks, invalid and oversized input, settings persistence, local path/symlink/format boundaries, snapshot transactions, retry/refresh/stale guards, history and pinned evidence, rollback, distinct sources with duplicate names, full Drive export, partial listing, nested folder ancestry and adapter failures. |
| Native import UI | Actual WKWebView controls and Rust/SQLite: settings save/reopen, local preview/cancel, title/type/parent choice, saved text and provenance, duplicate open, changed-source update, old revision, Chat entry, nested folder/back navigation, full fixture export over 20,000 characters, partial listing notice, failed export and restart all pass. |
| Real Meet source | Existing Agent Hands account used in place. The native app listed the configured Meet root and exported one document inside a meeting subfolder. Its complete plain text was compared with the stored document, provenance checked and the persisted document reopened after process restart. No model call, source mutation or credential copying occurred. |
| Review regression | Existing native selection/comment → @agent request boundary → inline fixture proposal → evidence → human revision → accept/reject → history/restart suite passes. This is a labeled deterministic candidate, not an additional live inference test. |

The real Meet source included notes and a transcript in separate Google Doc tabs. The export contained both; the check imported the complete record as notes, preserving the transcript section rather than treating the summary as verbatim speech. Per-tab extraction is not implemented. The native fixture journey validates the visible controls; the live source check invokes production Tauri commands and verifies persistence without recording screenshots or source text. These are separate evidence scopes.

Frontend formatting, Rust formatting, Clippy with warnings denied and the ordinary desktop debug build pass. The final native import rerun also passes after the preview styling/date cleanup. The ordinary binary was rebuilt without WebDriver instrumentation. SHA-256 checks confirm that all ten pre-existing unrelated dirty files are unchanged.

## Isolation and reproduction
Run `npm run check` and `npm run test:import:native` from `app/`. The import suite uses a private temporary data directory and port 4448, removes its test directory after stopping the app, and writes only fictional UI screenshots and outcomes under ignored `.artifacts/native-import/`. The opt-in live environment variables are documented in the app README. No private meeting titles, IDs, text, credentials or account details belong in this public record or the repository. Agent Hands is an already installed external runtime; this checkpoint does not change it.

The schema migration creates import/settings tables without rewriting existing knowledge records. Original snapshots and their revisions share one transaction. The frontend has no general shell/filesystem/SQL interface; the Rust adapter issues fixed list/meta/export commands. Export is bounded to UTF-8 text up to 1 MB. Google folder ancestry is checked before listing/export and again after export; metadata changes during export reject the result. Native test instrumentation remains debug-only and excluded from ordinary builds.

## Remaining limits
- Explicit browse/preview/import only. Natural-language “a meeting happened” discovery, watchers and background synchronization remain later work.
- No media playback, local ASR, DOCX/PDF parser or timestamp-to-audio anchors. Plain text retains source labels but does not verify speaker identity or transcript accuracy.
- Whole Google Docs export, without independent tab classification. Mixed notes/transcript documents remain together; summaries are not silently labeled transcripts.
- Local folders list direct files only, up to 500 scanned entries. Agent Hands returns one listing page; the UI discloses additional results and accepts a specific document link. `.gdoc` shortcuts need Drive export.
- Existing passage context is limited to eight excerpts and 16,000 characters per excerpt. Long linked sources can exceed that limit; the app errors instead of truncating or silently broadening retrieval. Import itself remains available.
- Provider plus source path/ID defines identity. Moving/renaming a local file creates a different source identity. A refreshed source replaces local text only after confirmation; no merge UI exists yet.
- Portable backup/restore is not built; keep external originals. Only macOS was exercised. Human review of source setup and import presentation is still open.

No Figma writes, standing-policy changes or remote publication are included. Unrelated design records and the mixed TODO index are excluded from this checkpoint.
