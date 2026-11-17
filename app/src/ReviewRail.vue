<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { reviewApi } from './reviewApi'
import type { Page } from './generated/Page'
import type { PassageAnchor } from './generated/PassageAnchor'
import type { Discussion } from './generated/Discussion'
import type { Evidence } from './generated/Evidence'
import type { Proposal } from './generated/Proposal'
import type { ConnectionStatus } from './generated/ConnectionStatus'
import type { AvailableModel } from './generated/AvailableModel'
const props = defineProps<{
  page: Page
  tab: 'references' | 'comments'
  selection: PassageAnchor | null
  editing: boolean
}>()
const emit = defineEmits<{ published: []; clearSelection: []; locate: [PassageAnchor] }>()
const discussions = ref<Discussion[]>([]),
  citations = ref<Evidence[]>([]),
  sources = ref<Evidence[]>([])
const activeId = ref(''),
  comment = ref(''),
  reply = ref(''),
  error = ref(''),
  notice = ref(''),
  working = ref(false)
const active = computed(() => discussions.value.find((d) => d.id === activeId.value))
const pending = computed(() => active.value?.proposals.find((p) => p.status === 'pending'))
const showRequest = ref(false),
  requestText = ref(''),
  sourceIds = ref<string[]>([]),
  requestAnchor = ref<PassageAnchor | null>(null)
const connection = ref<ConnectionStatus | null>(null),
  models = ref<AvailableModel[]>([]),
  model = ref(''),
  runId = ref('')
const editingProposal = ref(''),
  wording = ref(''),
  reason = ref('')
const preview = ref<Evidence | null>(null),
  evidenceDialog = ref<HTMLDialogElement>(),
  commentBox = ref<HTMLTextAreaElement>()
const historyThread = ref<Discussion | null>(null),
  historyDialog = ref<HTMLDialogElement>()
let poll: ReturnType<typeof setInterval> | undefined,
  sequence = 0
const displayActor = (actor: string) =>
  actor === 'local-operator' ? 'You' : actor === 'example-seed' ? 'Example workspace' : actor
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
watch(
  () => [props.page.id, props.page.revision],
  (current, old) => {
    if (current[0] !== old?.[0]) {
      activeId.value = ''
      comment.value = ''
      reply.value = ''
      showRequest.value = false
      editingProposal.value = ''
      error.value = ''
      notice.value = ''
    }
    void reload()
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
  showRequest.value = false
  editingProposal.value = ''
  reply.value = ''
}
async function postComment() {
  const anchor = props.selection
  if (!anchor) return
  await action(async () => {
    const d = await reviewApi.start(props.page.id, props.page.revision, anchor, comment.value)
    activeId.value = d.id
    comment.value = ''
    emit('clearSelection')
    notice.value = 'Comment added. The agent will wait until you ask.'
  })
}
async function postReply() {
  if (!active.value) return
  await action(async () => {
    await reviewApi.reply(activeId.value, reply.value)
    reply.value = ''
  })
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
async function refreshConnection() {
  try {
    connection.value = await reviewApi.connection()
    if (!connection.value.waiting && poll) {
      clearInterval(poll)
      poll = undefined
    }
    if (connection.value.connected && !connection.value.waiting && !models.value.length) {
      models.value = await reviewApi.models()
      model.value = models.value[0]?.slug ?? ''
    }
  } catch (e) {
    error.value = String(e)
  }
}
async function signIn() {
  await action(async () => {
    await reviewApi.signIn()
    await refreshConnection()
    if (!poll) poll = setInterval(() => void refreshConnection(), 1000)
  })
}
async function prepareRequest() {
  if (!active.value) return
  error.value = ''
  try {
    sources.value = await reviewApi.sources()
    sourceIds.value = pending.value?.evidence.map((e) => e.id) ?? []
    requestText.value = ''
    showRequest.value = true
    requestAnchor.value =
      props.selection ??
      (pending.value?.stale ? null : (pending.value?.anchor ?? active.value.anchor))
    if (requestAnchor.value?.state === 'detached') requestAnchor.value = null
    await refreshConnection()
  } catch (e) {
    error.value = String(e)
  }
}
async function cancelRequest() {
  try {
    const cancelled = await reviewApi.cancel(runId.value)
    if (!cancelled) notice.value = 'The request has already finished. Review its result below.'
  } catch (e) {
    error.value = String(e)
  }
}
async function ask() {
  if (!active.value || !requestAnchor.value || runId.value) return
  const id = crypto.randomUUID()
  runId.value = id
  error.value = ''
  notice.value = ''
  const threadId = activeId.value
  try {
    const result = await reviewApi.ask({
      runId: id,
      threadId,
      requestText: requestText.value,
      sources: sources.value.filter((e) => sourceIds.value.includes(e.id)).map((e) => e.source),
      anchor: requestAnchor.value,
      baseRevision: props.page.revision,
      parentId: pending.value?.id ?? null,
      model: model.value,
    })
    if (result.pageId === props.page.id) {
      activeId.value = result.id
      showRequest.value = false
      notice.value = 'Suggestion ready for your review.'
    }
  } catch (e) {
    error.value = String(e)
  } finally {
    runId.value = ''
    await reload()
  }
}
function editProposal(p: Proposal) {
  editingProposal.value = p.id
  wording.value = p.candidate.proposedWording
  reason.value = ''
}
async function saveProposal() {
  await action(async () => {
    await reviewApi.revise(editingProposal.value, wording.value, reason.value)
    editingProposal.value = ''
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
  if (poll) clearInterval(poll)
  if (runId.value) void reviewApi.cancel(runId.value)
})
defineExpose({ reload, openCitation, openEvidence, focusComment, discussions, inspectThread })
</script>
<template>
  <p v-if="error" class="review-error" role="alert">{{ error }}</p>
  <p v-if="notice" class="review-notice" role="status">{{ notice }}</p>
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
        placeholder="What would you like to discuss?"
        required
        maxlength="16000"
      ></textarea>
      <div class="review-actions">
        <button type="button" @click="emit('clearSelection')">Cancel</button
        ><button class="primary" :disabled="working || !comment.trim()">Add comment</button>
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
        <strong>Passage discussion</strong><span>{{ active.resolved ? 'Resolved' : 'Open' }}</span>
      </div>
      <button class="quoted-passage" @click="emit('locate', active.anchor)">
        {{ active.anchor.quote }}
      </button>
      <p v-if="active.anchor.state === 'detached'" class="muted">
        This passage changed. The original quote is retained from revision
        {{ active.baseRevision }}.
      </p>
      <div v-for="m in active.messages" :key="m.id" class="thread-message">
        <strong>{{ displayActor(m.actorId) }}</strong><time>{{
          new Date(m.createdAt).toLocaleString()
        }}</time>
        <p>{{ m.body }}</p>
      </div>
      <div
        v-for="p in active.proposals.filter((p) => p.status !== 'superseded')"
        :key="p.id"
        class="proposal-card"
        :data-proposal-id="p.id"
      >
        <div class="discussion-heading">
          <strong>{{
            p.status === 'pending'
              ? 'Suggested edit'
              : p.status === 'accepted'
                ? 'Accepted edit'
                : 'Rejected edit'
          }}</strong>
        </div>
        <p class="muted">{{ p.origin }}</p>
        <p>{{ p.candidate.explanation }}</p>
        <div class="proposal-diff">
          <del>{{ p.anchor.quote }}</del><ins>{{ p.candidate.proposedWording }}</ins>
        </div>
        <strong>Evidence</strong>
        <div
          v-for="e in p.evidence.filter((e) => p.candidate.citedSourceIds.includes(e.id))"
          :key="e.id"
        >
          <button class="text-link" @click="openEvidence(e)">
            {{ e.title }} · revision {{ e.source.revision }}
          </button>
        </div>
        <p v-if="p.stale && p.status === 'pending'" class="review-warning">
          The page changed after this suggestion. Select the current passage, then request a
          refreshed suggestion before accepting.
        </p>
        <form v-if="editingProposal === p.id" @submit.prevent="saveProposal">
          <label
            >Proposed wording<textarea
              v-model="wording"
              aria-label="Edit proposed wording"
              required
              maxlength="16000"
            ></textarea></label
          ><label
            >Reason for your revision<textarea
              v-model="reason"
              aria-label="Reason for proposal revision"
              required
              maxlength="16000"
            ></textarea>
          </label>
          <div class="review-actions">
            <button type="button" @click="editingProposal = ''">Cancel revision</button
            ><button class="primary" :disabled="working || !wording.trim() || !reason.trim()">
              Save proposal revision
            </button>
          </div>
        </form>
        <div v-else-if="p.status === 'pending'" class="review-actions">
          <button :disabled="working || p.stale || editing || !!runId" @click="editProposal(p)">
            Edit suggestion
          </button>
          <button :disabled="working || editing || !!runId" @click="prepareRequest">
            {{ p.stale ? 'Refresh suggestion' : 'Ask for revision' }}
          </button>
          <button :disabled="working || editing || !!runId" @click="decide(p, false)">
            Reject
          </button>
          <button
            class="primary"
            :disabled="working || p.stale || editing || !!runId"
            @click="decide(p, true)"
          >
            Accept edit
          </button>
        </div>
        <p v-if="p.publishedRevision" class="muted">Saved as revision {{ p.publishedRevision }}.</p>
      </div>
      <form class="reply-composer" @submit.prevent="postReply">
        <label
          >Reply<textarea
            v-model="reply"
            aria-label="Reply to discussion"
            placeholder="Write a reply…"
            required
            maxlength="16000"
          ></textarea></label
        ><button :disabled="working || !reply.trim()">Reply</button>
      </form>
      <div class="review-actions">
        <button
          v-if="!pending"
          :disabled="working || editing || !!runId"
          class="text-link"
          @click="prepareRequest"
        >
          Ask agent</button
        ><button
          :disabled="working"
          @click="action(() => reviewApi.resolve(activeId, !active!.resolved))"
        >
          {{ active.resolved ? 'Reopen thread' : 'Resolve thread' }}</button
        ><button @click="inspectThread(active)">Discussion history</button>
      </div>
      <section v-if="showRequest" class="agent-request">
        <h3>{{ pending ? 'Request another version' : 'Ask the agent' }}</h3>
        <p class="muted">
          Only this request, selected passage and chosen source excerpts will be sent.
        </p>
        <blockquote v-if="requestAnchor">{{ requestAnchor.quote }}</blockquote>
        <p v-else class="review-warning">
          Select the current passage before requesting a refreshed suggestion.
        </p>
        <button v-if="selection" @click="requestAnchor = selection">Use selected passage</button>
        <label
          >Your request<textarea
            v-model="requestText"
            aria-label="Agent request"
            placeholder="What should the agent check or change?"
            maxlength="16000"
          ></textarea>
        </label>
        <fieldset>
          <legend>Sources to include</legend>
          <label v-for="e in sources" :key="e.id" class="source-choice"
            ><input v-model="sourceIds" type="checkbox" :value="e.id" /><span
              >{{ e.title }} · r{{ e.source.revision }}<small>{{ e.quote }}</small></span
            ></label
          >
        </fieldset>
        <div v-if="!connection?.connected || connection.waiting" class="connection-card">
          <p>{{ connection?.message ?? 'Connect your ChatGPT subscription to continue.' }}</p>
          <button v-if="!connection?.waiting" class="primary" @click="signIn" :disabled="working">
            Continue with ChatGPT</button
          ><button
            v-else
            @click="
              action(async () => {
                await reviewApi.cancelSignIn()
                await refreshConnection()
              })
            "
          >
            Cancel sign-in
          </button>
        </div>
        <template v-else
          ><p class="muted">
            ChatGPT subscription connected{{ connection.account ? ` · ${connection.account}` : '' }}
          </p>
          <button class="text-link" :disabled="!!runId || working" @click="signIn">
            Reconnect ChatGPT
          </button>
          <label
            >Model<select v-model="model" aria-label="Suggestion model">
              <option v-for="m in models" :key="m.slug" :value="m.slug">{{ m.displayName }}</option>
            </select></label
          ></template
        >
        <div class="review-actions">
          <button v-if="runId" @click="cancelRequest">Cancel request</button
          ><template v-else
            ><button @click="showRequest = false">Close request</button
            ><button
              class="primary"
              :disabled="
                !connection?.connected ||
                !model ||
                !requestText.trim() ||
                !sourceIds.length ||
                sourceIds.length > 8 ||
                !requestAnchor ||
                editing
              "
              @click="ask"
            >
              Request suggestion
            </button></template
          >
        </div>
        <p v-if="runId" role="status">Preparing a suggestion… Your page stays unchanged.</p>
      </section>
      <details v-if="active.proposals.some((p) => p.status === 'superseded')">
        <summary>Earlier versions</summary>
        <article v-for="p in active.proposals.filter((p) => p.status === 'superseded')" :key="p.id">
          <p>{{ p.origin }} · Superseded</p>
          <blockquote>{{ p.candidate.proposedWording }}</blockquote>
          <p>{{ p.candidate.explanation }}</p>
        </article>
      </details>
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
        <p>{{ m.body }}</p>
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
