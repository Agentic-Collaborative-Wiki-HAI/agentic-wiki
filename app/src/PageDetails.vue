<script setup lang="ts">
import { computed, nextTick, ref } from 'vue'
import type { WikiRecord } from './generated/WikiRecord'
import type { Page } from './generated/Page'
import { workspaceApi } from './workspaceApi'
import { ancestors } from './wikiTree'
const props = defineProps<{ records: WikiRecord[] }>()
const emit = defineEmits<{ saved: [Page, boolean] }>()
const dialog = ref<HTMLDialogElement>(),
  title = ref(''),
  parent = ref(''),
  error = ref(''),
  busy = ref(false)
const target = ref<WikiRecord | null>(null),
  requestId = ref('')
const parents = computed(() =>
  props.records.filter(
    (r) => !target.value || !ancestors(props.records, r.id).some((a) => a.id === target.value!.id),
  ),
)
async function open(record: WikiRecord | null, parentId: string | null = null) {
  target.value = record
  title.value = record?.title ?? ''
  parent.value = record?.parentId ?? parentId ?? ''
  requestId.value = crypto.randomUUID()
  error.value = ''
  await nextTick()
  dialog.value?.showModal()
}
async function save() {
  busy.value = true
  try {
    const page = target.value
      ? await workspaceApi.organize(
          target.value.id,
          target.value.metadataVersion,
          title.value,
          parent.value || null,
        )
      : await workspaceApi.create(requestId.value, title.value, parent.value || null)
    dialog.value?.close()
    emit('saved', page, !target.value)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
defineExpose({ open })
</script>
<template>
  <dialog
    ref="dialog"
    class="workspace-dialog"
    aria-label="Add page"
    aria-labelledby="page-details-title"
    @cancel="busy && $event.preventDefault()"
  >
    <form @submit.prevent="save">
      <h2 id="page-details-title">{{ target ? 'Rename or move page' : 'Add page' }}</h2>
      <p v-if="error" role="alert" class="review-error">{{ error }}</p>
      <label
        >Page title<input
          v-model="title"
          aria-label="Page title"
          required
          maxlength="200"
          :disabled="busy"
      /></label>
      <label
        >Inside<select v-model="parent" aria-label="Parent page" :disabled="busy">
          <option value="">Wiki root</option>
          <option v-for="r in parents" :key="r.id" :value="r.id">
            {{
              ancestors(records, r.id)
                .map((a) => a.title)
                .join(' / ')
            }}
          </option>
        </select></label
      >
      <p v-if="target" class="muted">
        Links, citations and page history stay attached to this page.
      </p>
      <div class="review-actions">
        <button type="button" :disabled="busy" @click="dialog?.close()">Cancel</button
        ><button class="primary" :disabled="busy || !title.trim()">
          {{ target ? 'Save details' : 'Create page' }}
        </button>
      </div>
    </form>
  </dialog>
</template>
