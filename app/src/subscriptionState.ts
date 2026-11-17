import { ref, watch } from 'vue'
import { reviewApi } from './reviewApi'
import type { ConnectionStatus } from './generated/ConnectionStatus'
import type { AvailableModel } from './generated/AvailableModel'

// Only the model preference is stored in the webview. Credentials remain in the Rust adapter.
export const connection = ref<ConnectionStatus | null>(null)
export const models = ref<AvailableModel[]>([])
export const model = ref(localStorage.getItem('wiki.suggestion-model') ?? '')
export const agentBusy = ref(false)
watch(model, (value) => localStorage.setItem('wiki.suggestion-model', value))
let refreshing: Promise<void> | undefined
export function refreshConnection() {
  if (refreshing) return refreshing
  refreshing = (async () => {
    connection.value = await reviewApi.connection()
    if (connection.value.connected && !connection.value.waiting) {
      if (!models.value.length) models.value = await reviewApi.models()
      if (!models.value.some((m) => m.slug === model.value))
        model.value = models.value[0]?.slug ?? ''
    } else models.value = []
  })().finally(() => {
    refreshing = undefined
  })
  return refreshing
}
