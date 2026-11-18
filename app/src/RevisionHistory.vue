<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, shallowRef } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { wikiExtensions } from './editor'
import { api } from './api'
import { authoringApi } from './authoringApi'
import SourcePreview from './SourcePreview.vue'
import type { ReferenceEntry } from './generated/ReferenceEntry'
import { reviewApi } from './reviewApi'
import type { Page } from './generated/Page'
import type { HistoryEntry } from './generated/HistoryEntry'
import type { Discussion } from './generated/Discussion'
import type { Evidence } from './generated/Evidence'
const props = defineProps<{ page: Page; discussions: Discussion[]; shown: boolean }>()
const emit = defineEmits<{ thread: [Discussion]; evidence: [Evidence]; open: []; close: [] }>()
const visible = ref(false),
  entries = ref<HistoryEntry[]>([]),
  selected = ref<Page | null>(null),
  error = ref('')
const editor = shallowRef<Editor>(),
  references = ref<ReferenceEntry[]>([])
const sourcePreview = ref<InstanceType<typeof SourcePreview>>()
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
    const p = await reviewApi.revision(props.page.id, revision)
    const refs = await authoringApi.references(p.document)
    if (id !== sequence) return
    editor.value?.destroy()
    selected.value = p
    references.value = refs
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
  visible.value = true
  emit('open')
  await nextTick()
  document.querySelector<HTMLElement>('.history-page h1')?.focus()
}
function openThread(d: Discussion) {
  visible.value = false
  emit('close')
  emit('thread', d)
}
function citationClick(event: MouseEvent) {
  const marker = (event.target as HTMLElement).closest<HTMLElement>(
    '[data-citation-id], [data-note-id]',
  )
  const id = marker?.dataset.citationId ?? marker?.dataset.noteId
  const entry = references.value.find((e) => e.id === id)
  if (entry?.evidence) void sourcePreview.value?.open(entry.evidence)
  else if (entry) {
    const card = document.querySelector<HTMLElement>(
      `[data-history-reference="${CSS.escape(entry.id)}"]`,
    )
    card?.scrollIntoView({ block: 'center' })
    card?.focus()
  }
}
onBeforeUnmount(() => editor.value?.destroy())
defineExpose({ open })
function close() {
  visible.value = false
  emit('close')
}
</script>
<template>
  <main v-if="visible && shown" class="history-page" aria-label="Page history">
    <div class="discussion-heading">
      <h1 tabindex="-1">Page history</h1>
      <button @click="close">Back to page</button>
    </div>
    <p>{{ page.title }}</p>
    <div class="history-columns">
      <p v-if="error" role="alert" class="review-error">{{ error }}</p>
      <nav class="revision-list" aria-label="Saved revisions">
        <button
          v-for="entry in entries"
          :key="entry.revision"
          :aria-pressed="selected?.revision === entry.revision"
          @click="select(entry.revision)"
        >
          Revision {{ entry.revision }} ·
          {{
            entry.actorId === 'local-operator'
              ? 'You'
              : entry.actorId === 'local-import'
                ? 'You · source import'
                : 'Example workspace'
          }}
          <time>{{ new Date(entry.createdAt).toLocaleString() }}</time>
        </button>
      </nav>
      <div class="revision-layout">
        <section class="revision-content">
          <h2>{{ selected?.title }}</h2>
          <h3>Revision {{ selected?.revision }}</h3>
          <EditorContent v-if="editor" :editor="editor" @click="citationClick" />
          <section v-if="references.length" aria-label="Revision endnotes">
            <h3>Endnotes</h3>
            <article
              v-for="(entry, index) in references"
              :key="entry.id"
              :data-history-reference="entry.id"
              tabindex="-1"
              class="reference-card"
            >
              <h4>{{ index + 1 }} · {{ entry.title }}</h4>
              <p>{{ entry.body }}</p>
              <button
                v-if="entry.evidence"
                class="text-link"
                @click="sourcePreview?.open(entry.evidence)"
              >
                Inspect source
              </button>
            </article>
          </section>
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
    </div>
  </main>
  <SourcePreview ref="sourcePreview" return-label="Back to history" />
</template>
