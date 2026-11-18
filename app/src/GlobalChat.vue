<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { chatApi } from './chatApi'
import { reviewApi } from './reviewApi'
import { agentBusy, connection, model, refreshConnection } from './subscriptionState'
import PassagePicker from './PassagePicker.vue'
import SourcePreview from './SourcePreview.vue'
import type { Page } from './generated/Page'
import type { Evidence } from './generated/Evidence'
import type { ChatTurn } from './generated/ChatTurn'
import type { AnswerDraft } from './generated/AnswerDraft'
const props = defineProps<{ open: boolean; page: Page | null; canOpenDraft: boolean }>()
const emit = defineEmits<{
  close: []
  settings: []
  import: []
  draft: [AnswerDraft]
  page: [string]
}>()
const turns = ref<ChatTurn[]>([]),
  drafts = ref<AnswerDraft[]>([]),
  question = ref(''),
  error = ref(''),
  needsConnection = ref(false)
const defaults = ref<Evidence[]>([]),
  chosen = ref<Evidence[] | null>(null),
  linked = ref<string[]>([]),
  contextError = ref(''),
  contextBusy = ref(false)
const run = ref<{ id: string; question: string; pageTitle: string } | null>(null),
  working = ref(false)
const sources = computed(() => chosen.value ?? defaults.value)
const picker = ref<InstanceType<typeof PassagePicker>>(),
  preview = ref<InstanceType<typeof SourcePreview>>(),
  composer = ref<HTMLTextAreaElement>(),
  scroller = ref<HTMLElement>()
let sequence = 0,
  timer: ReturnType<typeof setTimeout> | undefined
async function reload() {
  ;[turns.value, drafts.value] = await Promise.all([chatApi.turns(), chatApi.drafts()])
}
async function refreshContext() {
  const id = props.page?.id,
    q = question.value,
    seq = ++sequence
  if (!id) return
  contextBusy.value = true
  contextError.value = ''
  try {
    const c = await chatApi.context(id, q)
    if (seq === sequence) {
      defaults.value = c.evidence
      linked.value = c.linkedIds
    }
  } catch (e) {
    if (seq === sequence) contextError.value = String(e)
  } finally {
    if (seq === sequence) contextBusy.value = false
  }
}
watch(
  () => props.page?.id,
  () => {
    chosen.value = null
    defaults.value = []
    if (props.open) void refreshContext()
  },
)
watch(question, () => {
  clearTimeout(timer)
  ++sequence
  if (props.open && !chosen.value) {
    contextBusy.value = true
    timer = setTimeout(refreshContext, 200)
  }
})
watch(
  () => props.open,
  async (open) => {
    if (!open) return
    try {
      await reload()
      await refreshContext()
      await nextTick()
      composer.value?.focus()
    } catch (e) {
      error.value = String(e)
    }
  },
)
function resetSources() {
  chosen.value = null
  void refreshContext()
}
async function choose() {
  if (contextBusy.value) await refreshContext()
  await picker.value?.open('evidence', { linkedIds: linked.value, selected: sources.value })
}
function evidenceFor(t: ChatTurn) {
  const ids = [...new Set(t.answer?.paragraphs.flatMap((p) => p.citedSourceIds) ?? [])]
  return ids.map((id) => t.evidence.find((e) => e.id === id)!).filter(Boolean)
}
function sourceNumber(t: ChatTurn, id: string) {
  return evidenceFor(t).findIndex((e) => e.id === id) + 1
}
function inspect(t: ChatTurn, id: string) {
  const e = t.evidence.find((e) => e.id === id)
  if (e) void preview.value?.open(e)
}
async function send() {
  if (agentBusy.value || working.value || !question.value.trim() || !props.page) return
  working.value = true
  error.value = ''
  needsConnection.value = false
  const pageId = props.page.id,
    pageTitle = props.page.title,
    text = question.value.trim()
  try {
    await refreshConnection()
    if (!connection.value?.connected || connection.value.waiting || !model.value) {
      needsConnection.value = true
      return
    }
    clearTimeout(timer)
    if (!chosen.value) await refreshContext()
    if (contextError.value && !chosen.value) throw Error(contextError.value)
    if (props.page?.id !== pageId || question.value.trim() !== text)
      throw Error('The page or message changed. Review the sources and send again.')
    const request = {
      id: crypto.randomUUID(),
      pageId,
      question: text,
      sources: sources.value.map((e) => e.source),
      model: model.value,
    }
    run.value = { id: request.id, question: text, pageTitle }
    agentBusy.value = true
    question.value = ''
    await nextTick()
    scroller.value?.scrollTo({ top: scroller.value.scrollHeight })
    await chatApi.ask(request)
  } catch (e) {
    error.value = String(e)
  } finally {
    if (run.value) {
      run.value = null
      agentBusy.value = false
      try {
        await reload()
      } catch (e) {
        error.value = String(e)
      }
      await nextTick()
      scroller.value?.scrollTo({ top: scroller.value.scrollHeight })
    }
    working.value = false
  }
}
async function cancel() {
  if (!run.value) return
  try {
    await reviewApi.cancel(run.value.id)
  } catch (e) {
    error.value = String(e)
  }
}
function retry(t: ChatTurn) {
  question.value = t.request.question
  chosen.value = [...t.evidence]
  needsConnection.value = false
  composer.value?.focus()
}
async function draft(t: ChatTurn) {
  if (!props.canOpenDraft) return
  working.value = true
  try {
    const d = await chatApi.createDraft(t.id)
    await reload()
    if (d.publishedPageId) emit('page', d.publishedPageId)
    else emit('draft', d)
  } catch (e) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}
function draftFor(t: ChatTurn) {
  return drafts.value.find((d) => d.turnId === t.id)
}
onBeforeUnmount(() => {
  clearTimeout(timer)
  ++sequence
})
async function focusTurn(t: ChatTurn) {
  await reload()
  if (!turns.value.some((old) => old.id === t.id)) turns.value.unshift(t)
  await nextTick()
  scroller.value
    ?.querySelector(`[data-turn-id="${CSS.escape(t.id)}"]`)
    ?.scrollIntoView({ block: 'start' })
}
defineExpose({ reload, focusTurn, focus: () => composer.value?.focus() })
</script>
<template>
  <aside
    v-show="open"
    class="chat-drawer"
    aria-label="Agent global chat"
    @keydown.esc.stop="emit('close')"
  >
    <div class="chat-title">
      <h2>Agent global chat</h2>
      <button class="compact" @click="emit('close')">
        <img src="/assets/83771.svg" alt="" />Hide chat
      </button>
    </div>
    <div ref="scroller" class="chat-messages" aria-live="polite" aria-relevant="additions">
      <template v-if="!turns.length && !run"
        ><h3>Work with the wiki</h3>
        <p>Ask a question, request a rewrite, discuss a page, or import a transcript.</p></template
      >
      <p v-if="turns.length === 50" class="muted">Showing the most recent 50 exchanges.</p>
      <article v-for="t in turns" :key="t.id" class="chat-exchange" :data-turn-id="t.id">
        <p class="chat-context-label">
          {{ t.request.pageTitle }} · Revision {{ t.request.pageRevision }}
        </p>
        <p class="chat-question">{{ t.request.question }}</p>
        <section v-if="t.answer" class="chat-answer">
          <p class="eyebrow">Agent · {{ t.model }}</p>
          <p v-for="(p, i) in t.answer.paragraphs" :key="i">
            {{ p.text
            }}<button
              v-for="id in p.citedSourceIds"
              :key="id"
              class="text-link chat-citation"
              :aria-label="`Inspect answer source ${sourceNumber(t, id)}`"
              @click="inspect(t, id)"
            >
              [{{ sourceNumber(t, id) }}]
            </button>
          </p>
          <p v-if="t.answer.missingEvidence" class="chat-missing">{{ t.answer.missingEvidence }}</p>
          <button
            v-if="t.answer.paragraphs.length"
            class="text-link answer-to-draft"
            :disabled="!canOpenDraft || working || agentBusy"
            @click="draft(t)"
          >
            {{
              draftFor(t)?.publishedPageId
                ? 'Open saved page'
                : draftFor(t)
                  ? 'Continue draft'
                  : 'Create draft page'
            }}
          </button>
        </section>
        <p v-else-if="t.status === 'pending'" role="status">Waiting for an answer…</p>
        <div v-else class="review-warning">
          <p>{{ t.error ?? 'No answer was saved.' }}</p>
          <button class="text-link" :disabled="agentBusy" @click="retry(t)">Prepare retry</button>
        </div>
        <details class="chat-context">
          <summary>Context used</summary>
          <p class="muted">
            {{ t.request.history.length }} earlier exchanges were included for conversation context.
            These excerpts were available as context:
          </p>
          <button v-for="e in t.evidence" :key="e.id" class="text-link" @click="preview?.open(e)">
            {{ e.title }} · Revision {{ e.source.revision }}
          </button>
          <p v-if="!t.evidence.length">No source excerpts supplied.</p>
        </details>
      </article>
      <article v-if="run" class="chat-exchange">
        <p class="chat-context-label">{{ run.pageTitle }}</p>
        <p class="chat-question">{{ run.question }}</p>
        <p role="status">Agent is responding…</p>
        <button @click="cancel">Cancel request</button>
      </article>
    </div>
    <div class="chat-composer">
      <p v-if="error" class="review-error" role="alert">{{ error }}</p>
      <p v-if="needsConnection" class="review-warning">
        Connect ChatGPT in Settings to send this message.
        <button class="text-link" @click="emit('settings')">Open Settings</button>
      </p>
      <details class="chat-context">
        <summary>Context</summary>
        <p class="muted">
          {{
            chosen
              ? 'Using your selected excerpts.'
              : 'Current page and linked sources. Choose other saved sources if needed.'
          }}
        </p>
        <p v-if="contextError" role="alert">{{ contextError }}</p>
        <p v-if="contextBusy">Finding relevant excerpts…</p>
        <button v-for="e in sources" :key="e.id" class="text-link" @click="preview?.open(e)">
          {{ e.title }} · Revision {{ e.source.revision }}</button
        ><button class="text-link" :disabled="agentBusy" @click="choose">Choose sources</button
        ><button v-if="chosen" class="text-link" :disabled="agentBusy" @click="resetSources">
          Use current page and linked sources
        </button>
      </details>
      <form @submit.prevent="send">
        <label class="sr-only" for="chat-message">Message Agent</label
        ><textarea
          id="chat-message"
          ref="composer"
          v-model="question"
          placeholder="Message Agent…"
          maxlength="1000"
          :disabled="agentBusy || working"
          @keydown.ctrl.enter.prevent="send"
          @keydown.meta.enter.prevent="send"
        ></textarea>
        <div class="chat-composer-actions">
          <button
            type="button"
            class="text-link"
            :disabled="!canOpenDraft || working"
            @click="emit('import')"
          >
            Import a transcript or notes</button
          ><button
            class="primary"
            type="submit"
            :disabled="!question.trim() || agentBusy || working"
          >
            Send
          </button>
        </div>
      </form>
    </div>
  </aside>
  <PassagePicker ref="picker" @evidence="chosen = $event" />
  <SourcePreview ref="preview" return-label="Back to chat" />
</template>
