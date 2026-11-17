import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile, rm } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-import')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-import-'))
const port = 4448,
  base = `http://127.0.0.1:${port}`
let child,
  session,
  logs = '',
  livePhase = false
await mkdir(artifacts, { recursive: true })
const delay = (ms) => new Promise((r) => setTimeout(r, ms))
async function request(path, body, method = body === undefined ? 'GET' : 'POST') {
  const response = await fetch(base + path, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: body === undefined ? undefined : JSON.stringify(body),
    signal: AbortSignal.timeout(35000),
  })
  const result = await response.json()
  if (!response.ok) throw Error(JSON.stringify(result.value))
  return result.value
}
async function until(fn, description) {
  const end = Date.now() + 180000
  while (Date.now() < end) {
    if (await fn()) return
    await delay(100)
  }
  throw Error(`Timed out: ${description}`)
}
const endpoint = (path) => `/session/${session}${path}`
const execute = (script, args = []) => request(endpoint('/execute/sync'), { script, args })
const value = (selector) =>
  execute('return document.querySelector(arguments[0])?.textContent ?? null', [selector])
async function element(selector) {
  const e = await request(endpoint('/element'), { using: 'css selector', value: selector })
  return e['element-6066-11e4-a52e-4f735466cecf']
}
async function click(selector) {
  await request(endpoint(`/element/${await element(selector)}/click`), {})
  await delay(50)
}
async function clickButton(text) {
  const index = await execute(
    'return [...document.querySelectorAll("button")].findIndex(e=>e.textContent.trim()===arguments[0])',
    [text],
  )
  assert.ok(index >= 0, `Missing button ${text}`)
  await execute(
    'const e=[...document.querySelectorAll("button")][arguments[0]];e.dataset.testButton="target";return true',
    [index],
  )
  await click('[data-test-button="target"]')
  await execute(
    'document.querySelector("[data-test-button]")?.removeAttribute("data-test-button");return true',
  )
}
async function screenshot(name) {
  const image = await request(endpoint('/screenshot'))
  await writeFile(join(artifacts, `${name}.png`), Buffer.from(image, 'base64'))
}
async function launch() {
  child = spawn(join(root, 'src-tauri/target/debug/agentic-wiki'), [], {
    cwd: root,
    env: { ...process.env, WIKI_E2E_DATA_DIR: data, TAURI_WEBDRIVER_PORT: String(port) },
  })
  child.stdout.on('data', (s) => {
    logs += s
  })
  child.stderr.on('data', (s) => {
    logs += s
  })
  await until(async () => {
    try {
      return (await request('/status')).ready
    } catch {
      return false
    }
  }, 'native WebDriver startup')
  const created = await request('/session', { capabilities: { alwaysMatch: {} } })
  session = created.sessionId
  await request(endpoint('/timeouts'), { script: 10000 })
  await until(async () => {
    if (child.exitCode !== null) throw Error('The native app exited before its page loaded.')
    try {
      return !!(await value('h1'))
    } catch (error) {
      if (/timeout|timed out/i.test(String(error))) return false
      throw error
    }
  }, 'native page load')
  assert.equal(await execute('return !!window.__TAURI_INTERNALS__'), true, 'real native IPC')
}
async function stop() {
  if (child) {
    const ended = once(child, 'exit')
    child.kill('SIGTERM')
    await ended
    child = undefined
    await delay(150)
  }
}
async function invoke(command, args = {}) {
  await execute(
    'window.__reviewResult = null; window.__TAURI_INTERNALS__.invoke(arguments[0],arguments[1]).then(value=>{window.__reviewResult={ok:true,value}},error=>{window.__reviewResult={ok:false,error:String(error)}});return true',
    [command, args],
  )
  await until(
    async () => !!(await execute('return window.__reviewResult')),
    'native command ' + command,
  )
  const result = await execute('return window.__reviewResult')
  if (!result.ok) throw Error(result.error)
  return result.value
}
async function type(selector, text, clear = false) {
  const id = await element(selector)
  if (clear) await request(endpoint(`/element/${id}/clear`), {})
  await request(endpoint(`/element/${id}/value`), { text })
}
const local = join(data, 'sources')
const bridge = join(data, 'agent-hands/use-google-drive/scripts')
await mkdir(local, { recursive: true })
await mkdir(bridge, { recursive: true })
const transcript =
  '00:12 Dana: Use the booking calendar.\n\n00:20 Mei: I will check the equipment page.'
await writeFile(join(local, 'meeting.txt'), transcript)
await writeFile(join(local, 'cloud.gdoc'), 'Pointer only')
await writeFile(
  join(bridge, 'export.txt'),
  '00:18 Dana: ' + 'This is a synthetic transcript. '.repeat(1000) + '\n\nEND OF FULL EXPORT',
)
await writeFile(join(bridge, 'mode'), 'ok')
await writeFile(
  join(bridge, 'drive'),
  `#!/bin/bash
set -eu
cd "$(dirname "$0")"
[ "$(cat mode)" != fail ] || exit 7
case "$1" in
list) if [ "$3" = folder1 ]; then printf '%s' '{"files":[{"id":"meeting1","name":"Meeting folder","mimeType":"application/vnd.google-apps.folder","parents":["folder1"]}]}';exit 0;fi; printf '%s' '{"files":[{"id":"doc1","name":"Drive meeting","mimeType":"application/vnd.google-apps.document","parents":["folder1"]}],"nextPageToken":"more"}' ;;
meta) if [ "$3" = meeting1 ]; then printf '%s' '{"id":"meeting1","name":"Meeting folder","mimeType":"application/vnd.google-apps.folder","parents":["folder1"]}';exit 0;fi; if [ "$3" = folder1 ]; then printf '%s' '{"id":"folder1","name":"Meet","mimeType":"application/vnd.google-apps.folder","parents":[]}';exit 0;fi; printf '%s' '{"id":"doc1","name":"Drive meeting","mimeType":"application/vnd.google-apps.document","modifiedTime":"2026-11-17T12:00:00Z","parents":["meeting1"]}' ;;
download) [ "$4" = --out ] && [ "$6" = --mime ] && [ "$7" = text/plain ]; cp export.txt "$5" ;;
*) exit 9 ;;
esac
`,
)
async function choose(selector, value) {
  await execute(
    'const e=document.querySelector(arguments[0]);e.value=arguments[1];e.dispatchEvent(new Event("change",{bubbles:true}));return true',
    [selector, value],
  )
}
async function openImport() {
  await clickButton('Add artifact')
  await until(
    async () => await execute('return !!document.querySelector(".import-dialog[open]")'),
    'import dialog',
  )
}
async function selectLocal() {
  await choose('[aria-label="Import source"]', 'local')
  await clickButton('Browse source folder')
  await until(async () => (await value('.source-list'))?.includes('meeting.txt'), 'local files')
  await clickButton('meeting.txt')
  await until(async () => !!(await value('.import-preview')), 'local preview')
}
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error(`Test port ${port} is already in use.`)
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  await click('.settings-button')
  await until(async () => !!(await value('.source-settings')), 'source settings')
  await type('[aria-label="Local source folder"]', local, true)
  await type('[aria-label="Agent Hands Drive tool"]', join(bridge, 'drive'), true)
  await type('[aria-label="Google account alias"]', 'example', true)
  await type('[aria-label="Meet folder link or ID"]', 'folder1', true)
  await clickButton('Save source settings')
  await until(
    async () => (await value('.source-settings'))?.includes('Source settings saved'),
    'saved settings',
  )
  await screenshot('source-settings')
  await clickButton('Back to page')
  await openImport()
  await selectLocal()
  assert.equal(await value('.import-preview'), transcript)
  await click('[aria-label="Close import"]')
  assert.equal((await invoke('imported_sources')).length, 0, 'cancel creates no artifact')
  await openImport()
  await selectLocal()
  await type('[aria-label="Import title"]', 'Team meeting transcript', true)
  await choose('[aria-label="Imported content type"]', 'transcript')
  await click('.import-parent input')
  await screenshot('transcript-preview')
  await click('.import-dialog button[type="submit"]')
  await until(async () => (await value('h1')) === 'Team meeting transcript', 'saved artifact')
  const first = (await invoke('imported_sources'))[0]
  assert.equal(first.parentId, 'getting-started')
  assert.match(await value('.metadata'), /Imported transcript/)
  assert.match(await value('.ProseMirror'), /00:20 Mei/)
  await screenshot('imported-transcript')
  await clickButton('Page history')
  await until(
    async () => (await value('.revision-list'))?.includes('source import'),
    'import attribution',
  )
  await click('.history-dialog button')
  await openImport()
  await selectLocal()
  assert.match(await value('.import-dialog'), /already imported/)
  await click('.import-dialog button[type="submit"]')
  await until(
    async () => await execute('return !document.querySelector(".import-dialog[open]")'),
    'duplicate closes',
  )
  assert.equal((await invoke('imported_sources')).length, 1)
  assert.equal((await invoke('read_page', { id: first.id })).revision, 1)
  await writeFile(join(local, 'meeting.txt'), '00:12 Dana: Use the replacement booking calendar.')
  await openImport()
  await selectLocal()
  assert.match(await value('.import-dialog'), /source has changed/)
  await click('.import-dialog button[type="submit"]')
  await until(
    async () => (await value('.ProseMirror'))?.includes('replacement booking calendar'),
    'refreshed artifact',
  )
  assert.equal((await invoke('read_page', { id: first.id })).revision, 2)
  const old = await invoke('read_revision', { id: first.id, revision: 1 })
  assert.match(JSON.stringify(old), /00:20 Mei/)
  await clickButton('Chat')
  await clickButton('Import a transcript or notes')
  await choose('[aria-label="Import source"]', 'drive')
  await clickButton('Browse source folder')
  await until(async () => (await value('.source-list'))?.includes('Meeting folder'), 'Meet folders')
  await clickButton('▸ Meeting folder')
  await until(
    async () => (await value('.source-list'))?.includes('Drive meeting'),
    'nested Drive list',
  )
  await clickButton('Back one folder')
  await until(
    async () => (await value('.source-list'))?.includes('Meeting folder'),
    'Meet parent folder',
  )
  await clickButton('▸ Meeting folder')
  await until(
    async () => (await value('.source-list'))?.includes('Drive meeting'),
    'return to meeting folder',
  )
  assert.match(await value('.import-dialog'), /not the full folder/)
  await clickButton('Drive meeting')
  await until(async () => !!(await value('.import-preview')), 'Drive preview')
  const full = await value('.import-preview')
  assert.ok(full.length > 20000)
  assert.ok(full.endsWith('END OF FULL EXPORT'))
  await type('[aria-label="Import title"]', 'Drive meeting notes', true)
  await choose('[aria-label="Imported content type"]', 'notes')
  await click('.import-dialog button[type="submit"]')
  await until(async () => (await value('h1')) === 'Drive meeting notes', 'Drive artifact')
  assert.match(await value('.metadata'), /Imported notes/)
  assert.match(await value('.source-provenance'), /Google Drive/)
  assert.equal(await execute('return !!document.querySelector(".chat-drawer")'), false)
  await screenshot('imported-drive-notes')
  const records = await invoke('imported_sources')
  assert.equal(records.length, 2)
  await openImport()
  await choose('[aria-label="Import source"]', 'drive')
  await writeFile(join(bridge, 'mode'), 'fail')
  await type(
    '[aria-label="Google Doc link or ID"]',
    'https://docs.google.com/document/d/doc1/edit',
    true,
  )
  await clickButton('Preview document')
  await until(
    async () => (await value('.import-dialog [role="alert"]'))?.includes('failed'),
    'export error',
  )
  assert.equal((await invoke('imported_sources')).length, 2)
  await click('[aria-label="Close import"]')
  await stop()
  await launch()
  await until(
    async () => (await value('#wiki-tree'))?.includes('Team meeting transcript'),
    'sources after restart',
  )
  await clickButton('Team meeting transcript')
  await until(
    async () => (await value('.ProseMirror'))?.includes('replacement booking calendar'),
    'reopened transcript',
  )
  assert.equal((await invoke('imported_sources')).length, 2)
  assert.equal((await invoke('source_settings')).googleAccount, 'example')
  const liveDoc = process.env.WIKI_LIVE_IMPORT_DOC,
    liveAccount = process.env.WIKI_LIVE_IMPORT_ACCOUNT,
    liveTool = process.env.WIKI_LIVE_IMPORT_TOOL
  let live = false
  if (liveDoc && liveAccount && liveTool) {
    livePhase = true
    await invoke('save_source_settings', {
      settings: {
        localFolder: '',
        agentHandsPath: liveTool,
        googleAccount: liveAccount,
        driveFolder: process.env.WIKI_LIVE_IMPORT_FOLDER ?? '',
      },
    })
    if (process.env.WIKI_LIVE_IMPORT_FOLDER) {
      const rootListing = await invoke('list_import_sources', { provider: 'drive', folderId: null })
      assert.ok(rootListing.items.length)
    }
    const preview = await invoke('preview_source_import', { provider: 'drive', key: liveDoc })
    assert.ok(preview.text.trim().length > 0)
    const imported = await invoke('confirm_source_import', {
      token: preview.token,
      title: 'Private live-import verification',
      kind: process.env.WIKI_LIVE_IMPORT_KIND ?? 'notes',
      parentId: null,
    })
    const reopened = await invoke('read_page', { id: imported.id })
    assert.equal(reopened.revision, 1)
    assert.match(preview.sourceUrl, /^https:\/\/docs.google.com\/document\/d\//)
    const text = reopened.document.doc.content
      .map((block) =>
        block.content
          .map((part) => (part.type === 'hardBreak' ? '\n' : (part.text ?? '')))
          .join(''),
      )
      .join('\n\n')
    assert.equal(
      text.trim(),
      preview.text
        .split('\n\n')
        .filter((part) => part.trim())
        .join('\n\n')
        .trim(),
    )
    await invoke('cancel_source_import', { token: preview.token })
    await stop()
    await launch()
    const persisted = await invoke('read_page', { id: imported.id })
    assert.deepEqual(persisted.document, reopened.document)
    live = true
  }
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: true,
        native: true,
        liveProvider: live,
        checks: [
          'settings saved and reopened',
          'local source preview and cancel',
          'explicit transcript/notes choice',
          'original text and provenance',
          'same source retry without duplicates',
          'changed source creates revision and keeps history',
          'Chat and Add artifact share importer',
          'Drive full export beyond 20,000 characters',
          'nested meeting folders and back navigation',
          'partial folder listing notice',
          'failed export leaves saved artifacts unchanged',
          'artifact navigation after restart',
        ],
      },
      null,
      2,
    ),
  )
  console.log(
    'PASS native source import, confirmation, retry, refresh, history and restart.' +
      (live
        ? ' Live Agent Hands export/import also passed.'
        : ' Drive used a labeled local fixture; live access is separate.'),
  )
} catch (e) {
  if (session && !livePhase) await screenshot('failure').catch(() => {})
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: false,
        error: livePhase ? 'Live source import failed; private source details omitted.' : String(e),
      },
      null,
      2,
    ),
  )
  throw livePhase ? Error('Live source import failed; private source details omitted.') : e
} finally {
  await stop()
  await writeFile(join(artifacts, 'native.log'), logs)
  await rm(data, { recursive: true, force: true })
}
