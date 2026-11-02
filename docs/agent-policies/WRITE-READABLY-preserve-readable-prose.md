---
id: WRITE-READABLY
type: operational
scope: "Agent edits to Markdown, prose, comments and examples."
status: adopted
authority: "Aslan approved D2 (the eleven-policy adoption and associated routing) in the active Agentic Wiki migration chat on 2026-11-02."
revised: "2026-11-02"
depends_on: ["RESOLVE-DEPENDENCIES"]
related_to: []
---
# Preserve readable prose
Do not hard-wrap prose in Markdown, comments, docs, or examples. Let editors handle soft wrapping. Preserve paragraphs as single lines unless line breaks carry meaning, such as lists, tables, code blocks, quoted text, frontmatter, or an existing semantic-line-break style.

Avoid reflow-only diffs. When editing prose, change the smallest relevant span instead of rewrapping neighboring paragraphs.

When touching existing Markdown or prose, apply this preferred style to the paragraph, section, or example being edited so files converge over time. Do not mass-reformat untouched sections just to normalize style unless asked for a cleanup pass.

Prefer compact Markdown heading spacing in hand-authored docs: do not add blank lines only to separate adjacent headings from each other. Follow existing file style, and let explicit project tooling win when a formatter or linter requires a different layout.
