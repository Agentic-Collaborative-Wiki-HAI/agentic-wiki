<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from 'vue'
import BackupSettings from './BackupSettings.vue'
import SourceSettings from './SourceSettings.vue'
import { reviewApi } from './reviewApi'
import { connection, models, model, agentBusy, refreshConnection } from './subscriptionState'
const emit = defineEmits<{ back: [] }>()
const error = ref(''),
  working = ref(false)
let poll: ReturnType<typeof setInterval> | undefined
async function refresh() {
  try {
    await refreshConnection()
    if (!connection.value?.waiting && poll) {
      clearInterval(poll)
      poll = undefined
    }
  } catch (e) {
    error.value = String(e)
  }
}
async function signIn() {
  working.value = true
  error.value = ''
  try {
    await reviewApi.signIn()
    await refresh()
    if (connection.value?.waiting && !poll) poll = setInterval(() => void refresh(), 1000)
  } catch (e) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}
async function cancelSignIn() {
  try {
    await reviewApi.cancelSignIn()
    await refresh()
  } catch (e) {
    error.value = String(e)
  }
}
onMounted(async () => {
  await refresh()
  if (connection.value?.waiting) poll = setInterval(() => void refresh(), 1000)
})
onBeforeUnmount(() => {
  if (poll) clearInterval(poll)
})
</script>
<template>
  <main class="settings-page" aria-label="Workspace settings">
    <button class="text-link" @click="emit('back')">Back to page</button>
    <nav class="settings-breadcrumb" aria-label="Settings breadcrumb">
      Settings / Connections and sources
    </nav>
    <h1>Settings</h1>
    <p>Connect your ChatGPT subscription to use the agent in this workspace.</p>
    <p v-if="error" class="review-error" role="alert">{{ error }}</p>
    <section class="settings-card">
      <h2>ChatGPT</h2>
      <p role="status">{{ connection?.message ?? 'Checking connection…' }}</p>
      <p v-if="connection?.connected && !connection.waiting">
        Connected{{ connection.account ? ` · ${connection.account}` : '' }}
      </p>
      <button v-if="connection?.waiting" class="compact" @click="cancelSignIn">
        Cancel sign-in
      </button>
      <button v-else class="primary" :disabled="working || agentBusy" @click="signIn">
        {{ connection?.connected ? 'Reconnect ChatGPT' : 'Continue with ChatGPT' }}
      </button>
      <label v-if="models.length" class="model-setting"
        >Suggestion model
        <select v-model="model" aria-label="Suggestion model" :disabled="agentBusy">
          <option v-for="m in models" :key="m.slug" :value="m.slug">{{ m.displayName }}</option>
        </select>
      </label>
      <p class="muted">
        Requests use your subscription’s limits. There is no automatic switch to paid API usage.
      </p>
    </section>
    <SourceSettings />
    <BackupSettings />
    <section>
      <h2>When the agent responds</h2>
      <p>
        Write <span class="mention">@agent</span> in a passage’s comment thread and send your reply.
        Ordinary comments do not start an agent request.
      </p>
      <p>
        The agent receives your request, the selected passage, linked source excerpts, and the
        previous suggestion when you ask for a revision. You can inspect that context in the thread
        before sending.
      </p>
      <p>Suggested edits stay pending until you accept or reject them.</p>
    </section>
  </main>
</template>
