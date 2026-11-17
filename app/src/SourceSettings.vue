<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { sourceApi } from './sourceApi'
import type { SourceSettings } from './generated/SourceSettings'
const settings = ref<SourceSettings>({
  localFolder: '',
  agentHandsPath: '',
  googleAccount: '',
  driveFolder: '',
})
const busy = ref(false),
  error = ref(''),
  status = ref('')
async function save() {
  busy.value = true
  error.value = ''
  status.value = ''
  try {
    await sourceApi.saveSettings({ ...settings.value })
    settings.value = await sourceApi.settings()
    status.value = 'Source settings saved on this device.'
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function detect() {
  error.value = ''
  status.value = ''
  try {
    const path = await sourceApi.installedTool()
    if (path) settings.value.agentHandsPath = path
    else
      error.value =
        'No installed Drive tool found. Enter the Agent Hands use-google-drive/scripts/drive path.'
  } catch (e) {
    error.value = String(e)
  }
}
onMounted(async () => {
  try {
    settings.value = await sourceApi.settings()
  } catch (e) {
    error.value = String(e)
  }
})
</script>
<template>
  <section class="settings-card source-settings" aria-label="Source settings">
    <h2>Sources</h2>
    <p>Import transcripts or notes from a local folder or your existing Agent Hands connection.</p>
    <form @submit.prevent="save">
      <fieldset :disabled="busy">
        <label
          >Local source folder<input
            v-model="settings.localFolder"
            aria-label="Local source folder"
            placeholder="/path/to/meeting-exports"
        /></label>
        <p class="muted">
          Read TXT, Markdown, SRT and VTT files directly inside this folder. Synced Google Doc
          shortcuts need the Drive connection below.
        </p>
        <h3>Meet transcripts through Agent Hands</h3>
        <label
          >Agent Hands Drive tool<input
            v-model="settings.agentHandsPath"
            aria-label="Agent Hands Drive tool"
            placeholder="/path/to/agent-hands/use-google-drive/scripts/drive"
        /></label>
        <button type="button" class="text-link" @click="detect">Find installed tool</button>
        <label
          >Google account alias<input
            v-model="settings.googleAccount"
            aria-label="Google account alias"
            placeholder="The alias already configured in Agent Hands"
        /></label>
        <label
          >Meet folder link or ID <span class="muted">(optional)</span
          ><input
            v-model="settings.driveFolder"
            aria-label="Meet folder link or ID"
            placeholder="https://drive.google.com/drive/folders/…"
        /></label>
        <p class="muted">
          With a folder set, imports are limited to documents inside it and its meeting subfolders.
          Leave it blank to import by a specific Google Doc link. Agent Hands manages your existing
          sign-in; this app only lists and exports files.
        </p>
        <button type="submit" class="primary">
          {{ busy ? 'Saving…' : 'Save source settings' }}
        </button>
      </fieldset>
    </form>
    <p v-if="error" class="review-error" role="alert">{{ error }}</p>
    <p v-if="status" role="status">{{ status }}</p>
  </section>
</template>
