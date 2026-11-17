<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { WikiRecord } from './generated/WikiRecord'
import { ancestors, visibleTree } from './wikiTree'
const props = defineProps<{ records: WikiRecord[]; active?: string }>()
const emit = defineEmits<{ open: [string] }>()
const collapsed = ref(new Set<string>())
const rows = computed(() => visibleTree(props.records, collapsed.value))
watch(
  () => [props.active, props.records],
  () => {
    for (const r of ancestors(props.records, props.active ?? '').slice(0, -1))
      collapsed.value.delete(r.id)
  },
)
</script>
<template>
  <div
    v-for="row in rows"
    :key="row.record.id"
    class="wiki-record"
    :style="{ paddingLeft: `${Math.min(row.depth, 8) * 14}px` }"
    :class="{ selected: active === row.record.id }"
  >
    <button
      v-if="row.children"
      class="tree-expander"
      :aria-label="`Expand or collapse ${row.record.title}`"
      :aria-expanded="!collapsed.has(row.record.id)"
      @click="
        collapsed.has(row.record.id)
          ? collapsed.delete(row.record.id)
          : collapsed.add(row.record.id)
      "
    >
      {{ collapsed.has(row.record.id) ? '›' : '⌄' }}
    </button>
    <span v-else class="tree-expander"></span>
    <button
      class="tree-record-link"
      :aria-current="active === row.record.id ? 'page' : undefined"
      @click="emit('open', row.record.id)"
    >
      {{ row.record.title }}
    </button>
  </div>
</template>
<style scoped>
.wiki-record {
  display: flex;
  align-items: baseline;
  border-radius: 4px;
}
.tree-expander {
  min-width: 24px;
  width: 24px;
}
.tree-record-link {
  text-align: left;
  padding: 12px 8px 12px 0;
  overflow-wrap: anywhere;
  flex: 1;
}
</style>
