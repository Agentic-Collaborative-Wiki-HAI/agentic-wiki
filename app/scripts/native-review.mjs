import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-review')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-native-'))
const port = 4447,
  base = `http://127.0.0.1:${port}`
let child,
  session,
  logs = ''
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
  const end = Date.now() + 45000
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
  const e = await execute(
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
async function replaceBooking(text) {
  await execute(
    'const e=document.querySelector("[data-block-id=booking]"); const r=document.createRange();r.selectNodeContents(e);const s=getSelection();s.removeAllRanges();s.addRange(r);return true',
  )
  const target = await element('.ProseMirror')
  await request(endpoint(`/element/${target}/value`), { text })
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
async function checkSelectionPopup() {
  await invoke('native_test_focus')
  await request(endpoint('/window/rect'), { width: 1440, height: 700 })
  const before = await execute(
    'return document.querySelector("[data-block-id=booking]").getBoundingClientRect().top',
  )
  await execute(
    'const e=document.querySelector("[data-block-id=booking]");const r=document.createRange();r.setStart(e.firstChild,8);r.setEnd(e.firstChild,28);const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event("selectionchange"));return true',
  )
  await until(async () => !!(await value('.passage-actions')), 'anchored popup')
  const geometry = () =>
    execute(
      'const p=document.querySelector(".passage-actions").getBoundingClientRect();const s=getSelection().getRangeAt(0).getBoundingClientRect();return {popup:{top:p.top,bottom:p.bottom,left:p.left,right:p.right,width:p.width},selection:{top:s.top,bottom:s.bottom,left:s.left,right:s.right},fixed:getComputedStyle(document.querySelector(".passage-actions")).position,articleTop:document.querySelector("[data-block-id=booking]").getBoundingClientRect().top}',
    )
  await until(async () => {
    const { popup, selection } = await geometry()
    return (
      Math.abs(selection.top - popup.bottom - 8) < 2 ||
      Math.abs(popup.top - selection.bottom - 8) < 2
    )
  }, 'popup measured beside the highlight')
  const initial = await geometry()
  assert.equal(initial.fixed, 'fixed')
  assert.equal(initial.articleTop, before, 'opening the popup does not move the article')
  assert.ok(initial.popup.width < 260, 'compact action instead of a full-width bar')
  assert.ok(
    Math.abs(initial.selection.top - initial.popup.bottom - 8) < 2 ||
      Math.abs(initial.popup.top - initial.selection.bottom - 8) < 2,
    'popup beside the actual highlight',
  )
  assert.ok(
    initial.popup.right >= initial.selection.left && initial.popup.left <= initial.selection.right,
  )
  await screenshot('selection-popup')
  const scrolled = await execute(
    'const w=document.querySelector(".workspace");w.scrollTop=80;return w.scrollTop',
  )
  assert.ok(scrolled > 0, 'actual article scroll')
  await delay(100)
  const after = await geometry()
  assert.ok(Math.abs(initial.selection.top - after.selection.top - scrolled) < 2)
  assert.ok(
    Math.abs(after.selection.top - after.popup.bottom - 8) < 2 ||
      Math.abs(after.popup.top - after.selection.bottom - 8) < 2,
    'popup follows the highlight while scrolling',
  )
  await execute(
    'document.dispatchEvent(new KeyboardEvent("keydown", {key:"Escape",bubbles:true}));return true',
  )
  assert.equal(await execute('return !!document.querySelector(".passage-actions")'), false)
  await execute(
    'getSelection().collapseToEnd();document.querySelector(".workspace").scrollTop=0;return true',
  )
  await request(endpoint('/window/rect'), { width: 1440, height: 960 })
}
async function selectBooking() {
  await invoke('native_test_focus')
  await execute(
    'document.querySelector(".ProseMirror").focus(); const e=document.querySelector("[data-block-id=booking]"); const r=document.createRange();r.selectNodeContents(e);const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event("selectionchange"));return true',
  )
  await until(async () => !!(await value('.passage-actions')), 'selection action')
  await clickButton('Comment')
}
async function createComment(text) {
  await selectBooking()
  await type('[aria-label="New passage comment"]', text)
  await clickButton('Add comment')
  await until(async () => (await value('.discussion'))?.includes(text), 'comment persisted')
  return execute('return document.querySelector(".discussion").dataset.threadId')
}
async function reload() {
  await clickButton('Microscope orientation recording')
  await clickButton('Getting started with the microscope')
  await click('#comments-tab')
  await until(async () => !!(await value('.discussion')), 'reloaded discussion')
}
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error(`Test port ${port} is already in use.`)
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  await checkSelectionPopup()
  const before = await invoke('read_page', { id: 'getting-started' })
  const original = await value('[data-block-id=booking]')
  const comment = 'This booking instruction may be out of date. Please check the recording.'
  const threadId = await createComment(comment)
  assert.deepEqual((await invoke('read_page', { id: 'getting-started' })).document, before.document)
  const requestText =
    '@agent Compare this passage with the orientation transcript and suggest a revision.'
  await type('[aria-label="Reply to discussion"]', requestText)
  await click('.request-context summary')
  assert.match(await value('.request-context'), /Microscope orientation recording/)
  assert.doesNotMatch(await value('.request-context'), /Sources to include/)
  await clickButton('Reply')
  await until(
    async () => !!(await value('.connection-prompt')),
    'settings link for disconnected request',
  )
  assert.equal((await invoke('list_discussions', { id: 'getting-started' }))[0].proposals.length, 0)
  await click('.connection-prompt button')
  await until(async () => !!(await value('.settings-page')), 'connection settings')
  assert.match(await value('.settings-page'), /Continue with ChatGPT/)
  await screenshot('connection-settings')
  await clickButton('Back to page')
  assert.equal(
    await execute('return document.querySelector(arguments[0]).value', [
      '[aria-label="Reply to discussion"]',
    ]),
    requestText,
    'settings preserves request draft',
  )
  assert.equal(await execute('return !!document.querySelector(".agent-request")'), false)
  await invoke('native_test_candidate', { threadId })
  await stop()
  await launch()
  await click('#comments-tab')
  await until(async () => !!(await value('.proposal-card')), 'test candidate visible')
  assert.match(await value('.proposal-card'), /Test fixture · no model call/)
  assert.equal(await value('.suggested-deletion'), original)
  assert.match(await value('.ProseMirror .suggested-insertion'), /current booking calendar/)
  assert.deepEqual((await invoke('read_page', { id: 'getting-started' })).document, before.document)
  await screenshot('inline-suggestion-and-thread')
  await type('[aria-label="Reply to discussion"]', '@agent Make that shorter.')
  await click('.request-context summary')
  assert.match(await value('.previous-proposal-context'), /current booking calendar/)
  await screenshot('reply-with-prior-context')
  await click('.proposal-card .text-link')
  assert.match(await value('.evidence-dialog[open]'), /booking calendar/)
  assert.match(await value('.evidence-dialog[open]'), /Revision 1/)
  await screenshot('evidence-preview')
  await clickButton('Back to discussion')
  await type(
    '[aria-label="Reply to discussion"]',
    'The source supports a calendar link; keep the instructions brief.',
    true,
  )
  await clickButton('Reply')
  await clickButton('Resolve thread')
  assert.match(await value('.discussion-heading'), /Resolved/)
  assert.match(await value('.proposal-card'), /Suggested edit/)
  await clickButton('Reopen thread')
  assert.equal(await execute('return !!document.querySelector(".reply-composer")'), true)
  await clickButton('Edit suggested text')
  const wording =
    'Check the equipment page for current booking instructions before your first session.'
  await type('[aria-label="Edit proposed wording"]', 'A version to discard.', true)
  await clickButton('Cancel edits')
  assert.match(await value('.suggested-insertion'), /current booking calendar/)
  assert.deepEqual((await invoke('read_page', { id: 'getting-started' })).document, before.document)
  await clickButton('Edit suggested text')
  await type('[aria-label="Edit proposed wording"]', wording, true)
  assert.equal(
    await execute(
      'return document.querySelector(".ProseMirror").contains(document.querySelector(".inline-wording"))',
    ),
    true,
  )
  const tabsTop = await execute(
    'return document.querySelector(".tabs").getBoundingClientRect().top',
  )
  await execute('document.querySelector(".context-rail").scrollTop=99999;return true')
  await delay(100)
  assert.equal(
    await execute('return document.querySelector(".tabs").getBoundingClientRect().top'),
    tabsTop,
  )
  await screenshot('inline-human-edit')
  await clickButton('Accept edited')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 2'),
    'accepted edited revision',
  )
  assert.equal(await value('[data-block-id=booking]'), wording + '[1]')
  assert.equal(await execute('return !!document.querySelector(".inline-suggestion")'), false)
  await click('[data-citation-id]')
  assert.match(await value('.evidence-dialog[open]'), /booking calendar/)
  await clickButton('Back to discussion')
  await clickButton('Edit page')
  await replaceBooking('A draft that must be discarded.')
  await clickButton('Cancel')
  assert.equal(await value('[data-block-id=booking]'), wording + '[1]')
  await clickButton('Page history')
  await until(
    async () => !!(await value('.history-dialog[open] .revision-content')),
    'history content',
  )
  assert.match(await value('.revision-content'), /current booking instructions/)
  assert.match(await value('.revision-discussions'), /accepted/)
  await click('.revision-list button:last-child')
  await until(
    async () => (await value('.revision-content'))?.includes(original),
    'original revision read',
  )
  await screenshot('original-revision-with-discussion')
  await clickButton('Back to page')
  await screenshot('accepted-with-discussion')
  await stop()
  await launch()
  assert.equal(await value('[data-block-id=booking]'), wording + '[1]')
  await click('#comments-tab')
  await until(
    async () => (await value('.discussion'))?.includes(comment),
    'persistent conversation',
  )
  assert.match(await value('.discussion'), /You · Edited suggestion/)
  assert.equal(
    (await invoke('list_discussions', { id: 'getting-started' }))[0].proposals.at(-1).origin,
    'human revision',
  )
  const accepted = await invoke('read_page', { id: 'getting-started' })
  const rejectedThread = await createComment(
    'Check this again, but do not publish a change until I accept.',
  )
  await invoke('native_test_candidate', { threadId: rejectedThread })
  await reload()
  await click('.thread-list button:last-child')
  await clickButton('Reject')
  await until(async () => (await value('.proposal-card'))?.includes('Rejected edit'), 'rejection')
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    accepted.document,
  )
  assert.equal((await invoke('page_history', { id: 'getting-started' })).length, 2)
  await stop()
  await launch()
  await click('#comments-tab')
  await until(async () => !!(await value('.thread-list')), 'reopened threads')
  await click('.thread-list button:last-child')
  assert.match(await value('.proposal-card'), /Rejected edit/)
  await selectBooking()
  assert.ok(await value('.comment-composer'), 'same passage can be selected again after rejection')
  await click('.comment-composer button[type=button]')
  await request(endpoint('/window/rect'), { width: 980, height: 760 })
  await screenshot('compact-review')
  assert.equal(await execute('return document.documentElement.scrollWidth<=innerWidth'), true)
  await request(endpoint('/window/rect'), { width: 1440, height: 960 })
  const staleThread = await createComment('Please check the booking instruction once more.')
  await invoke('native_test_candidate', { threadId: staleThread })
  await reload()
  await click('.thread-list button:last-child')
  await clickButton('Edit page')
  await execute(
    'const e=document.querySelector("[data-block-id=overview]");const r=document.createRange();r.selectNodeContents(e);const s=getSelection();s.removeAllRanges();s.addRange(r);return true',
  )
  await request(endpoint(`/element/${await element('.ProseMirror')}/value`), {
    text: 'Plan your first microscope session using this guide.',
  })
  await clickButton('Save page')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 3'),
    'intervening page revision',
  )
  await until(
    async () => (await value('.review-warning'))?.includes('page changed'),
    'stale warning',
  )
  assert.equal(
    await execute(
      'return [...document.querySelectorAll("button")].find(e=>e.textContent.trim()==="Accept").disabled',
    ),
    true,
  )
  assert.equal(
    await execute('return !!document.querySelector(".inline-suggestion")'),
    false,
    'stale changes are not projected onto the article',
  )
  await type('[aria-label="Reply to discussion"]', '@agent Refresh this suggestion.')
  await click('.request-context summary')
  assert.match(await value('.request-context'), /Select the current passage/)
  await selectBooking()
  assert.ok((await value('.request-context')).includes(wording))
  await screenshot('stale-request-requires-reselection')
  await click('.comment-composer button[type=button]')
  await invoke('native_test_candidate', { threadId: staleThread })
  await reload()
  await click('.thread-list button:last-child')
  assert.equal(
    await execute(
      'return [...document.querySelectorAll("button")].find(e=>e.textContent.trim()==="Accept").disabled',
    ),
    false,
  )
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    (await invoke('read_revision', { id: 'getting-started', revision: 3 })).document,
  )
  await clickButton('Accept')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 4'),
    'explicit refreshed acceptance',
  )
  assert.match(await value('[data-block-id=booking]'), /current booking calendar/)
  assert.equal((await invoke('page_history', { id: 'getting-started' })).length, 4)
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: true,
        native: true,
        database: data,
        model: 'Test-only candidate injection; no live model inference',
        checks: [
          'selection-anchored floating action without article reflow; scroll and Escape',
          'passage comment',
          'linked evidence context disclosed in the thread',
          'inline preview and cancelled human edit do not alter canonical JSON',
          'Settings round trip preserves reply draft',
          'ordinary replies do not invoke the agent',
          'sign-in required',
          'evidence and return',
          'reply',
          'resolve separate from acceptance',
          'human candidate revision',
          'pending proposal survives process restart',
          'revision request discloses previous draft',
          'stale acceptance blocked and explicit refreshed review',
          'atomic acceptance',
          'inline citation',
          'cancel after acceptance',
          'original and accepted revision views',
          'restart persistence',
          'rejection preserves wording and citations',
          'repeat passage selection after rejection and cancelled comment',
          'compact layout',
        ],
      },
      null,
      2,
    ),
  )
  console.log(
    'PASS native review journey, evidence, human revision, accept/reject, history and restart. Model output was an explicitly labeled test fixture.',
  )
} catch (error) {
  try {
    if (session) await screenshot('failure')
  } catch {}
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify({ passed: false, error: String(error) }, null, 2),
  )
  throw error
} finally {
  await stop()
  await writeFile(join(artifacts, 'native.log'), logs)
}
