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
async function selectBooking() {
  await execute(
    'const e=document.querySelector("[data-block-id=booking]"); const r=document.createRange();r.selectNodeContents(e);const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event("selectionchange"));return true',
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
  await clickButton('Getting started')
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
  const before = await invoke('read_page', { id: 'getting-started' })
  const original = await value('[data-block-id=booking]')
  const comment = 'This booking instruction may be out of date. Please check the recording.'
  const threadId = await createComment(comment)
  assert.deepEqual((await invoke('read_page', { id: 'getting-started' })).document, before.document)
  await clickButton('Ask agent')
  await until(async () => !!(await value('.connection-card')), 'honest disconnected sign-in')
  assert.match(await value('.connection-card'), /ChatGPT/)
  assert.equal(
    await execute(
      'return [...document.querySelectorAll("button")].find(e=>e.textContent.trim()==="Request suggestion").disabled',
    ),
    true,
  )
  await type(
    '[aria-label="Agent request"]',
    'Compare this passage with the booking-calendar guidance.',
  )
  await click('.source-choice input')
  await screenshot('explicit-context-and-sign-in')
  await clickButton('Close request')
  await invoke('native_test_candidate', { threadId })
  await stop()
  await launch()
  await click('#comments-tab')
  await until(async () => !!(await value('.proposal-card')), 'test candidate visible')
  assert.match(await value('.proposal-card'), /Test fixture · no model call/)
  assert.equal(await value('[data-block-id=booking]'), original)
  await clickButton('Ask for revision')
  assert.equal(
    await execute('return document.activeElement?.getAttribute("aria-label")'),
    'Agent request',
  )
  assert.equal(
    await execute(
      'const r=document.activeElement.getBoundingClientRect();return r.top>=68 && r.bottom<=innerHeight',
    ),
    true,
    'revision request is brought into view',
  )
  assert.match(
    await value('.previous-proposal-context'),
    /Check the equipment page for the current booking calendar/,
  )
  await type('[aria-label="Agent request"]', 'Make that shorter.')
  await screenshot('revision-request-context')
  await clickButton('Close request')
  await click('.proposal-card .text-link')
  assert.match(await value('.evidence-dialog[open]'), /booking calendar/)
  assert.match(await value('.evidence-dialog[open]'), /Revision 1/)
  await screenshot('evidence-preview')
  await clickButton('Back to discussion')
  await type(
    '[aria-label="Reply to discussion"]',
    'The source supports a calendar link; keep the instructions brief.',
  )
  await click('.reply-composer button')
  await clickButton('Resolve thread')
  assert.match(await value('.discussion-heading'), /Resolved/)
  assert.match(await value('.proposal-card'), /Suggested edit/)
  await clickButton('Reopen thread')
  await clickButton('Edit suggestion')
  const wording =
    'Check the equipment page for current booking instructions before your first session.'
  await type('[aria-label="Edit proposed wording"]', wording, true)
  await type('[aria-label="Reason for proposal revision"]', 'Keep the wording concise.')
  await clickButton('Save proposal revision')
  await until(
    async () => (await value('.proposal-diff ins')) === wording,
    'human candidate revision',
  )
  assert.equal(await value('[data-block-id=booking]'), original)
  const tabsTop = await execute(
    'return document.querySelector(".tabs").getBoundingClientRect().top',
  )
  await execute('document.querySelector(".context-rail").scrollTop=99999;return true')
  await delay(100)
  assert.equal(
    await execute('return document.querySelector(".tabs").getBoundingClientRect().top'),
    tabsTop,
    'tabs remain visible with a long discussion',
  )
  await screenshot('review-before-accept')
  await clickButton('Accept edit')
  await until(async () => (await value('.metadata'))?.includes('Revision 2'), 'accepted revision')
  assert.equal(await value('[data-block-id=booking]'), wording + '[1]')
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
  assert.match(await value('.discussion'), /human revision/)
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
      'return [...document.querySelectorAll("button")].find(e=>e.textContent.trim()==="Accept edit").disabled',
    ),
    true,
  )
  await clickButton('Refresh suggestion')
  assert.match(await value('.agent-request'), /Select the current passage/)
  await selectBooking()
  await clickButton('Use selected passage')
  await screenshot('stale-request-requires-reselection')
  await clickButton('Close request')
  await invoke('native_test_candidate', { threadId: staleThread })
  await reload()
  await click('.thread-list button:last-child')
  assert.equal(
    await execute(
      'return [...document.querySelectorAll("button")].find(e=>e.textContent.trim()==="Accept edit").disabled',
    ),
    false,
  )
  assert.equal(await value('[data-block-id=booking]'), wording + '[1]')
  await clickButton('Accept edit')
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
          'passage comment',
          'explicit source context',
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
  throw error
} finally {
  await stop()
  await writeFile(join(artifacts, 'native.log'), logs)
}
