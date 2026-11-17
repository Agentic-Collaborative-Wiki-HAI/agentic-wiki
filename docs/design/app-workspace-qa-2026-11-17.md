# Local workspace authoring and recovery QA
Date: 2026-11-17. Scope: Aslan’s approved N1–N3 batch, with linked source evidence by default and explicit selection of other saved sources. This is local app work; it does not change the Figma file, activate shared operation or publish a release.

## Implemented behavior
| Area | Behavior |
| --- | --- |
| Saved wiki tree | Create, rename and move pages; reject self/descendant moves and stale metadata writes. Record identity and content revisions stay stable through moves. Imported sources appear in the same tree. |
| Keyword search | Rebuildable SQLite FTS5 over current saved paragraphs; title and breadcrumb context, source revision and exact UTF-16 excerpt offsets. Drafts are excluded. Search opens the matching passage and retains its query for return. |
| Citation authoring | Select article text → Add citation → inspect a saved excerpt → confirm. The citation and new page revision commit together, with pinned source revision and range. References and collapsible endnotes open the exact evidence; endnotes can return to the marker. |
| Agent context | Deduplicated citations and relevant directly linked source excerpts by default, up to six excerpts/12 KB of source text. Choose sources explicitly adds/removes excerpts from other saved records, within the existing eight-excerpt/32 KB total request boundary. Pending follow-ups retain pinned evidence unless deliberately changed. |
| Backup | Consistent SQLite snapshot containing pages, originals, citations, conversations, proposals, decisions, saved document drafts, metadata events and history. Credentials, connection settings and disposable search caches are excluded. |
| Restore | Validate checksum, exact schema/migrations, foreign keys, documents, citation quotes, source-byte checksums and tree structure. Create a separate copy; select it for next start. Original workspace remains available. |

## Automated checks
- 10 frontend tests, including dynamic hierarchy depth, collapse and breadcrumb traversal.
- 73 Rust tests: 41 behavior tests and 32 generated-contract checks. New cases cover durable moves, cycles/stale writes, draft exclusion, search refresh and Unicode offsets, long and multibyte transcripts, deduplicated bounded context, manual-citation idempotency/pinning, accepted/rejected review persistence, full backup recovery, connection exclusion, original-workspace isolation, corrupt archives and foreign schemas.
- Cross-runtime document patches: ten valid fixtures agree between ProseMirror and Rust; four invalid range/quote cases are rejected by both.
- Type checking, frontend production build, frontend/Rust formatting and Clippy with warnings denied pass. The ordinary native debug build also passes; its dependency graph excludes WebDriver and its compile-time feature guards omit test commands.

## Native walkthrough
The first deterministic `native-workspace.mjs` run passed create → rename/move → local transcript import → keyword search/open → manual citation → linked/other-source picker → inline suggestion → acceptance/rejection → restart/history → backup → separate restore → reconnection required → original workspace unchanged. Candidate injection is explicitly labeled and compiled only in the native test build.

The macOS test harness focuses both its native window and WebView before browser text selection. Setting DOM focus alone can leave WebKit’s document inactive and the editor selection unchanged. This helper is compiled out of ordinary builds and does not alter product selection behavior.

The final live imported-source attempt reached create, move, rename, import and search/open, then timed out at native editor focus while macOS was locked. The test process and disposable workspace were cleaned up. It made no model request. The expanded explicit add/remove-source assertions and final foundation/review/import regression reruns remain pending an unlocked desktop; they are not recorded as passing. Earlier M1 live generation remains a separate result, not evidence for this batch’s live source-selection journey.

## Reproduction
From `app/`, run `npm run check`, then the native foundation, review, import and workspace commands documented in [the app README](../../app/README.md), sequentially. An explicitly authorized `WIKI_RUN_LIVE_CHECK=1 npm run test:workspace:native` uses the app’s existing subscription connection for a fictional imported-source suggestion; it uses no real meeting content or Codex credentials. The reject branch uses a labeled fixture. Restored-copy checks disable that test-only live connection override and verify that reconnection is required.

Test databases and `.wiki-backup` files live in disposable private directories and are removed. Screenshots/results remain in ignored `app/.artifacts/`. No database, archive, credential or research transcript belongs in the commit.

## Limits and human review
Search is lexical, not semantic. Relevance ranking and model output require human judgment; the tests do not establish source sufficiency for arbitrary questions. Transcript imports remain text, without transcription or playable audio. The backup format is unencrypted, limited to 100 MB and compatible with matching schema/migrations; it is not sync or a shared workspace. Saved document drafts are included; unsent comment/reply composer text is not durable backup content.

Human QA: create and organize a page, import one of your sources, inspect the default evidence, choose an additional source if useful, and review the suggested wording against its citations. Try a backup/restore with a new filename; the restored copy should require connection setup and leave the original selectable. Broader app milestones remain open.
