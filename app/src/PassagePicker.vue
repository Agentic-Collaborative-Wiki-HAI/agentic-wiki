<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { workspaceApi } from './workspaceApi'
import type { Evidence } from './generated/Evidence'
import type { PassageResult } from './generated/PassageResult'
const emit = defineEmits<{ navigate: [PassageResult]; cite: [Evidence]; evidence: [Evidence[]] }>()
const dialog = ref<HTMLDialogElement>(),
  mode = ref<'navigate' | 'cite' | 'evidence'>('navigate')
const query = ref(''),
  linked = ref<string[]>([]),
  others = ref(false),
  results = ref<PassageResult[]>([])
const preview = ref<Evidence | null>(null),
  selected = ref<Evidence[]>([]),
  error = ref(''),
  busy = ref(false),
  opened = ref(false)
let sequence = 0,
  timer: ReturnType<typeof setTimeout> | undefined
async function search() {
  const seq = ++sequence
  busy.value = true
  error.value = ''
  try {
    const hits = await workspaceApi.search(
      query.value,
      mode.value === 'evidence' && !others.value ? linked.value : null,
    )
    if (seq === sequence) results.value = hits
  } catch (e) {
    if (seq === sequence) error.value = String(e)
  } finally {
    if (seq === sequence) busy.value = false
  }
}
watch([query, others], () => {
  if (!opened.value) return
  clearTimeout(timer)
  ++sequence
  busy.value = true
  timer = setTimeout(search, 200)
})
async function open(
  kind: typeof mode.value,
  context: { linkedIds?: string[]; selected?: Evidence[] } = {},
) {
  const keepSearch = kind === 'navigate' && mode.value === 'navigate'
  mode.value = kind
  selected.value = [...(context.selected ?? [])]
  linked.value = context.linkedIds ?? []
  others.value = false
  if (!keepSearch) query.value = ''
  preview.value = null
  opened.value = true
  await nextTick()
  dialog.value?.showModal()
  await search()
}
function navigate(hit: PassageResult) {
  emit('navigate', hit)
  close()
}
function confirmSources() {
  emit('evidence', selected.value)
  close()
}
function close() {
  opened.value = false
  ++sequence
  clearTimeout(timer)
  dialog.value?.close()
}
async function inspect(hit: PassageResult) {
  try {
    preview.value = await workspaceApi.evidence(hit.source)
  } catch (e) {
    error.value = String(e)
  }
}
function add() {
  if (!preview.value) return
  if (mode.value === 'cite') {
    emit('cite', preview.value)
    close()
    return
  }
  if (selected.value.some((e) => e.id === preview.value!.id)) return
  if (selected.value.length >= 8) {
    error.value = 'Choose up to eight excerpts. Remove one to add another.'
    return
  }
  selected.value.push(preview.value)
  preview.value = null
}
defineExpose({ open })
onBeforeUnmount(() => {
  ++sequence
  clearTimeout(timer)
})
</script>
<template>
  <dialog
    ref="dialog"
    class="workspace-dialog passage-picker"
    aria-labelledby="picker-title"
    @cancel="close"
  >
    <div class="picker-heading">
      <h2 id="picker-title">
        {{
          mode === 'navigate'
            ? 'Search wiki'
            : mode === 'cite'
              ? 'Add citation'
              : 'Choose sources for @agent'
        }}
      </h2>
      <button @click="close" aria-label="Close search">Close</button>
    </div>
    <p>Search saved pages and imported documents on this device.</p>
    <label class="picker-query"
      >Search passages<input
        v-model="query"
        aria-label="Search passages"
        placeholder="Try booking instructions…"
        maxlength="1000"
    /></label>
    <label v-if="mode === 'evidence'" class="picker-scope"
      ><input v-model="others" type="checkbox" />Include other workspace pages</label
    >
    <p v-if="mode === 'evidence' && !others" class="muted">
      Showing sources linked to this page. Select the checkbox to choose other sources explicitly.
    </p>
    <p v-if="error" role="alert" class="review-error">{{ error }}</p>
    <p v-if="busy" role="status">Searching…</p>
    <div v-else class="passage-results">
      <p v-if="!results.length">
        No matching passages{{ mode === 'evidence' && !others ? ' in linked sources' : '' }}.
      </p>
      <article v-for="(hit, index) in results" :key="index">
        <p class="muted">{{ hit.breadcrumb.join(' / ') }} · Revision {{ hit.source.revision }}</p>
        <p>{{ hit.snippet }}</p>
        <button v-if="mode === 'navigate'" class="text-link" @click="navigate(hit)">
          Open passage
        </button>
        <button v-else class="text-link" @click="inspect(hit)">Inspect excerpt</button>
      </article>
      <p v-if="results.length === 60" class="muted">
        First 60 matches. Narrow your search to find a specific passage.
      </p>
    </div>
    <section v-if="preview" class="excerpt-preview">
      <h3>{{ preview.title }} · Revision {{ preview.source.revision }}</h3>
      <blockquote>{{ preview.quote }}</blockquote>
      <button class="primary" @click="add">
        {{ mode === 'cite' ? 'Cite this excerpt' : 'Use this excerpt' }}
      </button>
    </section>
    <section v-if="mode === 'evidence'">
      <h3>Selected excerpts ({{ selected.length }}/8)</h3>
      <article v-for="e in selected" :key="e.id">
        <strong>{{ e.title }} · Revision {{ e.source.revision }}</strong>
        <p>{{ e.quote.slice(0, 240) }}</p>
        <button class="text-link" @click="selected = selected.filter((s) => s.id !== e.id)">
          Remove excerpt
        </button>
      </article>
      <button class="primary" @click="confirmSources">Use selected sources</button>
    </section>
  </dialog>
</template>
