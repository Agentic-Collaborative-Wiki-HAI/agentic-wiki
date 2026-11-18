import { Extension, Mark, Node, mergeAttributes } from '@tiptap/core'
import { Plugin } from '@tiptap/pm/state'
import { history, undo, redo } from '@tiptap/pm/history'
import { keymap } from '@tiptap/pm/keymap'
import { baseKeymap } from '@tiptap/pm/commands'
import { inlineReview } from './inlineReview.ts'

const blockAttrs = () => ({
  blockId: {
    default: null,
    parseHTML: (el: HTMLElement) => el.dataset.blockId,
    renderHTML: (attrs: Record<string, unknown>) => ({ 'data-block-id': attrs.blockId }),
  },
})

export const wikiExtensions = [
  inlineReview,
  Node.create({ name: 'doc', topNode: true, content: 'block+' }),
  Node.create({ name: 'text', group: 'inline' }),
  Node.create({
    name: 'paragraph',
    group: 'block',
    content: 'inline*',
    addAttributes: blockAttrs,
    parseHTML: () => [{ tag: 'p' }],
    renderHTML: ({ HTMLAttributes }) => ['p', HTMLAttributes, 0],
  }),
  Node.create({
    name: 'heading',
    group: 'block',
    content: 'inline*',
    defining: true,
    addAttributes: () => ({ ...blockAttrs(), level: { default: 2, rendered: false } }),
    parseHTML: () => [1, 2, 3, 4, 5, 6].map((level) => ({ tag: `h${level}`, attrs: { level } })),
    renderHTML: ({ node, HTMLAttributes }) => [`h${node.attrs.level}`, HTMLAttributes, 0],
  }),
  Node.create({
    name: 'hardBreak',
    inline: true,
    group: 'inline',
    selectable: false,
    parseHTML: () => [{ tag: 'br' }],
    renderHTML: () => ['br'],
  }),
  Node.create({
    name: 'citation',
    inline: true,
    group: 'inline',
    atom: true,
    addAttributes: () => ({
      citationId: { default: null, parseHTML: (el: HTMLElement) => el.dataset.citationId },
    }),
    parseHTML: () => [{ tag: 'span[data-citation-id]' }],
    renderHTML: ({ node }) => [
      'span',
      { 'data-citation-id': node.attrs.citationId, class: 'citation-marker' },
      '[…]',
    ],
  }),
  Node.create({
    name: 'note',
    inline: true,
    group: 'inline',
    atom: true,
    addAttributes: () => ({
      noteId: { default: null, parseHTML: (el: HTMLElement) => el.dataset.noteId },
    }),
    parseHTML: () => [{ tag: 'span[data-note-id]' }],
    renderHTML: ({ node }) => [
      'span',
      { 'data-note-id': node.attrs.noteId, class: 'citation-marker' },
      '[…]',
    ],
  }),
  Mark.create({
    name: 'bold',
    parseHTML: () => [{ tag: 'strong' }, { tag: 'b' }],
    renderHTML: ({ HTMLAttributes }) => ['strong', HTMLAttributes, 0],
  }),
  Mark.create({
    name: 'italic',
    parseHTML: () => [{ tag: 'em' }, { tag: 'i' }],
    renderHTML: ({ HTMLAttributes }) => ['em', HTMLAttributes, 0],
  }),
  Mark.create({
    name: 'code',
    code: true,
    excludes: '_',
    parseHTML: () => [{ tag: 'code' }],
    renderHTML: ({ HTMLAttributes }) => ['code', HTMLAttributes, 0],
  }),
  Mark.create({
    name: 'link',
    inclusive: false,
    addAttributes: () => ({ href: { default: null } }),
    parseHTML: () => [
      {
        tag: 'a[href]',
        getAttrs: (el: HTMLElement) =>
          /^(https?:\/\/|#)/.test(el.getAttribute('href') ?? '') ? null : false,
      },
    ],
    renderHTML: ({ HTMLAttributes }) => [
      'a',
      mergeAttributes(HTMLAttributes, { rel: 'noopener noreferrer' }),
      0,
    ],
  }),
  Extension.create({
    name: 'citationNumbers',
    addProseMirrorPlugins: () => [
      new Plugin({
        view(view) {
          const update = () => {
            const numbers = new Map<string, number>()
            for (const marker of view.dom.querySelectorAll<HTMLElement>(
              '[data-citation-id], [data-note-id]',
            )) {
              const id = marker.dataset.citationId ?? marker.dataset.noteId!
              if (!numbers.has(id)) numbers.set(id, numbers.size + 1)
              const number = numbers.get(id)!
              if (marker.textContent !== `[${number}]`) marker.textContent = `[${number}]`
              marker.setAttribute('role', 'button')
              marker.setAttribute('tabindex', '0')
              marker.setAttribute(
                'aria-label',
                `Open ${marker.dataset.noteId ? 'note' : 'citation'} ${number}`,
              )
            }
          }
          update()
          const keydown = (event: KeyboardEvent) => {
            const marker = (event.target as HTMLElement)?.closest<HTMLElement>(
              '[data-citation-id], [data-note-id]',
            )
            if (marker && (event.key === 'Enter' || event.key === ' ')) {
              event.preventDefault()
              marker.click()
            }
          }
          view.dom.addEventListener('keydown', keydown)
          return { update, destroy: () => view.dom.removeEventListener('keydown', keydown) }
        },
      }),
    ],
  }),
  Extension.create({
    name: 'editingKeys',
    addProseMirrorPlugins: () => [
      history(),
      keymap({ 'Mod-z': undo, 'Mod-Shift-z': redo }),
      keymap(baseKeymap),
    ],
  }),
  Extension.create({
    name: 'blockIdentities',
    addProseMirrorPlugins: () => [
      new Plugin({
        appendTransaction(transactions, _old, state) {
          if (!transactions.some((t) => t.docChanged)) return null
          const seen = new Set<string>()
          const tr = state.tr
          state.doc.descendants((node, pos) => {
            if (!node.isTextblock) return
            let id = node.attrs.blockId
            if (!id || seen.has(id)) {
              id = crypto.randomUUID()
              tr.setNodeMarkup(pos, undefined, { ...node.attrs, blockId: id })
            }
            seen.add(id)
          })
          return tr.docChanged ? tr : null
        },
      }),
    ],
  }),
]

export const hasUnsupportedPaste = (html: string) =>
  /<\s*(table|ul|ol|li|img|video|audio|iframe|pre|blockquote)\b/i.test(html)
