<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { authoringApi } from './authoringApi'
import type { AgentInstructions } from './generated/AgentInstructions'
const instructions = ref<AgentInstructions | null>(null),
  defaults = ref<AgentInstructions | null>(null)
const error = ref(''),
  status = ref(''),
  busy = ref(false)
onMounted(async () => {
  try {
    ;[instructions.value, defaults.value] = await Promise.all([
      authoringApi.instructions(),
      authoringApi.defaults(),
    ])
  } catch (e) {
    error.value = String(e)
  }
})
async function save() {
  if (!instructions.value) return
  busy.value = true
  error.value = ''
  status.value = ''
  try {
    await authoringApi.saveInstructions({ ...instructions.value })
    status.value = 'Agent instructions saved for future requests.'
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
function restoreDefaults() {
  instructions.value = defaults.value && { ...defaults.value }
  status.value = ''
}
</script>
<template>
  <section class="settings-card agent-instructions">
    <h2>Agent instructions</h2>
    <p>
      Customize the instructions included with each request. Changes apply to this workspace’s
      future requests.
    </p>
    <p v-if="error" role="alert">{{ error }}</p>
    <p v-if="status" role="status">{{ status }}</p>
    <form v-if="instructions" @submit.prevent="save">
      <label
        >Passage editing requests<textarea
          v-model="instructions.proposal"
          aria-label="Passage editing instructions"
          required
          maxlength="12000"
          rows="7"
        />
      </label>
      <label
        >Global Chat<textarea
          v-model="instructions.chat"
          aria-label="Global Chat instructions"
          required
          maxlength="12000"
          rows="7"
        />
      </label>
      <div class="review-actions">
        <button type="button" @click="restoreDefaults">Restore defaults</button
        ><button class="primary" :disabled="busy">Save instructions</button>
      </div>
    </form>
    <details>
      <summary>Application rules</summary>
      <p>
        The app also requires a structured response, treats quoted material as context rather than
        instructions, and restricts citations to supplied excerpts. The agent has no tools or file
        access through this connection. It cannot publish changes: suggested edits still need your
        acceptance.
      </p>
    </details>
  </section>
</template>
