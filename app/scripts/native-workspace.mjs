import assert from 'node:assert/strict'
import { spawn, execFile } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile, rm } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-workspace')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-workspace-'))
const port = 4450,
  base = `http://127.0.0.1:${port}`
let child,
  session,
  logs = '',
  livePhase = process.env.WIKI_RUN_LIVE_CHECK === '1'
await mkdir(artifacts, { recursive: true })
await rm(join(artifacts, 'result.json'), { force: true })
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
  const end = Date.now() + (description.includes('suggestion') ? 180000 : 45000)
  while (Date.now() < end) {
    if (await fn()) return
    await delay(100)
  }
  throw Error(
    `Timed out: ${description}${description === 'native window focus' ? '. Keep macOS unlocked and click inside the temporary test window.' : ''}`,
  )
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
    'return [...document.querySelectorAll("button")].findIndex(e=>e.getClientRects().length && e.textContent.trim()===arguments[0])',
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
  if (livePhase && (await execute('return !!document.querySelector(".settings-page")'))) return
  const image = await request(endpoint('/screenshot'))
  await writeFile(join(artifacts, `${name}.png`), Buffer.from(image, 'base64'))
}
async function launch() {
  child = spawn(join(root, 'src-tauri/target/debug/agentic-wiki'), [], {
    cwd: root,
    env: {
      ...process.env,
      WIKI_E2E_DATA_DIR: data,
      TAURI_WEBDRIVER_PORT: String(port),
      WIKI_E2E_LIVE_SUBSCRIPTION: livePhase ? '1' : '0',
    },
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
async function choose(selector, value) {
  await execute(
    'const e=document.querySelector(arguments[0]);e.value=arguments[1];e.dispatchEvent(new Event("change",{bubbles:true}));return true',
    [selector, value],
  )
}
async function create(title, text) {
  await clickButton('Add page')
  await type('[aria-label="Page title"]', title)
  await clickButton('Create page')
  await until(
    async () =>
      !!(await execute('return document.querySelector(".ProseMirror")?.isContentEditable')),
    'created editable page',
  )
  await type('.ProseMirror', text)
  await clickButton('Save page')
  await until(async () => (await value('.status'))?.includes('Saved revision'), 'saved new page')
  return (await invoke('wiki_records')).find((r) => r.title === title)
}
async function selectPassage(blockId) {
  if (process.platform === 'darwin')
    await new Promise((resolve, reject) =>
      execFile(
        '/usr/bin/osascript',
        [
          '-l',
          'JavaScript',
          '-e',
          `ObjC.import('AppKit'); $.NSRunningApplication.runningApplicationWithProcessIdentifier(${child.pid}).activateWithOptions(3)`,
        ],
        (error) => (error ? reject(error) : resolve()),
      ),
    )
  await invoke('native_test_focus')
  await execute('window.focus();document.querySelector(".ProseMirror").focus();return true')
  await until(async () => await execute('return document.hasFocus()'), 'native window focus')
  await until(
    async () =>
      await execute(
        'return !!document.querySelector(`[data-block-id="${arguments[0]}"]`) && !document.querySelector(".page-actions button")?.disabled',
        [blockId],
      ),
    'page ready for selection',
  )
  await click('.ProseMirror')
  await execute(
    'document.querySelector(".ProseMirror").focus();const e=document.querySelector(`[data-block-id="${arguments[0]}"]`);e.scrollIntoView({block:"center"});const r=document.createRange();r.selectNodeContents(e.firstChild);const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event("selectionchange"));return true',
    [blockId],
  )
  await until(async () => !!(await value('.passage-actions')), 'selection actions')
}
async function reloadPage(title) {
  await clickButton('Getting started with the microscope')
  await clickButton(title)
  await click('#comments-tab')
  await until(async () => !!(await value('.discussion')), 'thread after reload')
}
const local = join(data, 'sources')
await mkdir(local)
const transcript = Array.from(
  { length: 20 },
  (_, i) =>
    `00:${String(i).padStart(2, '0')} Dana: ${i === 12 ? 'The old spreadsheet is retired. Use the booking calendar linked from the equipment page.' : 'Training notes and equipment context for the microscope session.'}`,
).join('\n\n')
await writeFile(join(local, 'meeting.txt'), transcript)
const backupPath = join(data, 'portable.wiki-backup')
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error('Test port is already in use.')
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  console.log('Native app ready.')
  const parent = await create('Session planning', 'Plan the next session.')
  const child = await create(
    'Working notes',
    'Reserve the microscope using the shared spreadsheet before your first session.',
  )
  assert.equal(child.parentId, parent.id)
  await clickButton('Edit page')
  await type('[aria-label="Page title"]', 'Booking notes', true)
  await clickButton('Save page')
  await until(async () => (await value('h1')) === 'Booking notes', 'rename')
  assert.equal((await invoke('read_page', { id: child.id })).revision, 3)
  await click('.settings-button')
  await type('[aria-label="Local source folder"]', local, true)
  await clickButton('Save source settings')
  await until(async () => (await value('.source-settings'))?.includes('saved'), 'source settings')
  await clickButton('Back to page')
  await clickButton('Add artifact')
  await choose('[aria-label="Import source"]', 'local')
  await clickButton('Browse source folder')
  await until(
    async () => (await value('.source-list'))?.includes('meeting.txt'),
    'local transcript',
  )
  await clickButton('meeting.txt')
  await until(async () => !!(await value('.import-preview')), 'preview')
  await type('[aria-label="Import title"]', 'Session transcript', true)
  await choose('[aria-label="Imported content type"]', 'transcript')
  await click('.import-parent input')
  await click('.import-dialog button[type=submit]')
  await until(async () => (await value('h1')) === 'Session transcript', 'import')
  const source = (await invoke('imported_sources'))[0]
  assert.equal(source.parentId, child.id)
  console.log('Created pages, edited a title, and imported a transcript.')
  await click('[aria-label="Search wiki"]')
  await type('.passage-picker[open] [aria-label="Search passages"]', 'retired')
  await until(
    async () =>
      (await value('.passage-picker[open] .passage-results'))?.includes('spreadsheet is retired'),
    'search result',
  )
  assert.match(
    await value('.passage-picker[open] .passage-results'),
    /Session planning \/ Booking notes \/ Session transcript/,
  )
  await clickButton('Open passage')
  await until(async () => (await value('.status'))?.includes('matching passage'), 'open match')
  await screenshot('search-open-passage')
  await clickButton('Booking notes')
  const before = await invoke('read_page', { id: child.id })
  const blockId = before.document.doc.content[0].attrs.blockId
  await selectPassage(blockId)
  await clickButton('Add citation')
  await type('.passage-picker[open] [aria-label="Search passages"]', 'retired')
  await until(
    async () =>
      (await value('.passage-picker[open] .passage-results'))?.includes('spreadsheet is retired'),
    'citation result',
  )
  await clickButton('Inspect excerpt')
  assert.match(await value('.passage-picker[open] .excerpt-preview'), /Revision 1/)
  await clickButton('Cite this excerpt')
  await until(async () => (await value('.status'))?.includes('Citation saved'), 'citation saved')
  const cited = await invoke('read_page', { id: child.id })
  assert.equal(cited.revision, 4)
  assert.equal((await invoke('page_citations', { id: child.id, revision: 4 })).length, 1)
  assert.match(await value('.page-endnotes'), /Session transcript/)
  console.log('Ready for passage comment.')
  await selectPassage(blockId)
  await clickButton('Comment')
  await type('[aria-label="New passage comment"]', 'Please check these booking instructions.')
  await clickButton('Add comment')
  await until(async () => !!(await value('.discussion')), 'discussion')
  const threadId = await execute(
    'return document.querySelector(".discussion[data-active=true]").dataset.threadId',
  )
  await type(
    '[aria-label="Reply to discussion"]',
    '@agent Update this booking instruction using the selected transcript evidence. Keep it one sentence.',
  )
  await click('.reply-composer .request-context summary')
  await until(
    async () => (await value('.reply-composer .request-context'))?.includes('Session transcript'),
    'linked context',
  )
  const context = await invoke('default_evidence', {
    id: child.id,
    query: 'booking spreadsheet retired',
  })
  assert.ok(context.evidence.length <= 6)
  assert.ok(context.evidence.every((e) => e.source.objectId === source.id))
  await clickButton('Choose sources')
  assert.equal(
    await execute(
      'return document.querySelector(".passage-picker[open] .picker-scope input").checked',
    ),
    false,
  )
  await click('.passage-picker[open] .picker-scope input')
  await type('.passage-picker[open] [aria-label="Search passages"]', 'orientation')
  await until(
    async () =>
      (await value('.passage-picker[open] .passage-results'))?.includes('Microscope orientation'),
    'explicit other sources',
  )
  await clickButton('Inspect excerpt')
  await clickButton('Use this excerpt')
  await screenshot('explicit-source-picker')
  await clickButton('Use selected sources')
  assert.match(await value('.reply-composer .request-context'), /Using your selected excerpts/)
  assert.match(await value('.reply-composer .request-context'), /Microscope orientation recording/)
  // Verify explicit inclusion, then retain only the relevant imported source for this request.
  await clickButton('Choose sources')
  await execute(
    'const cards=[...document.querySelectorAll(".passage-picker[open] section article")];const card=cards.find(e=>e.textContent.includes("Microscope orientation recording"));card.querySelector("button").click();return true',
  )
  await clickButton('Use selected sources')
  assert.doesNotMatch(
    await value('.reply-composer .request-context'),
    /Microscope orientation recording/,
  )
  await screenshot('bounded-linked-context')
  console.log('Source selection verified; requesting suggestion.')
  if (livePhase) {
    await clickButton('Reply')
    await until(
      async () =>
        !!(await value('.proposal-card')) ||
        !!(await value('.review-error')) ||
        !!(await value('.connection-prompt')),
      'live imported-source suggestion',
    )
    assert.ok(
      await value('.proposal-card'),
      (await value('.review-error')) || (await value('.connection-prompt')),
    )
  } else {
    await invoke('native_test_candidate', {
      threadId,
      sources: context.evidence.map((e) => e.source),
    })
    await reloadPage('Booking notes')
  }
  assert.ok(await value('.inline-suggestion'), 'suggested wording is inline in the real article')
  assert.equal(await value('.suggestion-citation'), null, 'evidence belongs in the discussion')
  assert.equal(
    await value('[data-citation-id]'),
    '[1]',
    'existing article citation keeps its own number',
  )
  await screenshot('suggestion-before-accept')
  console.log('Suggestion ready.')
  await clickButton('Accept')
  await until(
    async () => (await invoke('read_page', { id: child.id })).revision === 5,
    'accept commits',
  )
  const accepted = await invoke('read_page', { id: child.id })
  const acceptedCitations = await invoke('page_citations', { id: child.id, revision: 5 })
  assert.equal(
    acceptedCitations.length,
    1,
    'rewrite preserves the explicit citation without adding one',
  )
  assert.ok(acceptedCitations.every((e) => e.source.objectId === source.id))
  console.log('Ready for passage comment.')
  await selectPassage(blockId)
  await clickButton('Comment')
  await type(
    '[aria-label="New passage comment"]',
    'Consider another wording, but leave the original if rejected.',
  )
  await clickButton('Add comment')
  await until(
    async () => (await value('#comments-panel'))?.includes('Consider another'),
    'second thread',
  )
  const secondId = await execute(
    'return document.querySelector(".discussion[data-active=true]").dataset.threadId',
  )
  await invoke('native_test_candidate', {
    threadId: secondId,
    sources: context.evidence.map((e) => e.source),
  })
  await reloadPage('Booking notes')
  await execute(
    'document.querySelector(`[data-thread-id="${arguments[0]}"] button.text-link:last-child`)?.click();return true',
    [secondId],
  )
  await clickButton('Reject')
  await until(async () => (await value('#comments-panel'))?.includes('Rejected edit'), 'reject')
  assert.deepEqual((await invoke('read_page', { id: child.id })).document, accepted.document)
  assert.deepEqual(await invoke('page_citations', { id: child.id, revision: 5 }), acceptedCitations)
  await stop()
  await launch()
  await clickButton('Booking notes')
  assert.deepEqual((await invoke('read_page', { id: child.id })).document, accepted.document)
  await clickButton('Page history')
  await until(
    async () => (await value('.revision-list'))?.includes('Revision 5'),
    'history after restart',
  )
  await screenshot('history-after-restart')
  await click('.history-page button')
  await click('.settings-button')
  await type('[aria-label="Backup destination"]', backupPath, true)
  await clickButton('Create backup')
  await until(async () => (await value('.backup-settings'))?.includes('Backup saved:'), 'backup')
  await type('[aria-label="Restore file"]', backupPath)
  await clickButton('Restore as separate copy')
  await until(
    async () => (await value('.backup-settings'))?.includes('Select it below'),
    'restore separate copy',
  )
  const copy = (await invoke('workspace_copies')).find((c) => !c.active)
  assert.ok(copy)
  await clickButton('Use after restarting')
  await until(
    async () => (await value('.backup-settings'))?.includes('Close and reopen'),
    'switch ready',
  )
  await screenshot('restored-copy-ready')
  await stop()
  livePhase = false
  await launch()
  assert.equal((await invoke('workspace_copies')).find((c) => c.active).id, copy.id)
  assert.equal((await invoke('subscription_status')).connected, false)
  assert.equal((await invoke('source_settings')).localFolder, '')
  await clickButton('Booking notes')
  assert.deepEqual((await invoke('read_page', { id: child.id })).document, accepted.document)
  assert.deepEqual(await invoke('page_citations', { id: child.id, revision: 5 }), acceptedCitations)
  const discussions = await invoke('list_discussions', { id: child.id })
  assert.ok(discussions.some((d) => d.proposals.some((p) => p.status === 'accepted')))
  assert.ok(discussions.some((d) => d.proposals.some((p) => p.status === 'rejected')))
  await clickButton('Edit page')
  await type('[aria-label="Page title"]', 'Restored-only name', true)
  await clickButton('Save page')
  await until(async () => (await value('h1')) === 'Restored-only name', 'restored writable')
  await invoke('select_workspace', { id: 'original' })
  await stop()
  await launch()
  assert.equal((await invoke('read_page', { id: child.id })).title, 'Booking notes')
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: true,
        verifiedAt: new Date().toISOString(),
        liveImportedSource: process.env.WIKI_RUN_LIVE_CHECK === '1',
        checks: [
          'create and edit page title with stable ID',
          'import linked long transcript',
          'search breadcrumb/open passage',
          'manual pinned citation/endnotes',
          'linked default/other sources opt-in',
          'review accept/reject',
          'restart/history',
          'backup/restore separate copy',
          'reconnect required',
          'original unchanged',
        ],
      },
      null,
      2,
    ),
  )
  console.log('Native workspace journey passed.')
} catch (e) {
  try {
    await screenshot('failure')
  } catch {}
  await writeFile(join(artifacts, 'failure.log'), String(e) + '\n' + logs)
  throw e
} finally {
  await stop()
  await rm(data, { recursive: true, force: true })
}
