# App interface and authoring QA — 2026-11-17
Scope: approved U1–U3 and the accompanying authoring corrections, checked in the macOS Tauri application. This follows the [Prototype audit](app-prototype-audit-2026-11-17.md). The Figma file and unrelated design records are outside this change.

## Implemented behavior
| Area | Current behavior |
| --- | --- |
| Page shell | Breadcrumbs remain available at laptop widths. The title and page actions have separate room. Chat uses the rail width; compact layouts collapse navigation and place the context rail below the article. The body retains its width limit. Enlarged header text reflows rather than overlapping Search. |
| Discussion | Open threads appear as passage-aligned cards with highlighted anchors, without numbered article markers or thread-switching tabs. Resolving removes the thread and its highlight from the active page; history preserves the discussion and decisions. Long cards push later cards down rather than overlapping. |
| Margin notes | Select text → Add margin note creates an explicit note, shown beside its marker and in endnotes. Notes and citations share first-appearance numbering; Citations only filters the endnotes. |
| Editing | The title is edited with the page. Citation and margin-note insertion works in read and edit states. In edit mode these occurrences stay in the draft until Save; Cancel preserves the saved document. Rename / move is absent from the interface; page moving remains deferred. |
| Suggestions | An editorial rewrite may use zero source excerpts. Accept publishes the wording and decision while retaining existing explicit notes/citations. It creates no numbered article references from discussion evidence. Reject changes neither the saved text nor its references. A stale suggestion requires a fresh passage selection. |
| Agent configuration | Settings → Agent instructions exposes separate passage-editing and global-Chat instructions, with editable defaults and explicit Save. Instructions are workspace-local and survive restart/backup. Fixed response, tool-access and human-publication boundaries remain enforced by the app. |
| Chat | Messages can be instructions, discussion or questions. Context discloses the current selection/default excerpts without requiring sources for an editorial response. Hide chat uses the same Chat icon and preserves conversation and composer draft. Opening Chat makes no model request. |
| History | A dedicated page shows saved titles/content, revision navigation, explicit references and associated discussion. It replaces the wiki layout and returns to it; it is not a modal. Settings can temporarily replace History without leaving both pages visible. |

## Automated and native verification
All native journeys used disposable workspaces and fictional fixture content, not the working wiki. Tests that supply model candidates identify them as fixtures.

- Rust: 93 tests pass, including migration/annotation storage, source-free proposals, preservation of existing references, exact evidence validation, stale writes, rollback, draft cancellation, historical titles, instructions, backup/restore and restart.
- Frontend: 13 tests pass. The JavaScript/Rust editor contract agrees on eleven valid and four invalid cases, including note atoms.
- TypeScript/Vite build, Rust formatting, Prettier and Clippy with warnings denied pass.
- `native-authoring.mjs`: create citation/note while editing; inspect/filter endnotes; cancel or save title/content; open History and historic titles; Settings return; source-free acceptance with no added references; simultaneous discussion cards; resolution hides its card/highlight; Chat icon/draft preservation; instructions and annotations after restart; responsive layouts.
- `native-review.mjs`: passage comments, inline proposals, evidence, human editing, accept/reject, stale selection/retry, history and process restart. The regression exposed a stale editing selection; Save and Cancel now clear it before a refreshed suggestion can be requested.
- `native-workspace.mjs`: create/title-edit a page, import a linked transcript, keyword search, explicit citation, linked/default/selected source context, review, backup/restore into a separate writable copy, and original-workspace preservation.
- `native-chat.mjs`: Chat to editable unpublished draft, publication, provenance, missing evidence, cancellation and recovery. `native-smoke.mjs` covers navigation, independent contents controls, tabs, Chat, Save/Cancel and draft recovery.

The ordinary app build excludes native test commands and the WebDriver server. `.artifacts/`, temporary workspaces and databases remain untracked.

## Live subscription check
One real ChatGPT request through an `@agent` reply used zero source excerpts and the fictional booking sentence. It requested shorter wording without changing the booking method. The model returned “Book the microscope via the shared spreadsheet before your first session.” with an explanation and an empty citation list. The saved document stayed unchanged until Accept; acceptance added a revision and no references. History and restart retained the result. This demonstrates editorial rewriting without a mandatory source; it is not a factual-answer quality benchmark.

## Keyboard and visual checks
Actual macOS key events, rather than only synthetic DOM events, verified References/Comments ArrowLeft/ArrowRight navigation and focus, Enter on a citation opening its evidence, Space on a margin-note marker focusing its card, Enter on a historical note opening its endnote, and Escape hiding Chat with focus returned to its opener. Picker label IDs are unique and the new note dialog is named. Full VoiceOver coverage remains open.

Screenshots were inspected for read/edit annotations, discussion cards, History and compact layouts. Diagnostic doubled font sizes and increased text spacing retain readable body text; the header reflows at enlarged sizes. The muted text color is darkened to improve contrast on the subtle panel background. These are bounded checks, not a WCAG conformance claim or a completed test of every dialog at every scaling level. Tauri's ordinary minimum window width remains 960; narrower WebDriver sizes are diagnostic.

Local ignored evidence is under `app/.artifacts/native-authoring/`, `native-review/`, `native-workspace/`, `native-chat/`, `native-accessibility/` and `native-editorial-live/`. The temporary accessibility/live-editorial harnesses are local; the committed authoring and existing native suites provide repeatable fixture checks.

## Data preservation and limits
Existing saved content and historical references are not rewritten. Citations previously created by older app versions remain part of those documents; the corrected acceptance behavior applies to future decisions. Pre-migration revisions receive the title available at migration, since older title changes were not recorded as snapshots. New revisions preserve their own title.

Cancel removes draft occurrences, not the registered immutable annotation record; an unused record can remain in local storage without appearing in the article, rail or endnotes. Note editing/versioning and richer citation intake are future work. Comments start from saved text; citations and notes can be added while editing.

The dedicated history page still displays full snapshots rather than a complete comparison UI for arbitrary manual edits. Richer editing controls, timestamp/audio alignment, full Search results, source ingestion beyond the implemented importer, automatic maintenance, page moving and shared operation remain separate work. This batch does not claim full Prototype parity or close those spikes.

## Human QA
Review the everyday interaction feel: selecting text → comment or margin note, multiple passage-aligned cards, a source-free `@agent` rewrite, editing the title plus annotations, and History's return path. Review the workspace instructions in Settings and adjust them to the desired tone. The broader Chat-to-draft presentation also retains its existing human-QA item.
