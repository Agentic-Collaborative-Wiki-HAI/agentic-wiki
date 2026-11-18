import { JSDOM } from 'jsdom'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import assert from 'node:assert/strict'

const dom = new JSDOM('<!doctype html><body></body>')
globalThis.window = dom.window
globalThis.document = dom.window.document
Object.defineProperty(globalThis, 'navigator', { value: dom.window.navigator, configurable: true })
const { applyPassagePatch } = await import('../src/patch.ts')
const root = fileURLToPath(new URL('../', import.meta.url))
const cargo = process.env.CARGO ?? 'cargo'
const build = spawnSync(
  cargo,
  [
    'build',
    '--manifest-path',
    'src-tauri/Cargo.toml',
    '--no-default-features',
    '--bin',
    'editor-contract',
  ],
  { cwd: root, encoding: 'utf8' },
)
if (build.status !== 0) {
  process.stderr.write(build.stderr)
  process.exit(build.status ?? 1)
}
const executable = fileURLToPath(
  new URL('../src-tauri/target/debug/editor-contract', import.meta.url),
)
const text = (value, marks = []) =>
  marks.length ? { type: 'text', text: value, marks } : { type: 'text', text: value }
const document = (content) => ({
  schemaVersion: 1,
  doc: { type: 'doc', content: [{ type: 'paragraph', attrs: { blockId: 'booking' }, content }] },
})
const fixtures = [
  {
    name: 'keep margin note through adjacent rewrite',
    content: [text('Old'), { type: 'note', attrs: { noteId: 'note-1' } }],
    from: 0,
    to: 3,
    quote: 'Old',
    replacement: [text('New')],
  },
  {
    name: 'plain replacement',
    content: [text('Use the spreadsheet.')],
    from: 8,
    to: 19,
    quote: 'spreadsheet',
    replacement: [text('calendar')],
  },
  {
    name: 'emoji before selection',
    content: [text('🔬 Use spreadsheet')],
    from: 7,
    to: 18,
    quote: 'spreadsheet',
    replacement: [text('calendar')],
  },
  {
    name: 'combining characters',
    content: [text('Cafe\u0301 before')],
    from: 0,
    to: 5,
    quote: 'Cafe\u0301',
    replacement: [text('Café')],
  },
  {
    name: 'preserve surrounding marks',
    content: [text('Use '), text('old spreadsheet', [{ type: 'bold' }]), text(' now')],
    from: 8,
    to: 19,
    quote: 'spreadsheet',
    replacement: [text('calendar', [{ type: 'italic' }])],
  },
  {
    name: 'keep trailing citation',
    content: [text('Old wording'), { type: 'citation', attrs: { citationId: 'source-1' } }],
    from: 0,
    to: 3,
    quote: 'Old',
    replacement: [text('New')],
  },
  {
    name: 'replace citation atom',
    content: [text('Booking '), { type: 'citation', attrs: { citationId: 'source-1' } }],
    from: 8,
    to: 9,
    quote: '\uFFFC',
    replacement: [{ type: 'citation', attrs: { citationId: 'source-2' } }],
  },
  {
    name: 'preserve source link',
    content: [
      text('Use '),
      text('calendar', [{ type: 'link', attrs: { href: 'https://example.org/calendar' } }]),
    ],
    from: 0,
    to: 3,
    quote: 'Use',
    replacement: [text('Check')],
  },
  {
    name: 'hard break range',
    content: [text('A'), { type: 'hardBreak' }, text('B')],
    from: 1,
    to: 2,
    quote: '\n',
    replacement: [text(' ')],
  },
  {
    name: 'empty insertion',
    content: [text('AB')],
    from: 1,
    to: 1,
    quote: '',
    replacement: [text('🔬')],
  },
  {
    name: 'delete whole passage',
    content: [text('Remove')],
    from: 0,
    to: 6,
    quote: 'Remove',
    replacement: [],
  },
]
const canonical = (doc) => {
  const copy = structuredClone(doc)
  for (const block of copy.doc.content) if (!block.content) block.content = []
  return copy
}
for (const fixture of fixtures) {
  const input = {
    document: document(fixture.content),
    patch: {
      blockId: 'booking',
      fromUtf16: fixture.from,
      toUtf16: fixture.to,
      expectedQuote: fixture.quote,
      replacement: fixture.replacement,
    },
    anchors: [],
  }
  const js = applyPassagePatch(input.document, input.patch)
  const rust = spawnSync(executable, [], { input: JSON.stringify(input), encoding: 'utf8' })
  assert.equal(rust.status, 0, `${fixture.name}: ${rust.stdout}`)
  assert.deepEqual(canonical(JSON.parse(rust.stdout).document), canonical(js), fixture.name)
  console.log(`PASS ${fixture.name}`)
}
for (const invalid of [
  { fromUtf16: 1, toUtf16: 2, expectedQuote: '' },
  { fromUtf16: 0, toUtf16: 2, expectedQuote: 'wrong' },
  { fromUtf16: 7, toUtf16: 3, expectedQuote: '' },
  { fromUtf16: 0, toUtf16: 999, expectedQuote: '' },
]) {
  const input = {
    document: document([text('🔬 example')]),
    patch: { blockId: 'booking', replacement: [text('x')], ...invalid },
    anchors: [],
  }
  assert.throws(() => applyPassagePatch(input.document, input.patch))
  const rust = spawnSync(executable, [], { input: JSON.stringify(input), encoding: 'utf8' })
  assert.equal(rust.status, 2)
}
console.log('PASS 4 invalid-range/quote cases rejected by both runtimes')
