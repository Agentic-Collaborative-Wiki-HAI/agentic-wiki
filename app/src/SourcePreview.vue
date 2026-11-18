<script setup lang="ts">
import { nextTick, ref } from 'vue'
import type { Evidence } from './generated/Evidence'
const props = withDefaults(defineProps<{ returnLabel?: string }>(), { returnLabel: 'Back' })
const dialog = ref<HTMLDialogElement>(),
  evidence = ref<Evidence | null>(null)
async function open(source: Evidence) {
  evidence.value = source
  await nextTick()
  dialog.value?.showModal()
}
defineExpose({ open })
</script>
<template>
  <dialog ref="dialog" class="evidence-dialog source-preview" aria-label="Source evidence">
    <template v-if="evidence">
      <p class="eyebrow">Source evidence · Revision {{ evidence.source.revision }}</p>
      <h2>{{ evidence.title }}</h2>
      <blockquote>{{ evidence.quote }}</blockquote>
      <p class="muted">Saved source excerpt. Check whether it supports the answer.</p>
      <button class="primary" @click="dialog?.close()">{{ props.returnLabel }}</button>
    </template>
  </dialog>
</template>
