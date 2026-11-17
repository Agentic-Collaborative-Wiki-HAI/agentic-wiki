import { afterEach, describe, expect, it } from 'vitest'
import { Editor } from '@tiptap/core'
import { hasUnsupportedPaste, wikiExtensions } from '../src/editor'

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
