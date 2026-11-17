import { afterEach, expect, it } from 'vitest'
import { Editor } from '@tiptap/core'
import { wikiExtensions } from '../src/editor'
import { showInlineReview } from '../src/inlineReview'
import { mentionsAgent } from '../src/threadText'
import type { Proposal } from '../src/generated/Proposal'
let editor: Editor | undefined
afterEach(() => editor?.destroy())
const content = {
  type: 'doc',
  content: [
    {
      type: 'paragraph',
      attrs: { blockId: 'p1' },
      content: [{ type: 'text', text: 'Use the spreadsheet.', marks: [{ type: 'bold' }] }],
    },
  ],
}
const proposal: Proposal = {
  id: 'p',
  threadId: 't',
  parentId: null,
  baseRevision: 1,
  anchor: { blockId: 'p1', fromUtf16: 8, toUtf16: 19, quote: 'spreadsheet', state: 'attached' },
  candidate: {
    proposedWording: 'booking calendar',
    explanation: 'Source supports a calendar.',
    citedSourceIds: ['e'],
  },
  evidence: [
    {
      id: 'e',
      title: 'Orientation',
      quote: 'Use the booking calendar.',
      source: { objectId: 'recording', revision: 1, blockId: 'b1' },
    },
  ],
  requestText: '@agent Check this.',
  origin: 'Test',
  status: 'pending',
  stale: false,
  publishedRevision: null,
  decidedAt: null,
  createdAt: '2026-11-17T00:00:00Z',
}
it('shows exact inline changes and evidence without changing canonical JSON, marks or offsets', () => {
  editor = new Editor({ extensions: wikiExtensions, content, editable: false })
  const initial = editor.getJSON()
  let source = ''
  showInlineReview(editor, {
    proposal,
    anchor: null,
    editing: false,
    onEvidence: (e) => {
      source = e.id
    },
  })
  expect(editor.view.dom.querySelector('.suggested-deletion')?.textContent).toBe('spreadsheet')
  expect(editor.view.dom.querySelector('ins')?.textContent).toBe('booking calendar')
  editor.view.dom.querySelector<HTMLButtonElement>('.suggestion-citation')!.click()
  expect(source).toBe('e')
  expect(editor.getJSON()).toEqual(initial)
  showInlineReview(editor, { proposal: null, anchor: null, editing: false })
  expect(editor.view.dom.textContent).toBe('Use the spreadsheet.')
  expect(editor.getJSON()).toEqual(initial)
})
it('keeps edited suggestion text outside the saved document and restores the proposal on cancel', () => {
  editor = new Editor({ extensions: wikiExtensions, content, editable: false })
  let draft = ''
  showInlineReview(editor, {
    proposal,
    anchor: null,
    editing: true,
    onWording: (text) => {
      draft = text
    },
  })
  const field = editor.view.dom.querySelector('textarea')!
  field.value = 'a different calendar'
  field.dispatchEvent(new Event('input'))
  expect(draft).toBe('a different calendar')
  expect(editor.getJSON()).toEqual(content)
  showInlineReview(editor, { proposal, anchor: null, editing: false })
  expect(editor.view.dom.querySelector('ins')?.textContent).toBe('booking calendar')
})
it('does not project stale or mismatched suggestions onto a changed passage', () => {
  editor = new Editor({ extensions: wikiExtensions, content, editable: false })
  for (const p of [
    { ...proposal, stale: true },
    { ...proposal, anchor: { ...proposal.anchor, quote: 'different' } },
  ]) {
    showInlineReview(editor, { proposal: p, anchor: null, editing: false })
    expect(editor.view.dom.querySelector('.inline-suggestion')).toBeNull()
  }
})
it('only treats a standalone agent mention as an explicit request', () => {
  for (const text of [
    '@agent Check this.',
    'Please @agent, make that shorter.',
    '@Agent: revise it',
  ])
    expect(mentionsAgent(text)).toBe(true)
  for (const text of ['Please check this.', 'person@agent.com', '@agentic', 'Discuss the agent'])
    expect(mentionsAgent(text)).toBe(false)
})
