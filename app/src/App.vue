<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { hasUnsupportedPaste, wikiExtensions } from './editor'
import { api } from './api'
import ReviewRail from './ReviewRail.vue'
import AgentSettings from './AgentSettings.vue'
import { showInlineReview, type InlineReviewState } from './inlineReview'
const settingsOpen = ref(false)
const reviewPreview = shallowRef<InlineReviewState>({
  proposal: null,
  anchor: null,
  editing: false,
})
import type { PassageAnchor } from './generated/PassageAnchor'
import type { Discussion } from './generated/Discussion'
const selectionAnchor = ref<PassageAnchor | null>(null)
const reviewRail = ref<InstanceType<typeof ReviewRail>>()
import type { Page } from './generated/Page'
import type { RichDocument } from './generated/RichDocument'
import RevisionHistory from './RevisionHistory.vue'

const page = ref<Page | null>(null)
const editor = shallowRef<Editor>()
const editing = ref(false),
  busy = ref(false),
  treeOpen = ref(true),
  contentsOpen = ref(true),
  chatOpen = ref(false)
const equipmentOpen = ref(true),
  microscopeOpen = ref(true),
  guideOpen = ref(true)
const activeTab = ref<'references' | 'comments'>('references')
const error = ref(''),
  status = ref(''),
  draftState = ref(''),
  selectedText = ref('')
const revisionHistory = ref<InstanceType<typeof RevisionHistory>>()
const chatButton = ref<HTMLButtonElement>(),
  chatClose = ref<HTMLButtonElement>()
let draftTimer: ReturnType<typeof setTimeout> | undefined
let draftWrites = Promise.resolve()
let loadSequence = 0
watch(
  () => [editor.value, editing.value, reviewPreview.value] as const,
  () => {
    if (editor.value)
      showInlineReview(
        editor.value,
        editing.value ? { proposal: null, anchor: null, editing: false } : reviewPreview.value,
      )
  },
)
const headings = computed(
  () =>
    page.value?.document.doc.content
      .filter((b) => b.type === 'heading')
      .map((b) => ({
        id: b.attrs.blockId,
        text: b.content.map((n) => (n.type === 'text' ? n.text : '')).join(''),
      })) ?? [],
)
const icons = {
  tree: 'b5cc6',
  contents: 'e5c35',
  search: '93954',
  edit: 'a0fa6',
  history: '8773d',
  plus: '16a20',
  maintenance: '3eaa5',
  notifications: '433d9',
  profile: '6cf18',
  settings: '1b7d2',
  chat: '83771',
  chevron: 'afd50',
}
const icon = (name: keyof typeof icons) => `/assets/${icons[name]}.svg`
const readDocument = (): RichDocument => ({
  schemaVersion: 1,
  doc: editor.value!.getJSON() as unknown as RichDocument['doc'],
})

function scheduleDraft() {
  if (!editing.value || !page.value) return
  clearTimeout(draftTimer)
  draftState.value = 'Saving draft…'
  draftTimer = setTimeout(() => queueDraft(), 250)
}
function queueDraft() {
  if (!editing.value || !page.value) return
  const id = page.value.id,
    revision = page.value.revision,
    document = readDocument()
  draftWrites = draftWrites
    .catch(() => {})
    .then(() => api.putDraft(id, revision, document))
    .then(() => {
      draftState.value = 'Draft saved on this device'
    })
    .catch((e) => {
      error.value = String(e)
      draftState.value = 'Draft not saved'
    })
}
async function flushDraft() {
  clearTimeout(draftTimer)
  queueDraft()
  await draftWrites
}
async function loadPage(id: string) {
  if (busy.value) return
  if (editing.value) {
    error.value = 'Save or cancel your edit before leaving this page.'
    return
  }
  const seq = ++loadSequence
  busy.value = true
  error.value = ''
  status.value = ''
  try {
    const loaded = await api.read(id)
    if (seq !== loadSequence) return
    editor.value?.destroy()
    reviewPreview.value = { proposal: null, anchor: null, editing: false }
    page.value = loaded
    selectionAnchor.value = null
    selectedText.value = ''
    editor.value = new Editor({
      extensions: wikiExtensions,
      content: loaded.document.doc,
      editable: false,
      editorProps: {
        attributes: { 'aria-label': 'Page content', role: 'document', tabindex: '0' },
        handlePaste: (_view, event) => {
          if (!hasUnsupportedPaste(event.clipboardData?.getData('text/html') ?? '')) return false
          error.value =
            'This foundation supports paragraphs, headings and inline formatting. Paste lists, tables, images or block quotes as plain text for now.'
          return true
        },
      },
      onUpdate: scheduleDraft,
      onSelectionUpdate: ({ editor: e }) => {
        selectedText.value = e.state.doc.textBetween(
          e.state.selection.from,
          e.state.selection.to,
          ' ',
        )
        const { $from, $to, empty } = e.state.selection
        if (
          !editing.value &&
          !empty &&
          $from.sameParent($to) &&
          $from.parent.type.name === 'paragraph'
        ) {
          const quote = $from.parent.textBetween(
            $from.parentOffset,
            $to.parentOffset,
            '',
            (leaf) => (leaf.type.name === 'hardBreak' ? '\n' : '\uFFFC'),
          )
          selectionAnchor.value = {
            blockId: $from.parent.attrs.blockId,
            fromUtf16: $from.parentOffset,
            toUtf16: $to.parentOffset,
            quote,
            state: 'attached',
          }
        }
      },
    })
    const draft = await api.draft(id)
    if (draft) {
      if (draft.baseRevision === loaded.revision)
        status.value = 'An unfinished draft is saved. Edit page to continue it.'
      else status.value = 'A draft belongs to an older revision. The current page is unchanged.'
    }
    document.querySelector('.workspace')?.scrollTo(0, 0)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function beginEdit() {
  if (!page.value || busy.value) return
  busy.value = true
  error.value = ''
  try {
    const draft = await api.draft(page.value.id)
    if (draft && draft.baseRevision !== page.value.revision) {
      error.value =
        'The saved draft is outdated. Its contents are retained; reload or resolve it before editing.'
      return
    }
    if (draft) editor.value?.commands.setContent(draft.document.doc)
    selectionAnchor.value = null
    editing.value = true
    status.value = ''
    editor.value?.setEditable(true)
    editor.value?.commands.focus()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function save() {
  if (!page.value || busy.value) return
  busy.value = true
  error.value = ''
  try {
    await flushDraft()
    const saved = await api.save(page.value.id, page.value.revision, readDocument())
    page.value = saved
    editing.value = false
    editor.value?.setEditable(false)
    draftState.value = ''
    status.value = `Saved revision ${saved.revision}.`
    void reviewRail.value?.reload()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function cancel() {
  if (!page.value || busy.value) return
  busy.value = true
  clearTimeout(draftTimer)
  error.value = ''
  try {
    await draftWrites
    await api.discard(page.value.id)
    editor.value?.commands.setContent(page.value.document.doc)
    editing.value = false
    editor.value?.setEditable(false)
    draftState.value = ''
    status.value = 'Draft discarded. Saved page unchanged.'
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function openHistory() {
  try {
    await revisionHistory.value?.open()
  } catch (e) {
    error.value = String(e)
  }
}
function scrollTo(id: string) {
  document
    .querySelector(`[data-block-id="${CSS.escape(id)}"]`)
    ?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}
async function toggleChat() {
  chatOpen.value = !chatOpen.value
  await nextTick()
  ;(chatOpen.value ? chatClose.value : chatButton.value)?.focus()
}
function goHome() {
  settingsOpen.value = false
  void loadPage('getting-started')
}
function openSettings() {
  settingsOpen.value = true
}
async function closeSettings() {
  settingsOpen.value = false
  await nextTick()
  document.querySelector<HTMLButtonElement>('.settings-button')?.focus()
}
async function clearSelection() {
  const e = editor.value
  const position = e?.state.selection.to
  selectionAnchor.value = null
  selectedText.value = ''
  await nextTick()
  if (e && e === editor.value && position !== undefined) {
    // Return focus after removing the composer; removing a focused textarea can otherwise discard the restored browser selection.
    e.view.dom.focus({ preventScroll: true })
    e.commands.setTextSelection(position)
    const { node, offset } = e.view.domAtPos(position)
    window.getSelection()?.collapse(node, offset)
  }
}

async function startComment() {
  activeTab.value = 'comments'
  await nextTick()
  await reviewRail.value?.focusComment()
}
function citationClick(event: MouseEvent) {
  const marker = (event.target as HTMLElement).closest<HTMLElement>('[data-citation-id]')
  if (marker?.dataset.citationId) void reviewRail.value?.openCitation(marker.dataset.citationId)
}
async function showHistoryThread(d: Discussion) {
  await reviewRail.value?.inspectThread(d)
}
onMounted(() => loadPage('getting-started'))
onBeforeUnmount(() => {
  clearTimeout(draftTimer)
  editor.value?.destroy()
})
</script>

<template>
  <div class="application">
    <header class="wiki-header">
      <div class="identity">
        <button
          class="icon-button"
          :aria-expanded="treeOpen"
          aria-controls="wiki-tree"
          aria-label="Toggle wiki navigation"
          @click="treeOpen = !treeOpen"
        >
          <img :src="icon('tree')" alt="" />
        </button>
        <button class="brand" @click="goHome">The Lab</button>
        <nav v-if="settingsOpen" class="breadcrumbs" aria-label="Breadcrumb">
          <span>Settings</span><span>/</span><span class="crumb-current">Agent connection</span>
        </nav>
        <nav v-else class="breadcrumbs" aria-label="Breadcrumb">
          <span>Equipment</span><span>/</span><span>Microscope</span><span>/</span
          ><span class="crumb-current">{{ page?.title ?? 'Getting started' }}</span>
        </nav>
      </div>
      <label class="search"
        ><img :src="icon('search')" alt="" /><input
          placeholder="Search wiki"
          disabled
          aria-label="Search wiki — available in a later milestone"
          title="Search will be available in a later milestone"
      /></label>
      <nav class="global-actions" aria-label="Workspace actions">
        <button
          v-for="name in ['maintenance', 'notifications', 'profile'] as const"
          :key="name"
          class="compact"
          disabled
          :title="`${name} is planned for a later milestone`"
        >
          <img :src="icon(name)" alt="" />{{ name[0].toUpperCase() + name.slice(1) }}
        </button>
        <button
          class="compact settings-button"
          :aria-current="settingsOpen ? 'page' : undefined"
          @click="openSettings"
        >
          <img :src="icon('settings')" alt="" />Settings
        </button>
        <button ref="chatButton" class="compact" :aria-expanded="chatOpen" @click="toggleChat">
          <img :src="icon('chat')" alt="" />Chat
        </button>
      </nav>
    </header>
    <AgentSettings v-if="settingsOpen" @back="closeSettings" />
    <div
      v-show="!settingsOpen"
      class="workspace"
      :class="{ 'tree-hidden': !treeOpen, 'contents-hidden': !contentsOpen }"
    >
      <aside v-if="treeOpen" id="wiki-tree" class="wiki-tree" aria-label="Wiki pages">
        <div class="branch">
          <button
            class="chevron"
            aria-label="Expand or collapse Equipment"
            :aria-expanded="equipmentOpen"
            @click="equipmentOpen = !equipmentOpen"
          >
            <img :src="icon('chevron')" alt="" /></button
          ><span>Equipment</span>
        </div>
        <div v-if="equipmentOpen">
          <div class="branch depth-one">
            <button
              class="chevron"
              aria-label="Expand or collapse Microscope"
              :aria-expanded="microscopeOpen"
              @click="microscopeOpen = !microscopeOpen"
            >
              <img :src="icon('chevron')" alt="" /></button
            ><span>Microscope</span>
          </div>
          <div v-if="microscopeOpen">
            <div class="branch depth-two" :class="{ selected: page?.id === 'getting-started' }">
              <button
                class="chevron"
                aria-label="Expand or collapse Getting started"
                :aria-expanded="guideOpen"
                @click="guideOpen = !guideOpen"
              >
                <img :src="icon('chevron')" alt="" /></button
              ><button
                @click="loadPage('getting-started')"
                :aria-current="page?.id === 'getting-started' ? 'page' : undefined"
              >
                Getting started
              </button>
            </div>
            <button
              v-if="guideOpen"
              class="tree-row depth-three"
              :class="{ selected: page?.id === 'orientation-recording' }"
              :aria-current="page?.id === 'orientation-recording' ? 'page' : undefined"
              @click="loadPage('orientation-recording')"
            >
              Microscope orientation recording
            </button>
            <button
              v-for="name in [
                'Planning a microscope session',
                'Using the microscope',
                'Booking sheet',
              ]"
              :key="name"
              class="tree-row depth-two"
              disabled
              title="This route is planned for a later milestone"
            >
              {{ name }}
            </button>
          </div>
          <div class="branch depth-one">
            <img :src="icon('chevron')" alt="" /><span>Centrifuge</span>
          </div>
          <button
            class="tree-row depth-two"
            disabled
            title="This route is planned for a later milestone"
          >
            Centrifuge safety
          </button>
        </div>
        <button class="tree-row" disabled title="This route is planned for a later milestone">
          Onboarding
        </button>
      </aside>
      <aside class="page-contents" aria-label="On this page">
        <button
          class="icon-button contents-toggle"
          :aria-expanded="contentsOpen"
          aria-controls="heading-list"
          aria-label="Toggle page contents"
          @click="contentsOpen = !contentsOpen"
        >
          <img :src="icon('contents')" alt="" />
        </button>
        <template v-if="contentsOpen"
          ><h2>ON THIS PAGE</h2>
          <nav id="heading-list">
            <button v-for="h in headings" :key="h.id" @click="scrollTo(h.id)">{{ h.text }}</button>
          </nav></template
        >
      </aside>
      <main class="document-column">
        <div v-if="error" class="error" role="alert">{{ error }}</div>
        <p v-if="!page && !error" role="status">Opening the workspace…</p>
        <article v-if="page">
          <div class="title-row">
            <h1>{{ page.title }}</h1>
            <div class="page-actions">
              <button v-if="!editing" class="compact" @click="beginEdit" :disabled="busy">
                <img :src="icon('edit')" alt="" />{{
                  page.kind === 'recording' ? 'Edit transcript' : 'Edit page'
                }}</button
              ><button class="compact" @click="openHistory">
                <img :src="icon('history')" alt="" />Page history
              </button>
            </div>
          </div>
          <p class="metadata">
            {{
              page.kind === 'recording'
                ? 'Primary recording · Example transcript excerpt'
                : 'Documentation'
            }}
            · {{ page.revision === 1 ? 'Example workspace' : 'Edited by you' }} · Revision
            {{ page.revision }}
          </p>
          <p v-if="status" class="status" role="status">{{ status }}</p>
          <div v-if="editing" class="editor-toolbar" aria-label="Editing controls">
            <span>Editing</span
            ><button @click="editor?.chain().focus().toggleMark('bold').run()">Bold</button
            ><button @click="editor?.chain().focus().toggleMark('italic').run()">Italic</button
            ><span class="draft-status" role="status">{{ draftState }}</span
            ><button :disabled="busy" @click="cancel">Cancel</button
            ><button class="primary" :disabled="busy" @click="save">Save page</button>
          </div>
          <div v-if="selectionAnchor && !editing" class="passage-actions">
            <span>Selected passage</span
            ><button class="primary" @mousedown.prevent @click="startComment">Comment</button>
          </div>
          <EditorContent
            v-if="editor"
            :editor="editor"
            :class="{ editing }"
            @click="citationClick"
          />
        </article>
      </main>
      <aside class="context-rail" aria-label="Page context">
        <div class="rail-sticky">
          <div class="create-actions">
            <button class="compact" disabled title="Page creation is planned for a later milestone">
              <img :src="icon('plus')" alt="" />Add page</button
            ><button
              class="compact"
              disabled
              title="Artifact intake is planned for a later milestone"
            >
              <img :src="icon('plus')" alt="" />Add artifact
            </button>
          </div>
          <div class="tabs" role="tablist" aria-label="Page context">
            <button
              id="references-tab"
              role="tab"
              :aria-selected="activeTab === 'references'"
              aria-controls="references-panel"
              @click="activeTab = 'references'"
            >
              References</button
            ><button
              id="comments-tab"
              role="tab"
              :aria-selected="activeTab === 'comments'"
              aria-controls="comments-panel"
              @click="activeTab = 'comments'"
            >
              Comments
            </button>
          </div>
        </div>
        <ReviewRail
          v-if="page"
          ref="reviewRail"
          :page="page"
          :tab="activeTab"
          :selection="selectionAnchor"
          :editing="editing"
          @published="loadPage(page!.id)"
          @preview="reviewPreview = $event"
          @settings="openSettings"
          @clear-selection="clearSelection"
          @locate="(a) => scrollTo(a.blockId)"
        />
      </aside>
    </div>
    <footer>The Lab <span>Development build · Fictional example workspace</span></footer>
    <aside
      v-if="chatOpen"
      class="chat-drawer"
      aria-label="Agent global chat"
      @keydown.esc="toggleChat"
    >
      <div class="chat-title">
        <h2>Agent global chat</h2>
        <button ref="chatClose" class="compact" @click="toggleChat">
          <img :src="icon('chat')" alt="" />Close chat
        </button>
      </div>
      <p>Ask the agent about your workspace.</p>
      <p class="muted">
        Global chat is planned for a later milestone. You can already ask @agent in a passage’s
        comment thread.
      </p>
      <textarea aria-label="Message Agent" placeholder="Message Agent…" disabled></textarea>
    </aside>
    <RevisionHistory
      v-if="page"
      ref="revisionHistory"
      :page="page"
      :discussions="reviewRail?.discussions ?? []"
      @thread="showHistoryThread"
      @evidence="(e) => reviewRail?.openEvidence(e)"
    />
  </div>
</template>
