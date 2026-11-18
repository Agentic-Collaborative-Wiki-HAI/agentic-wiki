import { afterEach, describe, expect, it } from 'vitest'
import { Editor } from '@tiptap/core'
import { hasUnsupportedPaste, wikiExtensions } from '../src/editor'
import { showInlineReview } from '../src/inlineReview'
import type { Proposal } from '../src/generated/Proposal'
import type { Evidence } from '../src/generated/Evidence'

let editor: Editor | undefined
afterEach(() => editor?.destroy())
describe('bounded editor schema', () => {
  it('retains marks, citation identity and block identity across JSON round trip', () => {
    const content = {
      type: 'doc',
      content: [
        {
          type: 'paragraph',
          attrs: { blockId: 'p1' },
          content: [
            { type: 'text', text: '🔬 Calendar', marks: [{ type: 'bold' }] },
            { type: 'citation', attrs: { citationId: 'citation-1' } },
          ],
        },
      ],
    }
    editor = new Editor({ extensions: wikiExtensions, content })
    expect(editor.getJSON()).toEqual(content)
  })
  it('assigns new IDs after a split rather than duplicating an anchor identity', () => {
    editor = new Editor({
      extensions: wikiExtensions,
      content: {
        type: 'doc',
        content: [
          {
            type: 'paragraph',
            attrs: { blockId: 'p1' },
            content: [{ type: 'text', text: 'First second' }],
          },
        ],
      },
    })
    editor.commands.setTextSelection(6)
    editor.commands.splitBlock()
    const ids = editor.getJSON().content!.map((n) => n.attrs!.blockId)
    expect(ids).toHaveLength(2)
    expect(new Set(ids).size).toBe(2)
    expect(ids[0]).toBe('p1')
  })
})

it('does not silently flatten pasted structures outside the bounded schema', () => {
  expect(hasUnsupportedPaste('<table><tr><td>Important cell</td></tr></table>')).toBe(true)
  expect(hasUnsupportedPaste('<ul><li>List item</li></ul>')).toBe(true)
  expect(hasUnsupportedPaste('<p>A <strong>supported</strong> paragraph</p>')).toBe(false)
})

it('numbers citations by first appearance, reuses numbers, and preserves stored identities', () => {
  const content = {
    type: 'doc',
    content: [
      {
        type: 'paragraph',
        attrs: { blockId: 'p1' },
        content: [
          { type: 'citation', attrs: { citationId: 'second-source' } },
          { type: 'citation', attrs: { citationId: 'first-source' } },
          { type: 'citation', attrs: { citationId: 'second-source' } },
        ],
      },
    ],
  }
  editor = new Editor({ extensions: wikiExtensions, content })
  expect(
    [...editor.view.dom.querySelectorAll('[data-citation-id]')].map((e) => e.textContent),
  ).toEqual(['[1]', '[2]', '[1]'])
  expect(editor.getJSON()).toEqual(content)
})

// A preview must not impersonate the numbering of the article's saved citations.
it('keeps proposed sources distinct without changing the saved article or its citations', () => {
  const content = {
    type: 'doc',
    content: [
      {
        type: 'paragraph',
        attrs: { blockId: 'booking' },
        content: [
          { type: 'text', text: 'Use the spreadsheet.' },
          { type: 'citation', attrs: { citationId: 'saved-source' } },
        ],
      },
    ],
  }
  editor = new Editor({ extensions: wikiExtensions, content })
  const evidence: Evidence = {
    id: 'source',
    title: 'Recording',
    quote: 'Use the calendar.',
    source: { objectId: 'recording', revision: 1, blockId: 'passage' },
  }
  const proposal = {
    id: 'proposal',
    status: 'pending',
    stale: false,
    anchor: {
      blockId: 'booking',
      fromUtf16: 0,
      toUtf16: 20,
      quote: 'Use the spreadsheet.',
      state: 'attached',
    },
    candidate: {
      proposedWording: 'Use the calendar.',
      explanation: 'The recording names the calendar.',
      citedSourceIds: ['source'],
    },
    evidence: [evidence],
  } as Proposal
  let inspected: Evidence | undefined
  showInlineReview(editor, {
    proposal,
    anchor: null,
    editing: false,
    onEvidence: (e) => {
      inspected = e
    },
  })
  expect(editor.view.dom.querySelector('[data-citation-id]')?.textContent).toBe('[1]')
  const button = editor.view.dom.querySelector<HTMLButtonElement>('.suggestion-citation')!
  expect(button.textContent).not.toBe('[1]')
  expect(button.textContent).toContain('proposed source')
  button.click()
  expect(inspected).toEqual(evidence)
  expect(editor.getJSON()).toEqual(content)
  showInlineReview(editor, { proposal: null, anchor: null, editing: false })
  expect(editor.view.dom.querySelector('.suggestion-citation')).toBeNull()
  expect(editor.getJSON()).toEqual(content)
})
