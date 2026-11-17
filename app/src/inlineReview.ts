import { Extension } from '@tiptap/core'
import type { Editor } from '@tiptap/core'
import { Plugin, PluginKey } from '@tiptap/pm/state'
import { Decoration, DecorationSet } from '@tiptap/pm/view'
import type { Node } from '@tiptap/pm/model'
import type { PassageAnchor } from './generated/PassageAnchor'
import type { Proposal } from './generated/Proposal'
import type { Evidence } from './generated/Evidence'

export interface InlineReviewState {
  proposal: Proposal | null
  anchor: PassageAnchor | null
  editing: boolean
  onWording?: (text: string) => void
  onEvidence?: (evidence: Evidence) => void
}
const empty: InlineReviewState = { proposal: null, anchor: null, editing: false }
const key = new PluginKey<InlineReviewState>('inlineReview')
function range(doc: Node, anchor: PassageAnchor) {
  let found: { from: number; to: number } | undefined
  doc.forEach((node, pos) => {
    if (node.attrs.blockId !== anchor.blockId || node.type.name !== 'paragraph') return
    const text = node.textBetween(0, node.content.size, '', (leaf) =>
      leaf.type.name === 'hardBreak' ? '\n' : '\uFFFC',
    )
    if (
      anchor.state !== 'attached' ||
      anchor.fromUtf16 < 0 ||
      anchor.toUtf16 > text.length ||
      text.slice(anchor.fromUtf16, anchor.toUtf16) !== anchor.quote
    )
      return
    found = { from: pos + 1 + anchor.fromUtf16, to: pos + 1 + anchor.toUtf16 }
  })
  return found
}
function decorations(doc: Node, state: InlineReviewState) {
  const proposal = state.proposal
  const anchor = proposal?.anchor ?? state.anchor
  if (!anchor || (proposal && (proposal.stale || proposal.status !== 'pending')))
    return DecorationSet.empty
  const target = range(doc, anchor)
  if (!target) return DecorationSet.empty
  const marks = [
    Decoration.inline(target.from, target.to, {
      class: proposal ? 'suggested-deletion' : 'discussion-anchor',
      ...(proposal ? { role: 'deletion', 'aria-label': 'Suggested removal' } : {}),
    }),
  ]
  if (proposal)
    marks.push(
      Decoration.widget(
        target.to,
        () => {
          const wrapper = document.createElement('span')
          wrapper.className = 'inline-suggestion'
          wrapper.dataset.proposalId = proposal.id
          wrapper.contentEditable = 'false'
          if (state.editing) {
            const field = document.createElement('textarea')
            field.className = 'inline-wording'
            field.setAttribute('aria-label', 'Edit proposed wording')
            field.maxLength = 16000
            field.value = proposal.candidate.proposedWording
            const size = () => {
              field.style.height = 'auto'
              field.style.height = `${Math.max(90, field.scrollHeight)}px`
            }
            field.addEventListener('input', () => {
              state.onWording?.(field.value)
              size()
            })
            wrapper.append(field)
            requestAnimationFrame(() => {
              if (field.isConnected) {
                size()
                field.focus()
              }
            })
          } else {
            const addition = document.createElement('ins')
            addition.className = 'suggested-insertion'
            addition.setAttribute('aria-label', 'Suggested replacement')
            addition.textContent = proposal.candidate.proposedWording
            wrapper.append(document.createTextNode(' '), addition)
            proposal.evidence
              .filter((e) => proposal.candidate.citedSourceIds.includes(e.id))
              .forEach((e, i) => {
                const button = document.createElement('button')
                button.className = 'suggestion-citation text-link'
                button.textContent = `[${i + 1}]`
                button.setAttribute('aria-label', `Inspect suggestion source: ${e.title}`)
                button.addEventListener('click', () => state.onEvidence?.(e))
                wrapper.append(button)
              })
          }
          return wrapper
        },
        { side: 1, stopEvent: () => true, ignoreSelection: true },
      ),
    )
  return DecorationSet.create(doc, marks)
}
export const inlineReview = Extension.create({
  name: 'inlineReview',
  addProseMirrorPlugins: () => [
    new Plugin({
      key,
      state: {
        init: () => empty,
        // A document edit invalidates the preview. It never becomes part of the persisted document.
        apply: (tr, current) => tr.getMeta(key) ?? (tr.docChanged ? empty : current),
      },
      props: { decorations: (state) => decorations(state.doc, key.getState(state) ?? empty) },
    }),
  ],
})
export function showInlineReview(editor: Editor, state: InlineReviewState) {
  editor.view.dispatch(editor.state.tr.setMeta(key, state).setMeta('addToHistory', false))
}
