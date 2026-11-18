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
  showInlineReview(editor, { proposal, anchor: null, editing: false })
  expect(editor.view.dom.querySelector('[data-citation-id]')?.textContent).toBe('[1]')
  expect(editor.view.dom.querySelector('.suggestion-citation')).toBeNull()
  expect(editor.view.dom.querySelector('.suggested-insertion')?.textContent).toBe(
    'Use the calendar.',
  )
  expect(editor.getJSON()).toEqual(content)
  showInlineReview(editor, { proposal: null, anchor: null, editing: false })
  expect(editor.view.dom.querySelector('.suggestion-citation')).toBeNull()
  expect(editor.getJSON()).toEqual(content)
})

it('numbers explicit notes alongside citations and supports keyboard activation in read view', () => {
  const content = {
    type: 'doc',
    content: [
      {
        type: 'paragraph',
        attrs: { blockId: 'p' },
        content: [
          { type: 'text', text: 'Passage' },
          { type: 'note', attrs: { noteId: 'note' } },
          { type: 'citation', attrs: { citationId: 'cite' } },
        ],
      },
    ],
  }
  editor = new Editor({ extensions: wikiExtensions, content, editable: false })
  const note = editor.view.dom.querySelector<HTMLElement>('[data-note-id]')!
  const cite = editor.view.dom.querySelector<HTMLElement>('[data-citation-id]')!
  expect([note.textContent, cite.textContent]).toEqual(['[1]', '[2]'])
  let clicks = 0
  note.addEventListener('click', () => clicks++)
  for (const key of ['Enter', ' '])
    note.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true, cancelable: true }))
  expect(clicks).toBe(2)
  expect(editor.getJSON()).toEqual(content)
})
it('highlights every open discussion without inserting numbered markers into the document', () => {
  const content = {
    type: 'doc',
    content: [
      {
        type: 'paragraph',
        attrs: { blockId: 'p' },
        content: [{ type: 'text', text: 'First second' }],
      },
    ],
  }
  editor = new Editor({ extensions: wikiExtensions, content })
  const anchor = {
    blockId: 'p',
    fromUtf16: 0,
    toUtf16: 5,
    quote: 'First',
    state: 'attached' as const,
  }
  showInlineReview(editor, {
    proposal: null,
    anchor: null,
    editing: false,
    threads: [
      { id: 'first', anchor },
      { id: 'second', anchor: { ...anchor, fromUtf16: 6, toUtf16: 12, quote: 'second' } },
    ],
  })
  expect(editor.view.dom.querySelectorAll('[data-thread-anchor]')).toHaveLength(2)
  expect(editor.view.dom.querySelectorAll('[data-citation-id], [data-note-id]')).toHaveLength(0)
  expect(editor.getJSON()).toEqual(content)
  showInlineReview(editor, { proposal: null, anchor: null, editing: false, threads: [] })
  expect(editor.view.dom.querySelectorAll('[data-thread-anchor]')).toHaveLength(0)
})
