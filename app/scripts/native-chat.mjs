import assert from 'node:assert/strict'
import { spawn, execFile } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile, rm } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-chat')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-chat-'))
const port = 4452,
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
async function choose(selector, value) {
  await execute(
    'const e=document.querySelector(arguments[0]);e.value=arguments[1];e.dispatchEvent(new Event("change",{bubbles:true}));return true',
    [selector, value],
  )
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
async function setSources(values) {
  await click('.chat-composer .chat-context summary')
  await clickButton('Choose sources')
  await until(
    async () => await execute('return !!document.querySelector(".passage-picker[open]")'),
    'source picker',
  )
  if (values === 'empty') {
    while (
      await execute(
        'return [...document.querySelectorAll(".passage-picker button")].some(e=>e.textContent.trim()==="Remove excerpt")',
      )
    )
      await clickButton('Remove excerpt')
  }
  await clickButton('Use selected sources')
}
async function fixture(question, missing = false) {
  const c = await invoke('chat_context', { id: 'getting-started', query: question })
  return invoke('native_test_chat', {
    request: {
      id: crypto.randomUUID(),
      pageId: 'getting-started',
      question,
      sources: missing
        ? []
        : c.evidence
            .filter((e) => e.source.objectId === 'orientation-recording')
            .map((e) => e.source),
      model: 'fixture',
    },
    missing,
  })
}
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error('Test port is already in use.')
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  await focus()
  await screenshot('01-page')
  const original = await invoke('read_page', { id: 'getting-started' })
  await openChat()
  assert.equal((await invoke('chat_turns')).length, 0, 'opening Chat does not send a request')
  assert.equal(await execute('return document.querySelector("#chat-message").disabled'), false)
  await screenshot('02-empty-chat')
  const question =
    'Which source describes how to book a microscope session? Explain conflicting guidance and cite the supporting passages.'
  await type('#chat-message', question)
  await setSources('default')
  const chosen = await execute(
    'return document.querySelector(".chat-composer .chat-context").textContent',
  )
  assert.match(chosen, /selected excerpts/i)
  assert.match(chosen, /Microscope orientation/)
  let answer
  if (livePhase) {
    await clickButton('Send')
    await until(async () => {
      const t = await invoke('chat_turns')
      if (t.at(-1)?.status === 'failed') throw Error(t.at(-1).error)
      return t.at(-1)?.status === 'completed'
    }, 'live sourced answer')
    answer = (await invoke('chat_turns')).at(-1)
    assert.ok(answer.answer.paragraphs.length, 'real sourced answer')
  } else {
    answer = await fixture(question)
    await clickButton('Hide chat')
    await openChat()
  }
  await until(async () => !!(await value('.chat-answer')), 'rendered answer')
  assert.equal((await invoke('read_page', { id: 'getting-started' })).revision, original.revision)
  assert.ok(
    answer.answer.paragraphs.every((p) =>
      p.citedSourceIds.every((id) => answer.evidence.some((e) => e.id === id)),
    ),
  )
  await screenshot('03-sourced-answer')
  await click('.chat-citation')
  await until(
    async () => await execute('return !!document.querySelector(".source-preview[open]")'),
    'evidence opened',
  )
  const quote = await value('.source-preview[open] blockquote')
  assert.ok(
    answer.evidence.some((e) => e.quote === quote),
    'exact pinned source quote',
  )
  await screenshot('04-answer-evidence')
  await clickButton('Back to chat')
  await clickButton('Create draft page')
  await until(async () => !!(await value('.answer-draft-page h1')), 'draft editor')
  let draft = (await invoke('answer_drafts'))[0]
  assert.ok(
    !(await invoke('wiki_records')).some((r) => r.id === draft.id),
    'not published by drafting',
  )
  await type('.draft-fields input', 'Session planning guide', true)
  await type('.answer-draft-page .ProseMirror', ' Confirm availability with the equipment lead.')
  await until(
    async () => (await value('.answer-draft-page .draft-status')) === 'Draft saved on this device',
    'draft autosave',
  )
  draft = (await invoke('answer_drafts'))[0]
  assert.match(JSON.stringify(draft.document), /Confirm availability/)
  assert.ok(draft.evidence.length, 'citations survived editing')
  await screenshot('05-editable-draft')
  await clickButton('Keep draft and close')
  await until(async () => !!(await value('.chat-answer')), 'back to conversation')
  await stop()
  await launch()
  await focus()
  await openChat()
  assert.ok(!(await invoke('wiki_records')).some((r) => r.id === draft.id))
  await clickButton('Continue draft')
  assert.equal(
    await execute('return document.querySelector(".draft-fields input").value'),
    'Session planning guide',
  )
  assert.match(await value('.answer-draft-page .ProseMirror'), /Confirm availability/)
  await click('.publish-answer')
  await until(
    async () => (await value('.title-row h1')) === 'Session planning guide',
    'explicit publication',
  )
  const published = await invoke('read_page', { id: draft.id })
  assert.equal(published.revision, 1)
  assert.deepEqual(published.document, draft.document)
  const citations = await invoke('page_citations', { id: draft.id, revision: 1 })
  assert.equal(citations.length, draft.evidence.length)
  assert.match(await value('.metadata'), /Created from an AI answer/)
  await screenshot('06-published-page')
  await clickButton('View originating Chat answer')
  await until(async () => !!(await value('.chat-answer')), 'originating answer')
  await clickButton('Hide chat')
  await stop()
  await launch()
  await focus()
  await clickButton('Session planning guide')
  await until(
    async () => (await value('.title-row h1')) === 'Session planning guide',
    'published page after restart',
  )
  assert.deepEqual((await invoke('read_page', { id: draft.id })).document, published.document)
  await clickButton('Page history')
  await screenshot('07-draft-publication-history')
  // Leave native history using its own dismissal, then check an honest no-evidence response.
  await clickButton('Back to page')
  await clickButton('Getting started with the microscope')
  await openChat()
  const unknown = 'What booking times are available tomorrow?'
  if (livePhase) {
    await type('#chat-message', unknown, true)
    await setSources('empty')
    await clickButton('Send')
    await until(async () => {
      const t = (await invoke('chat_turns')).at(-1)
      if (t?.status === 'failed') throw Error(t.error)
      return t?.request.question === unknown && t.status === 'completed'
    }, 'missing-evidence answer')
  } else {
    await fixture(unknown, true)
    await clickButton('Hide chat')
    await openChat()
  }
  const missing = (await invoke('chat_turns')).at(-1)
  assert.equal(missing.answer.paragraphs.length, 0)
  assert.ok(missing.answer.missingEvidence.length)
  await screenshot('08-missing-evidence')
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    original.document,
  )
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      {
        passed: true,
        verifiedAt: new Date().toISOString(),
        live: livePhase,
        checks: [
          'quiet opening',
          'explicit source context',
          'sourced answer',
          'exact source preview',
          'unpublished editable draft',
          'draft restart',
          'explicit publication',
          'citations and AI origin',
          'published restart/history',
          'missing-evidence answer',
          'original page unchanged',
        ],
      },
      null,
      2,
    ),
  )
  console.log('Native Chat to draft journey passed.')
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
