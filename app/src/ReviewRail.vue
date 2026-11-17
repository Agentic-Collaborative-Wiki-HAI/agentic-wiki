<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import PassagePicker from './PassagePicker.vue'
import { workspaceApi } from './workspaceApi'
import { reviewApi } from './reviewApi'
import { connection, model, agentBusy, refreshConnection } from './subscriptionState'
import { mentionsAgent } from './threadText'
import ThreadText from './ThreadText.vue'
import type { InlineReviewState } from './inlineReview'
import type { Page } from './generated/Page'
import type { PassageAnchor } from './generated/PassageAnchor'
import type { Discussion } from './generated/Discussion'
import type { Evidence } from './generated/Evidence'
import type { Proposal } from './generated/Proposal'
const props = defineProps<{
  page: Page
  tab: 'references' | 'comments'
  selection: PassageAnchor | null
  editing: boolean
}>()
const emit = defineEmits<{
  published: []
  clearSelection: []
  locate: [PassageAnchor]
  settings: []
  preview: [InlineReviewState]
}>()
const discussions = ref<Discussion[]>([]),
  citations = ref<Evidence[]>([]),
  pageSources = ref<Evidence[]>([])
const activeId = ref(''),
  comment = ref(''),
  reply = ref(''),
  error = ref(''),
  notice = ref(''),
  working = ref(false)
const active = computed(() => discussions.value.find((d) => d.id === activeId.value))
const pending = computed(() => active.value?.proposals.find((p) => p.status === 'pending'))
const chosenSources = ref<Evidence[] | null>(null)
const linkedIds = ref<string[]>([]),
  contextNotice = ref('')
const picker = ref<InstanceType<typeof PassagePicker>>()
const commentSources = computed(() => chosenSources.value ?? pageSources.value)
const sources = computed(() => chosenSources.value ?? pending.value?.evidence ?? pageSources.value)
function chooseSources(context: Evidence[]) {
  chosenSources.value = context
  sourcesError.value = ''
}
function openSourcePicker(forComment: boolean) {
  void picker.value?.open('evidence', {
    linkedIds: linkedIds.value,
    selected: forComment ? commentSources.value : sources.value,
  })
}
const sourcesError = ref(''),
  loadingSources = ref(false),
  needsConnection = ref(false)
const runId = ref(''),
  runThread = ref('')
const editingProposal = ref(''),
  wording = ref('')
const preview = ref<Evidence | null>(null),
  evidenceDialog = ref<HTMLDialogElement>(),
  commentBox = ref<HTMLTextAreaElement>()
const historyThread = ref<Discussion | null>(null),
  historyDialog = ref<HTMLDialogElement>()
const drafts = new Map<string, string>()
let sequence = 0,
  sourceSequence = 0
const displayActor = (actor: string) =>
  actor === 'local-operator' ? 'You' : actor === 'example-seed' ? 'Example workspace' : actor
const requestAnchor = computed(() =>
  pending.value?.stale ? props.selection : (pending.value?.anchor ?? active.value?.anchor ?? null),
)
const events = computed(() => {
  if (!active.value) return []
  return [
    ...active.value.messages.map((message, i) => ({
      kind: 'message' as const,
      message,
      at: message.createdAt,
      order: i,
    })),
    ...active.value.proposals.map((proposal, i) => ({
      kind: 'proposal' as const,
      proposal,
      at: proposal.createdAt,
      order: active.value!.messages.length + i,
    })),
  ].sort((a, b) => a.at.localeCompare(b.at) || a.order - b.order)
})
watch(reply, (value) => {
  if (activeId.value) drafts.set(activeId.value, value)
})
watch(activeId, (id) => {
  chosenSources.value = null
  reply.value = drafts.get(id) ?? ''
  editingProposal.value = ''
  error.value = ''
  notice.value = ''
})
watch(
  () => [props.page.id, props.page.revision],
  (current, old) => {
    if (current[0] !== old?.[0]) {
      chosenSources.value = null
      activeId.value = ''
      comment.value = ''
      needsConnection.value = false
    }
    void reload()
  },
  { immediate: true },
)
async function reload() {
  const seq = ++sequence
  try {
    const [d, c] = await Promise.all([
      reviewApi.threads(props.page.id),
      reviewApi.citations(props.page.id, props.page.revision),
    ])
    if (seq !== sequence) return
    discussions.value = d
    citations.value = c
    if (!d.some((t) => t.id === activeId.value)) activeId.value = d.at(-1)?.id ?? ''
  } catch (e) {
    error.value = String(e)
  }
}
let sourceTimer: ReturnType<typeof setTimeout> | undefined
watch(
  () => [
    props.page.id,
    props.page.revision,
    props.selection?.quote,
    active.value?.anchor.quote,
    comment.value,
    reply.value,
  ],
  () => {
    const seq = ++sourceSequence
    clearTimeout(sourceTimer)
    sourcesError.value = ''
    loadingSources.value = true
    sourceTimer = setTimeout(async () => {
      try {
        const text = Array.from(
          `${props.selection?.quote ?? active.value?.anchor.quote ?? ''} ${props.selection ? comment.value : reply.value}`,
        )
          .slice(0, 1000)
          .join('')
        const context = await workspaceApi.context(props.page.id, text)
        if (seq === sourceSequence) {
          pageSources.value = context.evidence
          linkedIds.value = context.linkedIds
          contextNotice.value = context.notice
        }
      } catch (e) {
        if (seq === sourceSequence) sourcesError.value = String(e)
      } finally {
        if (seq === sourceSequence) loadingSources.value = false
      }
    }, 250)
  },
  { immediate: true },
)
watch(
  () => props.selection?.quote,
  () => {
    chosenSources.value = null
  },
)
watch(
  () =>
    [pending.value, active.value?.anchor, props.editing, props.tab, editingProposal.value] as const,
  () => {
    emit('preview', {
      proposal: props.editing ? null : (pending.value ?? null),
      anchor: props.editing || props.tab !== 'comments' ? null : (active.value?.anchor ?? null),
      editing: !!editingProposal.value,
      onWording: (value) => {
        wording.value = value
      },
      onEvidence: openEvidence,
    })
  },
  { immediate: true },
)
async function action(fn: () => Promise<unknown>) {
  if (working.value) return
  working.value = true
  error.value = ''
  notice.value = ''
  try {
    await fn()
    await reload()
  } catch (e) {
    error.value = String(e)
  } finally {
    working.value = false
  }
}
function selectThread(id: string) {
  activeId.value = id
  if (active.value) emit('locate', active.value.anchor)
}
async function readyForAgent(context = sources.value) {
  error.value = ''
  try {
    await refreshConnection()
  } catch (e) {
    error.value = String(e)
    needsConnection.value = true
    return false
  }
  needsConnection.value = !connection.value?.connected || !!connection.value.waiting || !model.value
  if (needsConnection.value) return false
  if ((!chosenSources.value && (loadingSources.value || sourcesError.value)) || !context.length) {
    error.value =
      sourcesError.value ||
      (loadingSources.value
        ? 'Source context is still loading. Please try again.'
        : 'There is no linked source evidence for this passage yet.')
    return false
  }
  return true
}
async function postComment() {
  const anchor = props.selection,
    text = comment.value.trim(),
    pageId = props.page.id,
    baseRevision = props.page.revision
  if (!anchor || !text || agentBusy.value || working.value) return
  const asks = mentionsAgent(text)
  if (asks && !(await readyForAgent(commentSources.value))) return
  const sourceContext = commentSources.value
  if (pageId !== props.page.id || baseRevision !== props.page.revision) return
  let d: Discussion | undefined
  await action(async () => {
    d = await reviewApi.start(pageId, baseRevision, anchor, text)
    activeId.value = d.id
    comment.value = ''
    emit('clearSelection')
    if (!asks) notice.value = 'Comment added.'
  })
  if (asks && d) await ask(d, text, anchor, d.messages[0]?.id, sourceContext)
}
async function postReply() {
  if (!active.value || !reply.value.trim() || agentBusy.value || working.value) return
  const text = reply.value.trim(),
    d = active.value
  if (mentionsAgent(text)) {
    if (!requestAnchor.value || requestAnchor.value.state === 'detached') {
      error.value =
        'Select the current passage in the article before asking for a refreshed suggestion.'
      return
    }
    const anchor = requestAnchor.value,
      baseRevision = props.page.revision
    if (
      (await readyForAgent()) &&
      d.pageId === props.page.id &&
      activeId.value === d.id &&
      baseRevision === props.page.revision
    )
      await ask(d, text, anchor)
  } else
    await action(async () => {
      await reviewApi.reply(d.id, text)
      reply.value = ''
    })
}
async function ask(
  d: Discussion,
  text: string,
  anchor: PassageAnchor,
  messageId?: string,
  sourceContext = sources.value,
) {
  if (agentBusy.value || d.pageId !== props.page.id) return
  const id = crypto.randomUUID(),
    pageId = props.page.id,
    baseRevision = props.page.revision,
    selectedSources = sourceContext.map((e) => e.source),
    selectedModel = model.value
  runId.value = id
  runThread.value = d.id
  agentBusy.value = true
  error.value = ''
  notice.value = ''
  try {
    if (!messageId) {
      const posted = await reviewApi.reply(d.id, text)
      messageId = posted.messages.at(-1)!.id
    }
    if (activeId.value === d.id) reply.value = ''
    await reload()
    await reviewApi.ask({
      runId: id,
      threadId: d.id,
      requestText: text,
      sources: selectedSources,
      anchor,
      baseRevision,
      parentId: d.proposals.find((p) => p.status === 'pending')?.id ?? null,
      model: selectedModel,
      messageId: messageId ?? null,
    })
    drafts.delete(d.id)
    if (props.page.id === pageId && activeId.value === d.id) {
      reply.value = ''
      notice.value = 'Suggested edit ready for review.'
    }
  } catch (e) {
    drafts.set(d.id, text)
    if (activeId.value === d.id) {
      error.value = String(e)
      reply.value = text
    }
  } finally {
    runId.value = ''
    runThread.value = ''
    agentBusy.value = false
    await reload()
  }
}
async function cancelRequest() {
  try {
    if (!(await reviewApi.cancel(runId.value)))
      notice.value = 'The request has already finished. Review its result below.'
  } catch (e) {
    error.value = String(e)
  }
}
async function decide(p: Proposal, accept: boolean) {
  await action(async () => {
    await reviewApi.decide(p.id, accept)
    notice.value = accept
      ? 'Wording and citation saved together.'
      : 'Suggestion rejected. Page wording and citations are unchanged.'
    if (accept) emit('published')
  })
}
async function openEvidence(e: Evidence) {
  preview.value = e
  await nextTick()
  evidenceDialog.value?.showModal()
}
async function openCitation(id: string) {
  const e = citations.value.find((e) => e.id === id)
  if (e) await openEvidence(e)
}
function editProposal(p: Proposal) {
  wording.value = p.candidate.proposedWording
  editingProposal.value = p.id
  emit('locate', p.anchor)
}
async function acceptEdited() {
  await action(async () => {
    const d = await reviewApi.revise(
      editingProposal.value,
      wording.value,
      'Suggested wording edited by the reviewer before acceptance.',
    )
    const revised = d.proposals.find((p) => p.status === 'pending')!
    // Revision and acceptance remain distinct durable events. If acceptance fails, the edited draft remains pending.
    editingProposal.value = ''
    await reviewApi.decide(revised.id, true)
    notice.value = 'Edited wording and citation saved together.'
    emit('published')
  })
}
async function focusComment() {
  await nextTick()
  commentBox.value?.focus()
}
async function inspectThread(d: Discussion) {
  historyThread.value = d
  await nextTick()
  historyDialog.value?.showModal()
}
onBeforeUnmount(() => {
  ++sequence
  ++sourceSequence
  clearTimeout(sourceTimer)
  if (runId.value) void reviewApi.cancel(runId.value)
})
defineExpose({
  reload,
  openCitation,
  openEvidence,
  focusComment,
  discussions,
  citations,
  inspectThread,
})
</script>
<template>
  <PassagePicker ref="picker" @evidence="chooseSources" />
  <p v-if="error" class="review-error" role="alert">{{ error }}</p>
  <p v-if="notice" class="review-notice" role="status">{{ notice }}</p>
  <p v-if="needsConnection" class="connection-prompt">
    Connect ChatGPT in <button class="text-link" @click="emit('settings')">Settings</button> to send
    this request. Your draft stays here.
  </p>
  <div
    v-show="tab === 'references'"
    id="references-panel"
    role="tabpanel"
    aria-labelledby="references-tab"
  >
    <p v-if="!citations.length">No references yet.</p>
    <article v-for="(e, i) in citations" :key="e.id" class="reference-card">
      <button class="text-link" @click="openEvidence(e)">{{ i + 1 }} · {{ e.title }}</button>
      <p class="muted">Revision {{ e.source.revision }}</p>
      <blockquote>{{ e.quote }}</blockquote>
    </article>
  </div>
  <div
    v-show="tab === 'comments'"
    id="comments-panel"
    role="tabpanel"
    aria-labelledby="comments-tab"
  >
    <form v-if="selection && !editing" class="comment-composer" @submit.prevent="postComment">
      <strong>Comment on this passage</strong>
      <blockquote>{{ selection.quote }}</blockquote>
      <textarea
        ref="commentBox"
        v-model="comment"
        aria-label="New passage comment"
        placeholder="Write a comment… Mention @agent to ask for help."
        required
        maxlength="16000"
        :disabled="agentBusy"
      ></textarea>
      <details v-if="mentionsAgent(comment)" class="request-context">
        <summary>Context sent to the agent</summary>
        <p>Your comment, the selected passage above, and these source excerpts:</p>
        <p v-if="loadingSources">Loading linked sources…</p>
        <p v-if="sourcesError" class="review-warning">{{ sourcesError }}</p>
        <p>{{ chosenSources ? 'Using your selected excerpts.' : contextNotice }}</p>
        <button
          type="button"
          class="text-link"
          :disabled="agentBusy"
          @click="openSourcePicker(true)"
        >
          Choose sources
        </button>
        <div v-for="e in commentSources" :key="e.id">
          <button type="button" class="text-link" @click="openEvidence(e)">
            {{ e.title }} · revision {{ e.source.revision }}
          </button>
          <blockquote>{{ e.quote }}</blockquote>
        </div>
        <p v-if="!loadingSources && !commentSources.length">
          No linked source excerpts are available.
        </p>
      </details>
      <div class="review-actions">
        <button type="button" @click="emit('clearSelection')">Cancel</button
        ><button class="primary" :disabled="working || agentBusy || !comment.trim()">
          Add comment
        </button>
      </div>
    </form>
    <p v-if="!discussions.length && !selection" class="muted">
      Select a passage to start a discussion.
    </p>
    <nav v-if="discussions.length > 1" class="thread-list" aria-label="Passage discussions">
      <button
        v-for="d in discussions"
        :key="d.id"
        :aria-pressed="activeId === d.id"
        @click="selectThread(d.id)"
      >
        {{ d.anchor.quote.slice(0, 60) }}{{ d.resolved ? ' · Resolved' : '' }}
      </button>
    </nav>
    <article v-if="active" class="discussion" :data-thread-id="active.id">
      <div class="discussion-heading">
        <span>Comment thread{{ active.resolved ? ' · Resolved' : '' }}</span
        ><button
          class="compact"
          :disabled="working"
          @click="action(() => reviewApi.resolve(activeId, !active!.resolved))"
        >
          {{ active.resolved ? 'Reopen thread' : 'Resolve thread' }}
        </button>
      </div>
      <button class="text-link passage-location" @click="emit('locate', active.anchor)">
        Show passage
      </button>
      <p v-if="active.anchor.state === 'detached'" class="muted">
        The passage has changed. This discussion keeps its original reference to revision
        {{ active.baseRevision }}.
      </p>
      <template
        v-for="event in events"
        :key="event.kind === 'message' ? event.message.id : event.proposal.id"
      >
        <div v-if="event.kind === 'message'" class="thread-message">
          <div class="message-author">
            <span class="person-avatar" aria-hidden="true">Y</span>
            <div>
              <strong>{{ displayActor(event.message.actorId) }}</strong><time>{{
                new Date(event.message.createdAt).toLocaleString()
              }}</time>
            </div>
          </div>
          <p><ThreadText :text="event.message.body" /></p>
        </div>
        <details v-else-if="event.proposal.status === 'superseded'" class="earlier-suggestion">
          <summary>Earlier suggestion</summary>
          <p>{{ event.proposal.candidate.proposedWording }}</p>
          <p>{{ event.proposal.candidate.explanation }}</p>
        </details>
        <div v-else class="proposal-card thread-message" :data-proposal-id="event.proposal.id">
          <div class="message-author">
            <span
              v-if="event.proposal.origin === 'human revision'"
              class="person-avatar"
              aria-hidden="true"
              >Y</span
            ><img v-else src="/assets/05e24.svg" alt="" />
            <div>
              <strong>{{
                event.proposal.origin === 'human revision' ? 'You · Edited suggestion' : 'Agent'
              }}</strong><time>{{ new Date(event.proposal.createdAt).toLocaleString() }}</time>
            </div>
          </div>
          <p class="proposal-status">
            {{
              event.proposal.status === 'pending'
                ? 'Suggested edit · Pending'
                : event.proposal.status === 'accepted'
                  ? 'Accepted edit'
                  : 'Rejected edit'
            }}
          </p>
          <p>{{ event.proposal.candidate.explanation }}</p>
          <p class="evidence-links">
            <button
              v-for="(e, i) in event.proposal.evidence.filter((e) =>
                event.proposal.candidate.citedSourceIds.includes(e.id),
              )"
              :key="e.id"
              class="text-link"
              @click="openEvidence(e)"
            >
              [{{ i + 1 }}] {{ e.title }}
            </button>
          </p>
          <p
            v-if="event.proposal.stale && event.proposal.status === 'pending'"
            class="review-warning"
          >
            The page changed after this suggestion. Select the current passage and reply to @agent
            to request a refreshed version before accepting.
          </p>
          <template v-if="editingProposal === event.proposal.id">
            <p class="muted">Edit the suggested wording in the article.</p>
            <div class="review-actions">
              <button
                class="primary"
                :disabled="working || !wording.trim() || agentBusy"
                @click="acceptEdited"
              >
                Accept edited</button
              ><button class="text-link" @click="editingProposal = ''">Cancel edits</button>
            </div>
          </template>
          <template v-else-if="event.proposal.status === 'pending'">
            <button
              class="text-link"
              :disabled="working || event.proposal.stale || editing || agentBusy"
              @click="editProposal(event.proposal)"
            >
              Edit suggested text
            </button>
            <div class="review-actions proposal-decisions">
              <button
                class="primary"
                :disabled="working || event.proposal.stale || editing || agentBusy"
                @click="decide(event.proposal, true)"
              >
                Accept</button
              ><button
                :disabled="working || editing || agentBusy"
                @click="decide(event.proposal, false)"
              >
                Reject
              </button>
            </div>
          </template>
          <details v-if="event.proposal.status !== 'pending' || event.proposal.stale">
            <summary>View suggested wording</summary>
            <div class="proposal-diff">
              <del>{{ event.proposal.anchor.quote }}</del><ins>{{
                event.proposal.candidate.proposedWording
              }}</ins>
            </div>
          </details>
          <p v-if="event.proposal.publishedRevision" class="muted">
            Saved as revision {{ event.proposal.publishedRevision }}.
          </p>
          <span v-if="event.proposal.origin.startsWith('Test fixture')" class="muted">{{
            event.proposal.origin
          }}</span>
        </div>
      </template>
      <form v-if="!active.resolved" class="reply-composer" @submit.prevent="postReply">
        <textarea
          v-model="reply"
          aria-label="Reply to discussion"
          placeholder="Write a reply… Mention @agent to ask for help."
          required
          maxlength="16000"
          :disabled="agentBusy || !!editingProposal"
        ></textarea>
        <details v-if="mentionsAgent(reply)" class="request-context">
          <summary>Context sent to the agent</summary>
          <p>
            {{
              chosenSources
                ? 'Using your selected excerpts.'
                : pending
                  ? 'Using the sources pinned to this pending suggestion.'
                  : contextNotice
            }}
          </p>
          <button
            type="button"
            class="text-link"
            :disabled="agentBusy"
            @click="openSourcePicker(false)"
          >
            Choose sources
          </button>
          <p>
            Your reply and this passage<span v-if="pending">, with the previous suggestion</span>:
          </p>
          <blockquote v-if="requestAnchor">{{ requestAnchor.quote }}</blockquote>
          <p v-else class="review-warning">
            Select the current passage before requesting a refreshed suggestion.
          </p>
          <div v-if="pending" class="previous-proposal-context">
            <strong>Previous suggestion</strong>
            <blockquote>{{ pending.candidate.proposedWording }}</blockquote>
            <p>{{ pending.candidate.explanation }}</p>
          </div>
          <p v-if="loadingSources">Loading linked sources…</p>
          <p v-if="sourcesError" class="review-warning">{{ sourcesError }}</p>
          <div v-for="e in sources" :key="e.id">
            <button type="button" class="text-link" @click="openEvidence(e)">
              {{ e.title }} · revision {{ e.source.revision }}
            </button>
            <blockquote>{{ e.quote }}</blockquote>
          </div>
          <p v-if="!loadingSources && !sources.length">No linked source excerpts are available.</p>
        </details>
        <div class="review-actions">
          <button type="button" :disabled="agentBusy" @click="reply = ''">Cancel</button
          ><button
            class="primary"
            :disabled="working || agentBusy || !!editingProposal || editing || !reply.trim()"
          >
            Reply
          </button>
        </div>
      </form>
      <div v-if="runId && runThread === active.id" class="agent-progress" role="status">
        Agent is preparing a suggestion…<button class="text-link" @click="cancelRequest">
          Cancel request
        </button>
      </div>
      <button class="text-link discussion-history-link" @click="inspectThread(active)">
        Discussion history
      </button>
    </article>
  </div>
  <dialog ref="evidenceDialog" class="evidence-dialog">
    <template v-if="preview"
      ><p class="eyebrow">Source evidence · Revision {{ preview.source.revision }}</p>
      <h2>{{ preview.title }}</h2>
      <blockquote>{{ preview.quote }}</blockquote>
      <p class="muted">This is the exact source revision cited by the suggestion or page.</p>
      <button class="primary" @click="evidenceDialog?.close()">Back to discussion</button></template
    >
  </dialog>
  <dialog ref="historyDialog" class="discussion-history">
    <template v-if="historyThread"
      ><h2>Discussion history</h2>
      <blockquote>{{ historyThread.anchor.quote }}</blockquote>
      <div v-for="m in historyThread.messages" :key="m.id">
        <strong>{{ displayActor(m.actorId) }}</strong>
        <p><ThreadText :text="m.body" /></p>
      </div>
      <article v-for="p in historyThread.proposals" :key="p.id" class="proposal-card">
        <h3>
          {{ p.status
          }}<span v-if="p.publishedRevision"> · Revision {{ p.publishedRevision }}</span>
        </h3>
        <p class="muted">{{ p.origin }}</p>
        <p><strong>Request:</strong> {{ p.requestText }}</p>
        <div class="proposal-diff">
          <del>{{ p.anchor.quote }}</del><ins>{{ p.candidate.proposedWording }}</ins>
        </div>
        <p>{{ p.candidate.explanation }}</p>
        <button v-for="e in p.evidence" :key="e.id" class="text-link" @click="openEvidence(e)">
          {{ e.title }} · r{{ e.source.revision }}
        </button>
      </article>
      <button class="primary" @click="historyDialog?.close()">Back to page</button></template
    >
  </dialog>
</template>
