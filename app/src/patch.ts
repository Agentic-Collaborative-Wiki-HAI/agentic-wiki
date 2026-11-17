import { getSchema } from '@tiptap/core'
import { Transform } from '@tiptap/pm/transform'
import { wikiExtensions } from './editor.ts'
import type { RichDocument } from './generated/RichDocument'
import type { PassagePatch } from './generated/PassagePatch'

export function applyPassagePatch(document: RichDocument, patch: PassagePatch): RichDocument {
  if (document.schemaVersion !== 1) throw new Error('Unsupported document schema')
  const schema = getSchema(wikiExtensions)
  const doc = schema.nodeFromJSON(document.doc)
  let position = -1
  let target: typeof doc | null = null
  doc.forEach((node, offset) => {
    if (node.attrs.blockId === patch.blockId) {
      if (target) throw new Error('Duplicate block identity')
      target = node
      position = offset
    }
  })
  const block = target as typeof doc | null
  if (!block || block.type.name !== 'paragraph') throw new Error('A paragraph is required')
  const plain = block.textBetween(0, block.content.size, '', (leaf) =>
    leaf.type.name === 'hardBreak' ? '\n' : '\uFFFC',
  )
  const { fromUtf16: from, toUtf16: to } = patch
  if (
    !Number.isSafeInteger(from) ||
    !Number.isSafeInteger(to) ||
    from < 0 ||
    to < from ||
    to > plain.length
  )
    throw new Error('Invalid range')
  const splitsSurrogate = (i: number) =>
    i > 0 &&
    i < plain.length &&
    /[\uD800-\uDBFF]/.test(plain[i - 1]) &&
    /[\uDC00-\uDFFF]/.test(plain[i])
  if (splitsSurrogate(from) || splitsSurrogate(to))
    throw new Error('Range splits a Unicode surrogate pair')
  if (plain.slice(from, to) !== patch.expectedQuote) throw new Error('Selected text does not match')
  const replacement = patch.replacement.map((node) => schema.nodeFromJSON(node))
  const tr = new Transform(doc).replaceWith(position + 1 + from, position + 1 + to, replacement)
  return { schemaVersion: 1, doc: tr.doc.toJSON() }
}
