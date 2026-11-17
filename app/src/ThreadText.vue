<script setup lang="ts">
import { computed } from 'vue'
import { messageParts } from './threadText'
const props = defineProps<{ text: string }>()
const parts = computed(() => messageParts(props.text))
</script>
<template>
  <span v-for="(part, i) in parts" :key="i" :class="{ mention: part.mention }">{{
    part.text
  }}</span>
</template>
