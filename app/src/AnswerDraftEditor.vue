<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { wikiExtensions, hasUnsupportedPaste } from './editor'
import { chatApi } from './chatApi'
import SourcePreview from './SourcePreview.vue'
import type { AnswerDraft } from './generated/AnswerDraft'
import type { WikiRecord } from './generated/WikiRecord'
import type { RichDocument } from './generated/RichDocument'
import type { Page } from './generated/Page'
const props = defineProps<{ draft: AnswerDraft; records: WikiRecord[] }>()
const emit = defineEmits<{ close: []; published: [Page] }>()
const editor = shallowRef<Editor>(),
  title = ref(props.draft.title),
  parent = ref(props.draft.parentId ?? ''),
  version = ref(props.draft.version)
const error = ref(''),
  status = ref('Draft saved on this device'),
  busy = ref(false),
  evidence = ref(props.draft.evidence),
  preview = ref<InstanceType<typeof SourcePreview>>()
let timer: ReturnType<typeof setTimeout> | undefined,
  chain = Promise.resolve(),
  dirty = false
function changed() {
  dirty = true
  status.value = 'Saving draft…'
  clearTimeout(timer)
  timer = setTimeout(() => void persist().catch(() => {}), 350)
}
function persist() {
  clearTimeout(timer)
  if (!editor.value || !dirty) return chain
  const name = title.value,
    parentId = parent.value || null,
    document = { schemaVersion: 1, doc: editor.value.getJSON() } as unknown as RichDocument
  dirty = false
  chain = chain
    .catch(() => {})
    .then(async () => {
      try {
        const saved = await chatApi.saveDraft(
          props.draft.id,
          version.value,
          name,
          parentId,
          document,
        )
        version.value = saved.version
        evidence.value = saved.evidence
        status.value = 'Draft saved on this device'
        error.value = ''
      } catch (e) {
        dirty = true
        status.value = 'Draft not saved'
        error.value = String(e)
        throw e
      }
    })
  return chain
}
async function close() {
  busy.value = true
  try {
    await persist()
    emit('close')
  } catch {
  } finally {
    busy.value = false
  }
}
async function publish() {
  busy.value = true
  try {
    await persist()
    const p = await chatApi.publish(props.draft.id, version.value)
    emit('published', p)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
function citationClick(event: MouseEvent) {
  const id = (event.target as HTMLElement).closest<HTMLElement>('[data-citation-id]')?.dataset
    .citationId
  const e = evidence.value.find((e) => e.id === id)
  if (e) void preview.value?.open(e)
}
watch([title, parent], changed)
watch(busy, (value) => editor.value?.setEditable(!value))
onMounted(() => {
  editor.value = new Editor({
    extensions: wikiExtensions,
    content: props.draft.document.doc,
    editorProps: {
      attributes: { 'aria-label': 'Draft page content', role: 'textbox', 'aria-multiline': 'true' },
      handlePaste: (_view, event) => {
        if (!hasUnsupportedPaste(event.clipboardData?.getData('text/html') ?? '')) return false
        error.value = 'Paste lists, tables or images as plain text for now.'
        return true
      },
    },
    onUpdate: changed,
  })
})
onBeforeUnmount(() => {
  clearTimeout(timer)
  editor.value?.destroy()
})
</script>
<template>
  <main class="answer-draft-page" aria-label="Draft page from Chat">
    <div class="answer-draft-heading">
      <h1>Draft page</h1>
      <div>
        <button :disabled="busy" @click="close">Keep draft and close</button
        ><button class="primary publish-answer" :disabled="busy || !title.trim()" @click="publish">
          Save page
        </button>
      </div>
    </div>
    <p class="metadata">AI-generated from a Chat answer · {{ draft.model }} · Not published</p>
    <p>Review the wording and its sources before adding this page to the wiki.</p>
    <p v-if="error" class="review-error" role="alert">{{ error }}</p>
    <div class="draft-fields">
      <label>Page title<input v-model="title" maxlength="200" :disabled="busy" /></label
      ><label
        >Place under<select v-model="parent" :disabled="busy">
          <option value="">Wiki root</option>
          <option v-for="r in records" :key="r.id" :value="r.id">{{ r.title }}</option>
        </select></label
      >
    </div>
    <div class="draft-body-grid">
      <section class="draft-body">
        <div class="editor-toolbar">
          <button @click="editor?.chain().focus().toggleMark('bold').run()">Bold</button
          ><button @click="editor?.chain().focus().toggleMark('italic').run()">Italic</button
          ><span class="draft-status" role="status">{{ status }}</span>
        </div>
        <EditorContent v-if="editor" :editor="editor" class="editing" @click="citationClick" />
      </section>
      <aside class="draft-references" aria-label="Draft references">
        <h2>References</h2>
        <article v-for="(e, i) in evidence" :key="e.id">
          <button class="text-link" @click="preview?.open(e)">{{ i + 1 }} · {{ e.title }}</button>
          <p class="muted">Revision {{ e.source.revision }}</p>
          <blockquote>{{ e.quote }}</blockquote>
        </article>
      </aside>
    </div>
  </main>
  <SourcePreview ref="preview" return-label="Back to draft" />
</template>
