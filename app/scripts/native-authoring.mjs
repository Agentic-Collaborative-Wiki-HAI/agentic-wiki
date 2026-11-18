import assert from 'node:assert/strict'
import { spawn, execFile } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile, rm } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-authoring')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-authoring-'))
const port = 4454,
  base = `http://127.0.0.1:${port}`
let child,
  session,
  logs = '',
  livePhase = false
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
  const end = Date.now() + (description.includes('answer') ? 180000 : 45000)
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
async function focus() {
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
        (e) => (e ? reject(e) : resolve()),
      ),
    )
  await invoke('native_test_focus')
  await until(async () => await execute('return document.hasFocus()'), 'native window focus')
}
async function openChat() {
  await clickButton('Chat')
  await until(
    async () =>
      await execute('return !!document.querySelector("#chat-message")?.getClientRects().length'),
    'chat open',
  )
}
async function selectParagraph(id, from = 0, to = null) {
  await focus()
  await execute(
    `const e=document.querySelector('.document-column [data-block-id="'+arguments[0]+'"]');e.scrollIntoView({block:'center'});const r=document.createRange(); const node=e.firstChild; r.setStart(node,arguments[1]);r.setEnd(node,arguments[2]??node.textContent.length);const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event('selectionchange'));return true`,
    [id, from, to],
  )
  await until(async () => !!(await value('.passage-actions')), 'selection popup')
}
async function addNote(title, body) {
  await clickButton('Add margin note')
  await type('[aria-label="Margin note title"]', title)
  await type('[aria-label="Margin note text"]', body)
  await clickButton('Add note')
  await until(
    async () =>
      !(await execute(
        'return !!document.querySelector("dialog[aria-labelledby=note-title][open]")',
      )),
    'note inserted',
  )
}
const checks = []
try {
  await launch()
  await focus()
  await request(endpoint('/window/rect'), { width: 1440, height: 960 })
  await clickButton('Edit page')
  await type('[aria-label="Page title"]', 'Session guide', true)
  await selectParagraph('booking')
  await addNote('Training', 'Contact the equipment lead for training.')
  assert.equal(await value('[data-note-id]'), '[1]')
  assert.equal((await invoke('read_page', { id: 'getting-started' })).revision, 1)
  await until(
    async () => !!(await invoke('read_draft', { id: 'getting-started' }))?.title,
    'draft with title',
  )
  await screenshot('note-in-draft')
  await clickButton('Cancel')
  assert.equal(await value('[data-note-id]'), null)
  assert.equal(await value('h1'), 'Getting started with the microscope')
  checks.push(
    'cancel discards explicit annotations and title draft without changing the saved page',
  )
  await clickButton('Edit page')
  await type('[aria-label="Page title"]', 'Session guide', true)
  await selectParagraph('booking')
  await addNote('Training', 'Contact the equipment lead for training.')
  await selectParagraph('overview')
  await clickButton('Add citation')
  await type('.passage-picker[open] [aria-label="Search passages"]', 'booking calendar')
  await until(async () => !!(await value('.passage-results article')), 'source search')
  await clickButton('Inspect excerpt')
  await clickButton('Cite this excerpt')
  await until(async () => !!(await value('[data-citation-id]')), 'citation in draft')
  assert.equal((await invoke('read_page', { id: 'getting-started' })).revision, 1)
  await clickButton('Save page')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 2'),
    'saved annotated page',
  )
  assert.equal(await value('h1'), 'Session guide')
  assert.equal(await value('[data-citation-id]'), '[1]')
  assert.equal(await value('[data-note-id]'), '[2]')
  await click('.page-endnotes summary')
  assert.match(await value('.page-endnotes'), /Training/)
  await click('.reference-filter input')
  assert.equal(
    await execute(
      'return [...document.querySelectorAll(".page-endnotes article")].filter(e=>e.getClientRects().length).length',
    ),
    1,
  )
  await screenshot('explicit-references')
  const saved = await invoke('read_page', { id: 'getting-started' })
  const refs = await invoke('document_references', { document: saved.document })
  assert.equal(refs.length, 2)
  checks.push(
    'citations and margin notes can be authored during editing; endnotes filter only explicit references',
  )
  await clickButton('Page history')
  assert.equal(await execute('return !!document.querySelector(".history-page")'), true)
  assert.equal(await execute('return !!document.querySelector("dialog.history-dialog")'), false)
  await click('.revision-list button:last-child')
  await until(
    async () => (await value('.revision-content h2')) === 'Getting started with the microscope',
    'historical title',
  )
  await screenshot('history-page')
  assert.equal(
    await execute(
      'return document.querySelector("footer").getBoundingClientRect().top >= document.querySelector(".history-page").getBoundingClientRect().bottom',
    ),
    true,
    'history occupies the content region above the footer',
  )
  await click('.settings-button')
  assert.equal(
    await execute('return document.querySelector(".history-page") !== null'),
    false,
    'settings replaces history',
  )
  await clickButton('Back to page')
  await until(async () => !!(await value('.history-page')), 'return from settings to history')
  await clickButton('Back to page')
  checks.push('history is a full page and retains historical titles')
  await selectParagraph('booking')
  await clickButton('Comment')
  await type('[aria-label="New passage comment"]', 'Please make this sentence clearer.')
  await clickButton('Add comment')
  await until(async () => !!(await value('.discussion')), 'comment')
  const d = (await invoke('list_discussions', { id: 'getting-started' }))[0]
  assert.equal(
    (
      await invoke('document_references', {
        document: (await invoke('read_page', { id: 'getting-started' })).document,
      })
    ).length,
    2,
  )
  assert.equal(await execute('return !!document.querySelector("[data-thread-anchor]")'), true)
  await invoke('native_test_candidate', { threadId: d.id, sources: [] })
  await clickButton('Microscope orientation recording')
  await clickButton('Session guide')
  await click('#comments-tab')
  await until(async () => !!(await value('.suggested-insertion')), 'source free suggestion')
  await clickButton('Accept')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 3'),
    'source free acceptance',
  )
  const accepted = await invoke('read_page', { id: 'getting-started' })
  assert.deepEqual(
    (await invoke('document_references', { document: accepted.document })).map((e) => e.id),
    refs.map((e) => e.id),
  )
  await selectParagraph('overview', 0, 12)
  await clickButton('Comment')
  await type('[aria-label="New passage comment"]', 'A second passage discussion.')
  await clickButton('Add comment')
  await until(
    async () => (await execute('return document.querySelectorAll(".discussion").length')) === 2,
    'two visible discussion cards',
  )
  assert.equal(await execute('return !!document.querySelector(".thread-list")'), false)
  await screenshot('discussion-cards')
  const activeId = await execute(
    'return document.querySelector(".discussion[data-active=true]").dataset.threadId',
  )
  await click('.discussion[data-active=true] .discussion-heading button')
  await until(
    async () => (await execute('return document.querySelectorAll(".discussion").length')) === 1,
    'resolved thread hidden',
  )
  assert.equal(
    await execute('return !!document.querySelector(`[data-thread-anchor="${arguments[0]}"]`)', [
      activeId,
    ]),
    false,
  )
  checks.push(
    'source-free suggestion acceptance adds no references; comments use highlights and resolving hides the card',
  )
  await click('.settings-button')
  await type(
    '[aria-label="Passage editing instructions"]',
    'Preserve my tone and use short sentences.',
    true,
  )
  await type(
    '[aria-label="Global Chat instructions"]',
    'Be concise. Respond to requests as well as questions.',
    true,
  )
  await clickButton('Save instructions')
  await until(
    async () =>
      (await invoke('read_agent_instructions')).proposal ===
      'Preserve my tone and use short sentences.',
    'custom instructions',
  )
  await screenshot('agent-instructions')
  await clickButton('Back to page')
  await openChat()
  await type('#chat-message', 'A draft message to keep.')
  await clickButton('Hide chat')
  await openChat()
  assert.equal(
    await execute('return document.querySelector("#chat-message").value'),
    'A draft message to keep.',
  )
  assert.equal(
    await execute('return document.querySelector(".chat-title button img").getAttribute("src")'),
    '/assets/83771.svg',
  )
  assert.equal(await value('.chat-composer summary'), 'Context')
  const overlap = await execute(
    'return document.querySelector(".chat-drawer").getBoundingClientRect().left < document.querySelector(".document-column").getBoundingClientRect().right',
  )
  assert.equal(overlap, false, 'Chat leaves the article visible')
  await screenshot('chat-context')
  await clickButton('Hide chat')
  checks.push(
    'chat hides and preserves its message; uses the shared Chat icon and quiet Context disclosure',
  )
  await stop()
  await launch()
  await focus()
  assert.equal(await value('h1'), 'Session guide')
  assert.equal(
    (await invoke('read_agent_instructions')).proposal,
    'Preserve my tone and use short sentences.',
  )
  assert.equal(
    (
      await invoke('document_references', {
        document: (await invoke('read_page', { id: 'getting-started' })).document,
      })
    ).length,
    2,
  )
  checks.push('annotations, edited title and custom instructions survive restart')
  for (const width of [1440, 980, 650, 400]) {
    await request(endpoint('/window/rect'), { width, height: 900 })
    await delay(250)
    const geometry = await execute(
      'const r=document.querySelector(".document-column").getBoundingClientRect(); return {fits:document.documentElement.scrollWidth<=innerWidth,bodyWidth:r.width,breadcrumb:getComputedStyle(document.querySelector(".breadcrumbs")).display}',
    )
    assert.equal(geometry.fits, true)
    assert.ok(geometry.bodyWidth > 250)
    assert.notEqual(geometry.breadcrumb, 'none')
    await screenshot(`layout-${width}`)
  }
  checks.push('responsive body and visible breadcrumbs at wide and narrow widths')
  await writeFile(join(artifacts, 'result.json'), JSON.stringify({ passed: true, checks }, null, 2))
  console.log('PASS native authoring corrections: ' + checks.join('; '))
} catch (e) {
  try {
    await screenshot('failure')
  } catch {}
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify({ passed: false, error: String(e), checks }, null, 2),
  )
  throw e
} finally {
  await stop()
  await writeFile(join(artifacts, 'native.log'), logs)
}
