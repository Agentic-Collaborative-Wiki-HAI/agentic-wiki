<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { workspaceApi } from './workspaceApi'
import type { WorkspaceCopy } from './generated/WorkspaceCopy'
const destination = ref(
    `~/Downloads/agentic-wiki-${new Date().toISOString().slice(0, 10)}.wiki-backup`,
  ),
  source = ref('')
const copies = ref<WorkspaceCopy[]>([]),
  error = ref(''),
  message = ref(''),
  busy = ref(false)
async function run(fn: () => Promise<void>) {
  busy.value = true
  error.value = ''
  message.value = ''
  try {
    await fn()
    copies.value = await workspaceApi.copies()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function backup() {
  await run(async () => {
    message.value = `Backup saved: ${await workspaceApi.export(destination.value)}`
  })
}
async function restore() {
  await run(async () => {
    const copy = await workspaceApi.restore(source.value)
    message.value = `${copy.label}. Select it below to open after restarting. Your current workspace is unchanged.`
  })
}
async function select(id: string) {
  await run(async () => {
    await workspaceApi.select(id)
    message.value =
      'Close and reopen the app to use the selected workspace. Finish any current edits first.'
  })
}
onMounted(() => run(async () => {}))
</script>
<template>
  <section class="settings-card backup-settings">
    <h2>Backup and restore</h2>
    <p>
      Save pages, source originals, citations, discussions, suggestions, drafts and history. Backups
      contain your private content; keep them somewhere you trust.
    </p>
    <p class="muted">
      ChatGPT credentials and connection settings stay on this device. Reconnect services after
      opening a restored copy.
    </p>
    <p v-if="error" role="alert" class="review-error">{{ error }}</p>
    <p v-if="message" role="status">{{ message }}</p>
    <form @submit.prevent="backup">
      <label
        >New backup file<input
          v-model="destination"
          aria-label="Backup destination"
          required
          :disabled="busy" /></label
      ><button class="primary" :disabled="busy">Create backup</button>
    </form>
    <form @submit.prevent="restore">
      <label
        >Existing backup file<input
          v-model="source"
          aria-label="Restore file"
          placeholder="~/Downloads/my-workspace.wiki-backup"
          required
          :disabled="busy" /></label
      ><button :disabled="busy || !source">Restore as separate copy</button>
    </form>
    <h3>Workspace copies</h3>
    <ul>
      <li v-for="copy in copies" :key="copy.id">
        {{ copy.label }}<strong v-if="copy.active"> · Open now</strong
        ><span v-if="copy.selected"> · Selected for next start</span
        ><button v-else class="text-link" :disabled="busy" @click="select(copy.id)">
          Use after restarting
        </button>
      </li>
    </ul>
  </section>
</template>
