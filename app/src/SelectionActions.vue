<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import type { Editor } from '@tiptap/core'
const props = defineProps<{ editor: Editor; enabled: boolean }>()
const emit = defineEmits<{ comment: [] }>()
const popup = ref<HTMLElement>()
const position = ref<{ left: number; top: number } | null>(null)
let frame = 0,
  selecting = false,
  dismissed = false
function hide() {
  dismissed = true
  position.value = null
}
function update() {
  frame = 0
  const selection = window.getSelection()
  const view = props.editor.view
  const { $from, $to, empty } = props.editor.state.selection
  if (
    !props.enabled ||
    dismissed ||
    selecting ||
    empty ||
    !$from.sameParent($to) ||
    $from.parent.type.name !== 'paragraph' ||
    !selection?.rangeCount ||
    selection.isCollapsed ||
    !view.dom.contains(selection.anchorNode) ||
    !view.dom.contains(selection.focusNode)
  ) {
    position.value = null
    return
  }
  const workspace = view.dom.closest('.workspace')?.getBoundingClientRect()
  const article = view.dom.getBoundingClientRect()
  const top = Math.max(8, workspace?.top ?? 0)
  const bottom = Math.min(window.innerHeight - 8, workspace?.bottom ?? window.innerHeight)
  const left = Math.max(8, article.left)
  const right = Math.min(window.innerWidth - 8, article.right)
  const rectangles = [...selection.getRangeAt(0).getClientRects()]
  const anchor = rectangles
    .reverse()
    .find(
      (r) => r.height > 0 && r.top >= top && r.bottom <= bottom && r.right > left && r.left < right,
    )
  if (!anchor) {
    position.value = null
    return
  }
  const width = popup.value?.offsetWidth ?? 96
  const height = popup.value?.offsetHeight ?? 38
  const y = anchor.top - height - 8 >= top ? anchor.top - height - 8 : anchor.bottom + 8
  if (y + height > bottom) {
    position.value = null
    return
  }
  position.value = {
    left: Math.max(
      left,
      Math.min(
        right - width,
        (Math.max(left, anchor.left) + Math.min(right, anchor.right) - width) / 2,
      ),
    ),
    top: y,
  }
}
function schedule() {
  if (!frame) frame = requestAnimationFrame(update)
}
function selectionUpdated() {
  dismissed = false
  schedule()
}
function pointerDown(event: PointerEvent) {
  if (popup.value?.contains(event.target as Node)) return
  selecting = props.editor.view.dom.contains(event.target as Node)
  hide()
}
function pointerUp() {
  if (selecting) dismissed = false
  selecting = false
  schedule()
}
function keyDown(event: KeyboardEvent) {
  if (event.key === 'Escape') hide()
}
function comment() {
  hide()
  emit('comment')
}
watch(
  () => props.editor,
  (editor, old) => {
    old?.off('selectionUpdate', selectionUpdated)
    editor.on('selectionUpdate', selectionUpdated)
    dismissed = false
    schedule()
  },
  { immediate: true },
)
watch(() => props.enabled, schedule)
watch(
  () => !!position.value,
  async (visible) => {
    if (visible) {
      await nextTick()
      update()
    }
  },
)
let observer: ResizeObserver | undefined
onMounted(() => {
  document.addEventListener('selectionchange', schedule)
  document.addEventListener('pointerdown', pointerDown)
  document.addEventListener('pointerup', pointerUp)
  document.addEventListener('keydown', keyDown)
  window.addEventListener('scroll', schedule, true)
  window.addEventListener('resize', schedule)
  observer = new ResizeObserver(schedule)
  const workspace = props.editor.view.dom.closest('.workspace')
  if (workspace) observer.observe(workspace)
})
onBeforeUnmount(() => {
  props.editor.off('selectionUpdate', selectionUpdated)
  cancelAnimationFrame(frame)
  observer?.disconnect()
  document.removeEventListener('selectionchange', schedule)
  document.removeEventListener('pointerdown', pointerDown)
  document.removeEventListener('pointerup', pointerUp)
  document.removeEventListener('keydown', keyDown)
  window.removeEventListener('scroll', schedule, true)
  window.removeEventListener('resize', schedule)
})
</script>
<template>
  <Teleport to="body">
    <div
      v-if="position"
      ref="popup"
      class="passage-actions"
      role="group"
      aria-label="Selected text actions"
      :style="{ left: position.left + 'px', top: position.top + 'px' }"
    >
      <button class="compact" @mousedown.prevent @click="comment">Comment</button>
    </div>
  </Teleport>
</template>
