import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-smoke')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-native-'))
const port = 4446,
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
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error(`Test port ${port} is already in use.`)
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  await execute('window.__TAURI_INTERNALS__.invoke("native_test_focus");return true')
  await until(async () => await execute('return document.hasFocus()'), 'native window focus')
  assert.equal(await value('h1'), 'Getting started with the microscope')
  await screenshot('getting-started')
  const original = await value('[data-block-id=booking]')
  await click('[aria-label="Toggle wiki navigation"]')
  assert.equal(await value('#wiki-tree'), null)
  await click('[aria-label="Toggle wiki navigation"]')
  assert.ok(await value('#wiki-tree'))
  await click('[aria-label="Toggle page contents"]')
  assert.equal(await value('#heading-list'), null)
  await click('[aria-label="Toggle page contents"]')
  assert.ok(await value('#heading-list'))
  await click('#comments-tab')
  assert.equal(await value('[data-block-id=booking]'), original)
  await click('#references-tab')
  assert.equal(await value('[data-block-id=booking]'), original)
  await clickButton('Chat')
  assert.ok(await value('.chat-drawer'))
  assert.equal(await value('[data-block-id=booking]'), original)
  await clickButton('Close chat')
  assert.equal(await value('.chat-drawer'), null)
  await clickButton('Microscope orientation recording')
  assert.equal(await value('h1'), 'Microscope orientation recording')
  await clickButton('Getting started with the microscope')
  assert.equal(await value('[data-block-id=booking]'), original)
  await execute(
    'const r=document.createRange();r.selectNodeContents(document.querySelector("[data-block-id=booking]"));const s=getSelection();s.removeAllRanges();s.addRange(r);document.dispatchEvent(new Event("selectionchange"));return true',
  )
  await click('#comments-tab')
  await until(
    async () => (await value('blockquote')) === original,
    'selected passage appears in the comments rail',
  )
  await click('#references-tab')
  console.log('PASS native navigation, independent ToCs, tabs and clean chat shell')
  await clickButton('Edit page')
  await replaceBooking('Use the booking calendar. 🔬')
  await until(
    async () => (await value('.draft-status')) === 'Draft saved on this device',
    'draft persisted',
  )
  assert.equal(await value('[data-block-id=booking]'), 'Use the booking calendar. 🔬')
  await clickButton('Cancel')
  assert.equal(await value('[data-block-id=booking]'), original)
  console.log('PASS cancel preserves the saved page')
  await clickButton('Edit page')
  await replaceBooking('Use the booking calendar. 🔬')
  await until(
    async () => (await value('.draft-status')) === 'Draft saved on this device',
    'recoverable draft',
  )
  await stop()
  await launch()
  assert.equal(await value('[data-block-id=booking]'), original)
  assert.match(await value('.status'), /unfinished draft/)
  await clickButton('Edit page')
  assert.equal(await value('[data-block-id=booking]'), 'Use the booking calendar. 🔬')
  await clickButton('Save page')
  await until(async () => (await value('.status')) === 'Saved revision 2.', 'save')
  await clickButton('Page history')
  assert.match(await value('.history-dialog'), /Revision 2/)
  await clickButton('Back to page')
  await stop()
  await launch()
  assert.equal(await value('[data-block-id=booking]'), 'Use the booking calendar. 🔬')
  assert.match(await value('.metadata'), /Revision 2/)
  console.log('PASS draft recovery, save, history and persistence across process restarts')
  await request(endpoint('/window/rect'), { width: 980, height: 760 })
  const tabsTop = await execute(
    'return document.querySelector(".tabs").getBoundingClientRect().top',
  )
  await execute('document.querySelector(".workspace").scrollTop=9999;return true')
  await delay(100)
  assert.equal(
    await execute('return document.querySelector(".tabs").getBoundingClientRect().top'),
    tabsTop,
    'rail tabs stay visible while scrolling',
  )
  await screenshot('compact-window')
  assert.equal(
    await execute('return document.documentElement.scrollWidth<=innerWidth'),
    true,
    'no horizontal document overflow',
  )
  console.log('PASS compact desktop layout has no document overflow')
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: true,
        native: true,
        database: data,
        checks: [
          'navigation',
          'ToCs',
          'tabs',
          'chat shell',
          'cancel',
          'draft recovery',
          'save',
          'history',
          'process restart',
          'compact window',
        ],
      },
      null,
      2,
    ),
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
