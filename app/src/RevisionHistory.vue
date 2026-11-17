<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, shallowRef } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { wikiExtensions } from './editor'
import { api } from './api'
import { reviewApi } from './reviewApi'
import type { Page } from './generated/Page'
import type { HistoryEntry } from './generated/HistoryEntry'
import type { Discussion } from './generated/Discussion'
import type { Evidence } from './generated/Evidence'
const props = defineProps<{ page: Page; discussions: Discussion[] }>()
const emit = defineEmits<{ thread: [Discussion]; evidence: [Evidence] }>()
const dialog = ref<HTMLDialogElement>(),
  entries = ref<HistoryEntry[]>([]),
  selected = ref<Page | null>(null),
  error = ref('')
const editor = shallowRef<Editor>(),
  citations = ref<Evidence[]>([])
const linked = computed(() =>
  props.discussions.filter(
    (d) =>
      d.proposals.some((p) => p.publishedRevision === selected.value?.revision) ||
      d.baseRevision === selected.value?.revision,
  ),
)
let sequence = 0
async function select(revision: number) {
  const id = ++sequence
  error.value = ''
  try {
    const [p, c] = await Promise.all([
      reviewApi.revision(props.page.id, revision),
      reviewApi.citations(props.page.id, revision),
    ])
    if (id !== sequence) return
    editor.value?.destroy()
    selected.value = p
    citations.value = c
    editor.value = new Editor({
      extensions: wikiExtensions,
      content: p.document.doc,
      editable: false,
      editorProps: { attributes: { 'aria-label': `Revision ${revision} content` } },
    })
  } catch (e) {
    error.value = String(e)
  }
}
async function open() {
  entries.value = await api.history(props.page.id)
  await select(props.page.revision)
  await nextTick()
  dialog.value?.showModal()
}
function openThread(d: Discussion) {
  dialog.value?.close()
  emit('thread', d)
}
function citationClick(event: MouseEvent) {
  const id = (event.target as HTMLElement).closest<HTMLElement>('[data-citation-id]')?.dataset
    .citationId
  const e = citations.value.find((e) => e.id === id)
  if (e) emit('evidence', e)
}
onBeforeUnmount(() => editor.value?.destroy())
defineExpose({ open })
</script>
<template>
  <dialog ref="dialog" class="history-dialog">
    <div class="discussion-heading">
      <h2>Page history</h2>
      <button @click="dialog?.close()">Back to page</button>
    </div>
    <p>{{ page.title }}</p>
    <p v-if="error" role="alert" class="review-error">{{ error }}</p>
    <nav class="revision-list" aria-label="Saved revisions">
      <button
        v-for="entry in entries"
        :key="entry.revision"
        :aria-pressed="selected?.revision === entry.revision"
        @click="select(entry.revision)"
      >
        Revision {{ entry.revision }} ·
        {{ entry.actorId === 'local-operator' ? 'You' : 'Example workspace' }}
        <time>{{ new Date(entry.createdAt).toLocaleString() }}</time>
      </button>
    </nav>
    <div class="revision-layout">
      <section class="revision-content">
        <h3>Revision {{ selected?.revision }}</h3>
        <EditorContent v-if="editor" :editor="editor" @click="citationClick" />
      </section>
      <aside class="revision-discussions">
        <h3>Discussion and decisions</h3>
        <p v-if="!linked.length">No discussion is associated with this revision.</p>
        <article v-for="d in linked" :key="d.id" class="reference-card">
          <blockquote>{{ d.anchor.quote }}</blockquote>
          <p v-for="m in d.messages" :key="m.id">{{ m.body }}</p>
          <p v-for="p in d.proposals" :key="p.id">
            <strong>{{ p.status }}</strong><span v-if="p.publishedRevision">
              · Revision {{ p.publishedRevision }}</span
            ><br />{{ p.candidate.proposedWording }}
          </p>
          <button class="text-link" @click="openThread(d)">Open full discussion</button>
        </article>
      </aside>
    </div>
  </dialog>
</template>
