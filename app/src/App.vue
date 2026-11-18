<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import { hasUnsupportedPaste, wikiExtensions } from './editor'
import { api } from './api'
import { authoringApi } from './authoringApi'
import type { ReferenceEntry } from './generated/ReferenceEntry'
const references = ref<ReferenceEntry[]>([]),
  citationsOnly = ref(false),
  draftTitle = ref(''),
  historyOpen = ref(false)
const noteDialog = ref<HTMLDialogElement>(),
  noteTitle = ref(''),
  noteBody = ref(''),
  noteError = ref('')
let referenceSequence = 0
async function refreshReferences() {
  if (!editor.value) return
  const seq = ++referenceSequence
  try {
    const entries = await authoringApi.references(readDocument())
    if (seq === referenceSequence) references.value = entries
  } catch (e) {
    error.value = String(e)
  }
}

import ReviewRail from './ReviewRail.vue'
import AgentSettings from './AgentSettings.vue'
import GlobalChat from './GlobalChat.vue'
import AnswerDraftEditor from './AnswerDraftEditor.vue'
import { chatApi } from './chatApi'
import type { AnswerDraft } from './generated/AnswerDraft'
import type { ChatTurn } from './generated/ChatTurn'
const answerDraft = ref<AnswerDraft | null>(null)
const pageOrigin = ref<ChatTurn | null>(null)
const globalChat = ref<InstanceType<typeof GlobalChat>>()
import SelectionActions from './SelectionActions.vue'
import ImportSource from './ImportSource.vue'
import { sourceApi } from './sourceApi'
import type { ImportedSource } from './generated/ImportedSource'
const importedSources = ref<ImportedSource[]>([])
const importer = ref<InstanceType<typeof ImportSource>>()
const currentSource = computed(() => importedSources.value.find((s) => s.id === page.value?.id))
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
import WikiTree from './WikiTree.vue'
import PageDetails from './PageDetails.vue'
import PassagePicker from './PassagePicker.vue'
import { workspaceApi } from './workspaceApi'
import { ancestors } from './wikiTree'
import type { WikiRecord } from './generated/WikiRecord'
import type { PassageResult } from './generated/PassageResult'
import type { Evidence } from './generated/Evidence'
const records = ref<WikiRecord[]>([])
const pageDetails = ref<InstanceType<typeof PageDetails>>()
const passagePicker = ref<InstanceType<typeof PassagePicker>>()
const breadcrumb = computed(() => ancestors(records.value, page.value?.id ?? ''))
let citationTarget: {
  pageId: string
  revision: number
  anchor: PassageAnchor
  editing: boolean
} | null = null
async function detailsSaved(saved: Page, created: boolean) {
  await refreshSources()
  await loadPage(saved.id)
  if (created) await beginEdit()
}
async function openSearchHit(hit: PassageResult) {
  await loadPage(hit.source.objectId)
  await nextTick()
  if (page.value?.id !== hit.source.objectId) return
  if (page.value.revision !== hit.source.revision) {
    error.value = 'This page changed since the search. Search again for its current wording.'
    return
  }
  scrollTo(hit.source.blockId)
  status.value = 'Opened the matching passage.'
}
function captureAnnotation() {
  if (!page.value || !selectionAnchor.value) return false
  citationTarget = {
    pageId: page.value.id,
    revision: page.value.revision,
    anchor: { ...selectionAnchor.value },
    editing: editing.value,
  }
  return true
}
async function startCitation() {
  if (captureAnnotation()) await passagePicker.value?.open('cite')
}
async function startNote() {
  if (!captureAnnotation()) return
  noteTitle.value = ''
  noteBody.value = ''
  noteError.value = ''
  await nextTick()
  noteDialog.value?.showModal()
}
function annotationPosition() {
  const target = citationTarget,
    e = editor.value
  if (
    !target ||
    !e ||
    target.pageId !== page.value?.id ||
    target.revision !== page.value?.revision ||
    target.editing !== editing.value
  )
    throw Error('The page changed. Select the passage again.')
  let position: number | undefined
  e.state.doc.forEach((block, pos) => {
    if (block.attrs.blockId !== target.anchor.blockId) return
    const text = block.textBetween(0, block.content.size, '', (leaf) =>
      leaf.type.name === 'hardBreak' ? '\n' : '\uFFFC',
    )
    if (text.slice(target.anchor.fromUtf16, target.anchor.toUtf16) === target.anchor.quote)
      position = pos + 1 + target.anchor.toUtf16
  })
  if (position === undefined)
    throw Error('The selected passage changed. Select it again before adding a reference.')
  return position
}
async function insertReference(entry: ReferenceEntry) {
  const e = editor.value!,
    pos = annotationPosition(),
    target = citationTarget!
  const atom =
    entry.kind === 'note'
      ? { type: 'note', attrs: { noteId: entry.id } }
      : { type: 'citation', attrs: { citationId: entry.id } }
  const node = e.schema.nodeFromJSON(atom)
  if (editing.value) {
    e.view.dispatch(e.state.tr.insert(pos, node))
    await flushDraft()
    await refreshReferences()
    status.value = 'Reference added to your draft. Save the page to publish it.'
  } else {
    const document = {
      schemaVersion: 1,
      doc: e.state.tr.insert(pos, node).doc.toJSON(),
    } as RichDocument
    await api.save(target.pageId, target.revision, document)
    busy.value = false
    await loadPage(target.pageId)
    status.value =
      entry.kind === 'note' ? 'Margin note saved.' : 'Citation saved with its source revision.'
  }
  activeTab.value = 'references'
  citationTarget = null
  selectionAnchor.value = null
}
async function cite(evidence: Evidence) {
  if (!citationTarget || busy.value) return
  busy.value = true
  try {
    annotationPosition()
    await insertReference(await authoringApi.citation(evidence.source))
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function saveNote() {
  if (!citationTarget || busy.value) return
  busy.value = true
  noteError.value = ''
  try {
    annotationPosition()
    await insertReference(await authoringApi.note(noteTitle.value, noteBody.value))
    noteDialog.value?.close()
  } catch (e) {
    noteError.value = String(e)
  } finally {
    busy.value = false
  }
}
function tabsKey(event: KeyboardEvent) {
  if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return
  event.preventDefault()
  activeTab.value =
    event.key === 'Home'
      ? 'references'
      : event.key === 'End'
        ? 'comments'
        : activeTab.value === 'references'
          ? 'comments'
          : 'references'
  void nextTick(() => document.getElementById(`${activeTab.value}-tab`)?.focus())
}

const page = ref<Page | null>(null)
const editor = shallowRef<Editor>()
const editing = ref(false),
  busy = ref(false),
  treeOpen = ref(true),
  contentsOpen = ref(true),
  chatOpen = ref(false)
const activeTab = ref<'references' | 'comments'>('references')
const error = ref(''),
  status = ref(''),
  draftState = ref(''),
  selectedText = ref('')
const revisionHistory = ref<InstanceType<typeof RevisionHistory>>()
const chatButton = ref<HTMLButtonElement>()
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
  void refreshReferences()
  clearTimeout(draftTimer)
  draftState.value = 'Saving draft…'
  draftTimer = setTimeout(() => queueDraft(), 250)
}
function queueDraft() {
  if (!editing.value || !page.value) return
  const id = page.value.id,
    revision = page.value.revision,
    document = readDocument(),
    title = draftTitle.value
  draftWrites = draftWrites
    .catch(() => {})
    .then(() => api.putDraft(id, revision, document, title))
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
    const [loaded, origin] = await Promise.all([api.read(id), chatApi.origin(id)])
    if (seq !== loadSequence) return
    editor.value?.destroy()
    reviewPreview.value = { proposal: null, anchor: null, editing: false }
    page.value = loaded
    historyOpen.value = false
    references.value = []
    draftTitle.value = loaded.title
    pageOrigin.value = origin
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
        if (!empty && $from.sameParent($to) && $from.parent.type.name === 'paragraph') {
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
    await refreshReferences()
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
    draftTitle.value = draft?.title ?? page.value.title
    if (draft) editor.value?.commands.setContent(draft.document.doc)
    await refreshReferences()
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
    const saved = await api.save(
      page.value.id,
      page.value.revision,
      readDocument(),
      draftTitle.value,
    )
    page.value = saved
    editing.value = false
    editor.value?.setEditable(false)
    await clearSelection()
    draftState.value = ''
    status.value = `Saved revision ${saved.revision}.`
    await refreshSources()
    await refreshReferences()
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
    await clearSelection()
    draftState.value = ''
    draftTitle.value = page.value.title
    await refreshReferences()
    status.value = 'Draft discarded. Saved page unchanged.'
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function openHistory() {
  try {
    if (editing.value) {
      error.value = 'Save or cancel your edit before opening history.'
      return
    }
    chatOpen.value = false
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
  if (chatOpen.value) globalChat.value?.focus()
  else chatButton.value?.focus()
}
function goHome() {
  settingsOpen.value = false
  void loadPage('getting-started')
}
function openSettings() {
  chatOpen.value = false
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
function returnToCitation(id: string) {
  const marker = document.querySelector<HTMLElement>(
    `.document-column .ProseMirror [data-citation-id="${CSS.escape(id)}"], .document-column .ProseMirror [data-note-id="${CSS.escape(id)}"]`,
  )
  marker?.scrollIntoView({ behavior: 'smooth', block: 'center' })
  marker?.focus({ preventScroll: true })
}
function citationClick(event: MouseEvent) {
  const target = event.target as HTMLElement
  const marker = target.closest<HTMLElement>('[data-citation-id], [data-note-id]')
  const id = marker?.dataset.citationId ?? marker?.dataset.noteId
  if (id) {
    activeTab.value = 'references'
    void nextTick(() => reviewRail.value?.openReference(id))
    return
  }
  const thread = target.closest<HTMLElement>('[data-thread-anchor]')?.dataset.threadAnchor
  if (thread) {
    activeTab.value = 'comments'
    void nextTick(() => reviewRail.value?.selectThread(thread))
  }
}
function articleKey(event: KeyboardEvent) {
  const target = event.target as HTMLElement
  if ((event.key === 'Enter' || event.key === ' ') && target.matches('[data-thread-anchor]')) {
    event.preventDefault()
    target.click()
  }
}
async function showHistoryThread(d: Discussion) {
  await reviewRail.value?.inspectThread(d)
}
async function refreshSources() {
  const [sources, pages] = await Promise.all([sourceApi.imported(), workspaceApi.records()])
  importedSources.value = sources
  records.value = pages
}
async function imported(source: Page) {
  chatOpen.value = false
  await refreshSources()
  await loadPage(source.id)
}
async function openOriginal() {
  if (!page.value) return
  try {
    await sourceApi.openOriginal(page.value.id)
  } catch (e) {
    error.value = String(e)
  }
}
function openAnswerDraft(draft: AnswerDraft) {
  if (editing.value || busy.value || settingsOpen.value) return
  answerDraft.value = draft
  chatOpen.value = false
}
function closeAnswerDraft() {
  answerDraft.value = null
  chatOpen.value = true
}
async function openChatPage(id: string) {
  if (editing.value || busy.value) return
  chatOpen.value = false
  await loadPage(id)
}
async function publishAnswer(saved: Page) {
  answerDraft.value = null
  await refreshSources()
  await loadPage(saved.id)
  status.value = 'Page saved. Its AI origin and source citations are retained.'
}
async function showAnswerOrigin() {
  if (!pageOrigin.value) return
  chatOpen.value = true
  await nextTick()
  await globalChat.value?.focusTurn(pageOrigin.value)
}
const headerElement = ref<HTMLElement>()
const headerReflow = ref(false)
const headerHeight = ref(100)
let headerObserver: ResizeObserver | undefined
function measureHeader() {
  const header = headerElement.value
  if (!header) return
  const actions = header.querySelector<HTMLElement>('.global-actions')!
  const brand = header.querySelector<HTMLElement>('.brand')!
  // Text enlargement can exhaust the centered search's side columns without changing the viewport width.
  const searchWidth = innerWidth > 1000 ? 375 : 230
  const sideRoom = (header.clientWidth - searchWidth) / 2 - 32
  headerReflow.value =
    innerWidth > 650 && (actions.scrollWidth > sideRoom || brand.offsetWidth + 72 > sideRoom)
  if (innerWidth <= 650) headerReflow.value = parseFloat(getComputedStyle(brand).fontSize) > 28
  headerHeight.value = header.offsetHeight
}
const narrowLayout = window.matchMedia('(max-width: 1000px)')
const compactContents = window.matchMedia('(max-width: 1200px)')
function adaptLayout() {
  if (narrowLayout.matches) treeOpen.value = false
  if (compactContents.matches) contentsOpen.value = false
}
onMounted(async () => {
  adaptLayout()
  headerObserver = new ResizeObserver(measureHeader)
  if (headerElement.value) {
    headerObserver.observe(headerElement.value)
    for (const element of headerElement.value.querySelectorAll('.global-actions, .brand'))
      headerObserver.observe(element)
  }
  narrowLayout.addEventListener('change', adaptLayout)
  compactContents.addEventListener('change', adaptLayout)
  await loadPage('getting-started')
  try {
    await refreshSources()
  } catch (e) {
    error.value = String(e)
  }
})
onBeforeUnmount(() => {
  headerObserver?.disconnect()
  narrowLayout.removeEventListener('change', adaptLayout)
  compactContents.removeEventListener('change', adaptLayout)
  clearTimeout(draftTimer)
  editor.value?.destroy()
})
</script>

<template>
  <div
    class="application"
    :class="{ 'chat-open': chatOpen }"
    :style="headerReflow ? { '--header-height': `${headerHeight}px` } : undefined"
  >
    <header ref="headerElement" class="wiki-header" :class="{ 'header-reflow': headerReflow }">
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
        <button class="brand" :disabled="!!answerDraft" @click="goHome">The Lab</button>
        <nav v-if="settingsOpen" class="breadcrumbs" aria-label="Breadcrumb">
          <span>Settings</span><span>/</span><span class="crumb-current">Agent and workspace</span>
        </nav>
        <nav v-else class="breadcrumbs" aria-label="Breadcrumb">
          <template v-for="(r, index) in breadcrumb" :key="r.id"
            ><span v-if="index">/</span
            ><button
              :class="{ 'crumb-current': r.id === page?.id }"
              :disabled="!!answerDraft"
              :title="r.title"
              @click="loadPage(r.id)"
            >
              {{ r.title }}
            </button></template
          ><span v-if="historyOpen">/ History</span>
        </nav>
      </div>
      <button
        class="search search-launch"
        aria-label="Search wiki"
        :disabled="editing || busy || !!answerDraft"
        @click="passagePicker?.open('navigate')"
      >
        <img :src="icon('search')" alt="" />Search wiki
      </button>
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
          :disabled="!!answerDraft"
          :aria-current="settingsOpen ? 'page' : undefined"
          @click="openSettings"
        >
          <img :src="icon('settings')" alt="" />Settings
        </button>
        <button
          ref="chatButton"
          :disabled="!!answerDraft"
          class="compact"
          :aria-expanded="chatOpen"
          @click="toggleChat"
        >
          <img :src="icon('chat')" alt="" />Chat
        </button>
      </nav>
    </header>
    <AgentSettings v-if="settingsOpen" @back="closeSettings" />
    <AnswerDraftEditor
      v-if="answerDraft"
      :draft="answerDraft"
      :records="records"
      @close="closeAnswerDraft"
      @published="publishAnswer"
    />
    <div
      v-show="!settingsOpen && !answerDraft && !historyOpen"
      class="workspace"
      :class="{ 'tree-hidden': !treeOpen, 'contents-hidden': !contentsOpen }"
    >
      <aside v-if="treeOpen" id="wiki-tree" class="wiki-tree" aria-label="Wiki pages">
        <WikiTree :records="records" :active="page?.id" @open="loadPage" />
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
            <input
              v-if="editing"
              v-model="draftTitle"
              class="page-title-input"
              aria-label="Page title"
              maxlength="240"
              @input="scheduleDraft"
            />
            <h1 v-else>{{ page.title }}</h1>
            <div class="page-actions">
              <button v-if="!editing" class="compact" @click="beginEdit" :disabled="busy">
                <img :src="icon('edit')" alt="" />{{
                  page.kind === 'recording' || currentSource?.kind === 'transcript'
                    ? 'Edit transcript'
                    : 'Edit page'
                }}</button
              ><button class="compact" @click="openHistory">
                <img :src="icon('history')" alt="" />Page history
              </button>
            </div>
          </div>
          <p class="metadata">
            {{
              currentSource
                ? currentSource.kind === 'transcript'
                  ? 'Imported transcript'
                  : 'Imported notes'
                : page.kind === 'recording'
                  ? 'Primary recording · Example transcript excerpt'
                  : 'Documentation'
            }}
            ·
            {{
              pageOrigin
                ? 'Created from an AI answer · Published by you'
                : currentSource
                  ? 'Saved on this device'
                  : page.revision === 1 &&
                      ['getting-started', 'orientation-recording'].includes(page.id)
                    ? 'Example workspace'
                    : 'Edited by you'
            }}
            · Revision
            {{ page.revision }}
          </p>
          <button v-if="pageOrigin" class="text-link answer-origin" @click="showAnswerOrigin">
            View originating Chat answer
          </button>
          <div v-if="currentSource" class="source-provenance">
            <p>
              Imported from
              {{ currentSource.provider === 'drive' ? 'Google Drive' : 'a local file' }}:
              {{ currentSource.sourceName }} · Source snapshot saved with revision
              {{ currentSource.importedRevision }}.
            </p>
            <button v-if="currentSource.sourceUrl" class="text-link" @click="openOriginal">
              Open original Google Doc ↗
            </button>
            <button
              v-if="currentSource.parentId"
              class="text-link"
              @click="loadPage(currentSource.parentId)"
            >
              Open linked page
            </button>
          </div>
          <p v-if="status" class="status" role="status">{{ status }}</p>
          <div v-if="editing" class="editor-toolbar" aria-label="Editing controls">
            <span>Editing</span
            ><button @click="editor?.chain().focus().toggleMark('bold').run()">Bold</button
            ><button @click="editor?.chain().focus().toggleMark('italic').run()">Italic</button
            ><span class="draft-status" role="status">{{ draftState }}</span
            ><button :disabled="busy" @click="cancel">Cancel</button
            ><button class="primary" :disabled="busy" @click="save">Save page</button>
          </div>
          <SelectionActions
            v-if="editor"
            :editor="editor"
            :enabled="!settingsOpen && !chatOpen && !historyOpen"
            :editing="editing"
            @comment="startComment"
            @citation="startCitation"
            @note="startNote"
          />
          <EditorContent
            v-if="editor"
            :editor="editor"
            :class="{ editing }"
            @click="citationClick"
            @keydown="articleKey"
          />
          <details v-if="references.length" class="page-endnotes">
            <summary>Endnotes ({{ references.length }})</summary>
            <label class="reference-filter"
              ><input type="checkbox" role="switch" v-model="citationsOnly" />Citations only</label
            >
            <article
              v-for="(entry, index) in references"
              :key="entry.id"
              v-show="!citationsOnly || entry.kind === 'citation'"
            >
              <button
                v-if="entry.evidence"
                class="text-link"
                @click="reviewRail?.openEvidence(entry.evidence)"
              >
                {{ index + 1 }} · {{ entry.title }} · Revision {{ entry.evidence.source.revision }}
              </button>
              <h3 v-else>{{ index + 1 }} · {{ entry.title }}</h3>
              <blockquote>{{ entry.body }}</blockquote>
              <button class="text-link" @click="returnToCitation(entry.id)">
                Return to passage
              </button>
            </article>
          </details>
        </article>
      </main>
      <aside class="context-rail" aria-label="Page context">
        <div class="rail-sticky">
          <div class="create-actions">
            <button
              class="compact"
              :disabled="editing || busy || !!answerDraft"
              @click="pageDetails?.open(null, page?.id ?? null)"
            >
              <img :src="icon('plus')" alt="" />Add page</button
            ><button
              class="compact"
              :disabled="editing || busy || !!answerDraft"
              @click="importer?.open()"
            >
              <img :src="icon('plus')" alt="" />Add artifact
            </button>
          </div>
          <div class="tabs" role="tablist" aria-label="Page context" @keydown="tabsKey">
            <button
              id="references-tab"
              role="tab"
              :aria-selected="activeTab === 'references'"
              :tabindex="activeTab === 'references' ? 0 : -1"
              aria-controls="references-panel"
              @click="activeTab = 'references'"
            >
              References</button
            ><button
              id="comments-tab"
              role="tab"
              :aria-selected="activeTab === 'comments'"
              :tabindex="activeTab === 'comments' ? 0 : -1"
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
          :references="references"
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
    <RevisionHistory
      v-if="page"
      ref="revisionHistory"
      :page="page"
      :shown="historyOpen && !settingsOpen && !answerDraft"
      :discussions="reviewRail?.discussions ?? []"
      @open="historyOpen = true"
      @close="historyOpen = false"
      @thread="showHistoryThread"
      @evidence="(e) => reviewRail?.openEvidence(e)"
    />
    <footer>The Lab <span>Local workspace · Includes fictional starter pages</span></footer>
    <GlobalChat
      ref="globalChat"
      :open="chatOpen"
      :page="page"
      :can-open-draft="!editing && !busy && !settingsOpen"
      @close="toggleChat"
      @settings="openSettings"
      @import="importer?.open()"
      @draft="openAnswerDraft"
      @page="openChatPage"
    />
    <PageDetails ref="pageDetails" :records="records" @saved="detailsSaved" />
    <PassagePicker ref="passagePicker" @navigate="openSearchHit" @cite="cite" />
    <ImportSource ref="importer" :page="page" @imported="imported" @settings="openSettings" />
    <dialog ref="noteDialog" class="workspace-dialog" aria-labelledby="note-title">
      <form @submit.prevent="saveNote">
        <h2 id="note-title">Add margin note</h2>
        <p v-if="noteError" role="alert">{{ noteError }}</p>
        <label
          >Title<input v-model="noteTitle" required maxlength="240" aria-label="Margin note title"
        /></label>
        <label
          >Note<textarea
            v-model="noteBody"
            required
            maxlength="16000"
            rows="5"
            aria-label="Margin note text"
          />
        </label>
        <div class="review-actions">
          <button type="button" @click="noteDialog?.close()">Cancel</button
          ><button class="primary" :disabled="busy">Add note</button>
        </div>
      </form>
    </dialog>
  </div>
</template>
