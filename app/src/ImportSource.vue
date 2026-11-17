<script setup lang="ts">
import { nextTick, ref } from 'vue'
import { sourceApi } from './sourceApi'
import type { SourceProvider } from './generated/SourceProvider'
import type { SourceListing } from './generated/SourceListing'
import type { ImportPreview } from './generated/ImportPreview'
import type { ImportKind } from './generated/ImportKind'
import type { Page } from './generated/Page'
const props = defineProps<{ page: Page | null }>()
const emit = defineEmits<{ imported: [page: Page]; settings: [] }>()
const dialog = ref<HTMLDialogElement>(),
  provider = ref<SourceProvider>('local'),
  listing = ref<SourceListing | null>(null),
  preview = ref<ImportPreview | null>(null)
const busy = ref(false),
  error = ref(''),
  title = ref(''),
  kind = ref<ImportKind | ''>(''),
  linkToPage = ref(false),
  documentLink = ref('')
const folders = ref<{ id: string; title: string }[]>([])
function modifiedLabel(value: string) {
  const seconds = /^Unix time (\d+)$/.exec(value)
  const date = seconds ? new Date(Number(seconds[1]) * 1000) : new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString()
}
let generation = 0
async function discardPreview() {
  const token = preview.value?.token
  preview.value = null
  if (token) await sourceApi.cancel(token)
}
async function open() {
  generation++
  error.value = ''
  listing.value = null
  folders.value = []
  busy.value = false
  linkToPage.value = false
  await discardPreview()
  dialog.value?.showModal()
  await nextTick()
}
async function close() {
  generation++
  busy.value = false
  dialog.value?.close()
  await discardPreview()
}
async function changeSource() {
  generation++
  busy.value = false
  error.value = ''
  listing.value = null
  folders.value = []
  await discardPreview()
}
async function list(folderId: string | null = null) {
  const seq = ++generation
  busy.value = true
  error.value = ''
  try {
    const result = await sourceApi.list(provider.value, folderId)
    if (seq !== generation) return false
    listing.value = result
    return true
  } catch (e) {
    if (seq === generation) error.value = String(e)
    return false
  } finally {
    if (seq === generation) busy.value = false
  }
}
async function enterFolder(id: string, title: string) {
  if (await list(id)) folders.value.push({ id, title })
}
async function parentFolder() {
  const remaining = folders.value.slice(0, -1)
  if (await list(remaining.at(-1)?.id ?? null)) folders.value = remaining
}
async function inspect(key: string) {
  const seq = ++generation
  busy.value = true
  error.value = ''
  try {
    await discardPreview()
    const result = await sourceApi.preview(provider.value, key)
    if (seq !== generation) {
      await sourceApi.cancel(result.token)
      return
    }
    preview.value = result
    title.value = result.title
    kind.value = result.existingKind ?? ''
  } catch (e) {
    if (seq === generation) error.value = String(e)
  } finally {
    if (seq === generation) busy.value = false
  }
}
async function confirm() {
  if (!preview.value || !kind.value) return
  busy.value = true
  error.value = ''
  try {
    const page = await sourceApi.confirm(
      preview.value.token,
      title.value,
      kind.value,
      linkToPage.value ? (props.page?.id ?? null) : null,
    )
    await close()
    emit('imported', page)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function settings() {
  await close()
  emit('settings')
}
defineExpose({ open })
</script>
<template>
  <dialog
    ref="dialog"
    class="import-dialog"
    aria-labelledby="import-title"
    @cancel.prevent="busy && preview ? undefined : close()"
  >
    <div class="import-heading">
      <h2 id="import-title">Import a transcript or notes</h2>
      <button :disabled="busy && !!preview" @click="close" aria-label="Close import">Close</button>
    </div>
    <p v-if="error" class="review-error" role="alert">{{ error }}</p>
    <template v-if="!preview">
      <p>Choose a source to preview. Nothing is added to the wiki until you confirm.</p>
      <label class="source-choice"
        >Source<select
          v-model="provider"
          aria-label="Import source"
          :disabled="busy"
          @change="changeSource"
        >
          <option value="local">Local source folder</option>
          <option value="drive">Meet / Google Drive</option>
        </select></label
      >
      <div class="import-controls">
        <button class="primary" :disabled="busy" @click="list(folders.at(-1)?.id ?? null)">
          {{ busy ? 'Reading source…' : 'Browse source folder' }}</button><button
          class="text-link"
          :disabled="busy"
          @click="settings"
        >
          Source settings
        </button>
      </div>
      <form
        v-if="provider === 'drive'"
        class="import-direct"
        @submit.prevent="inspect(documentLink)"
      >
        <label
          >Google Doc link or ID<input
            v-model="documentLink"
            aria-label="Google Doc link or ID" /></label
        ><button :disabled="busy || !documentLink.trim()">Preview document</button>
      </form>
      <nav v-if="folders.length" class="import-controls" aria-label="Meet folder navigation">
        <button :disabled="busy" @click="parentFolder">Back one folder</button
        ><span>{{ folders.map((f) => f.title).join(' / ') }}</span>
      </nav>
      <template v-if="listing"
        ><p class="muted">{{ listing.notice }}</p>
        <ul class="source-list">
          <li v-for="item in listing.items" :key="item.key">
            <button
              :disabled="busy || !item.supported"
              @click="item.folder ? enterFolder(item.key, item.title) : inspect(item.key)"
            >
              {{ item.folder ? '▸ ' : '' }}{{ item.title }}</button
            ><span v-if="!item.supported" class="muted">Format not supported yet</span>
          </li>
        </ul>
        <p v-if="!listing.items.length">No matching documents in this folder.</p></template
      >
    </template>
    <template v-else>
      <p class="muted">
        {{ preview.provider === 'drive' ? 'Google Docs export' : 'Local file' }} ·
        {{ preview.sourceName }} · {{ preview.byteCount.toLocaleString() }} bytes
      </p>
      <p v-if="preview.sourceUrl" class="import-source-url">{{ preview.sourceUrl }}</p>
      <p v-if="preview.sourceModified" class="muted">
        Source modified: {{ modifiedLabel(preview.sourceModified) }}
      </p>
      <p v-if="preview.unchanged" role="status">
        This exact source text is already imported. Open its current copy; no duplicate or new
        revision will be created.
      </p>
      <p v-else-if="preview.existingId" class="status">
        This source has changed. Updating the imported copy creates a new revision; previous text
        and citations remain available in history. Any local edits to the current copy will be
        replaced by the preview below.
      </p>
      <form @submit.prevent="confirm">
        <fieldset :disabled="busy">
          <label
            >Title<input
              v-model="title"
              aria-label="Import title"
              maxlength="200"
              :disabled="preview.unchanged"
          /></label>
          <label
            >What does this document contain?<select
              v-model="kind"
              aria-label="Imported content type"
              :disabled="!!preview.existingId"
            >
              <option disabled value="">Choose a type</option>
              <option value="transcript">Transcript</option>
              <option value="notes">Notes or summary</option>
            </select></label
          >
          <p class="muted">
            Choose Transcript for spoken content only. For a document with summaries or action
            items, choose Notes or summary; any transcript stays included.
          </p>
          <label v-if="!preview.existingId && page" class="import-parent"
            ><input v-model="linkToPage" type="checkbox" />Link as a source for “{{
              page.title
            }}”</label
          >
          <p v-if="linkToPage && !preview.existingId" class="muted">
            Linked source excerpts can be included when you explicitly ask @agent on that page.
            Importing alone does not send anything to the model.
          </p>
          <pre class="import-preview" aria-label="Source text preview" tabindex="0">{{
            preview.text
          }}</pre>
          <p class="muted">
            Text, timestamps and speaker labels are kept as written.
            <template v-if="preview.provider === 'drive'"
              >The entire Google Doc is imported as plain text, including its tabs.</template
            >
            <template v-else>Rich formatting is not imported.</template>
          </p>
          <div class="import-controls">
            <button type="button" @click="discardPreview">Back</button
            ><button type="submit" class="primary" :disabled="!kind || !title.trim()">
              {{
                busy
                  ? 'Saving…'
                  : preview.unchanged
                    ? 'Open existing copy'
                    : preview.existingId
                      ? 'Update imported copy'
                      : 'Add artifact'
              }}
            </button>
          </div>
        </fieldset>
      </form>
    </template>
  </dialog>
</template>
