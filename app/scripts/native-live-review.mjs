import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { once } from 'node:events'
import { mkdir, mkdtemp, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

if (process.env.WIKI_RUN_LIVE_CHECK !== '1') {
  throw Error('Set WIKI_RUN_LIVE_CHECK=1 explicitly; this check consumes ChatGPT plan usage.')
}

const root = fileURLToPath(new URL('../', import.meta.url))
const artifacts = join(root, '.artifacts/native-live-review')
const data = await mkdtemp(join(tmpdir(), 'agentic-wiki-native-'))
const port = 4448,
  base = `http://127.0.0.1:${port}`
let child, session
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
async function until(fn, description, waitMs = 45000) {
  const end = Date.now() + waitMs
  while (Date.now() < end) {
    if (await fn()) return
    await delay(200)
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
async function launch() {
  child = spawn(join(root, 'src-tauri/target/debug/agentic-wiki'), [], {
    cwd: root,
    stdio: 'ignore',
    env: {
      ...process.env,
      WIKI_E2E_DATA_DIR: data,
      TAURI_WEBDRIVER_PORT: String(port),
      WIKI_E2E_LIVE_SUBSCRIPTION: '1',
    },
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
    if (child.exitCode === null && child.signalCode === null) {
      const ended = once(child, 'exit')
      child.kill('SIGTERM')
      await ended
    }
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
async function chooseRecording() {
  const index = await execute(
    'return [...document.querySelectorAll(".source-choice")].findIndex(e=>e.textContent.includes("Microscope orientation recording") && e.textContent.includes("booking calendar"))',
  )
  assert.ok(index >= 0, 'booking evidence is available')
  await click(`.source-choice:nth-of-type(${index + 1}) input`)
}
async function readyRequest(text, chooseSource = true) {
  await clickButton('Ask agent')
  await until(async () => {
    const failure = await value('.review-error')
    if (failure) throw Error(failure)
    return await execute('return !!document.querySelector(arguments[0])?.value', [
      '[aria-label="Suggestion model"]',
    ])
  }, 'live model catalog')
  await type('[aria-label="Agent request"]', text)
  if (chooseSource) await chooseRecording()
}
async function discussion(id) {
  return (await invoke('list_discussions', { id: 'getting-started' })).find((d) => d.id === id)
}
async function pendingProposal(id, previousId = null) {
  let proposal
  await until(
    async () => {
      const failure = await value('.review-error')
      if (failure) throw Error(failure)
      proposal = (await discussion(id)).proposals.find(
        (p) => p.status === 'pending' && p.id !== previousId,
      )
      return !!proposal
    },
    'complete live model suggestion',
    195000,
  )
  assert.match(proposal.origin, /^ChatGPT · /)
  assert.ok(proposal.candidate.citedSourceIds.length)
  for (const e of proposal.evidence) {
    assert.equal(e.source.objectId, 'orientation-recording')
    assert.equal(e.source.revision, 1)
  }
  return proposal
}
const checks = []
const outputs = []
try {
  try {
    await fetch(base + '/status', { signal: AbortSignal.timeout(300) })
    throw Error(`Test port ${port} is already in use.`)
  } catch (e) {
    if (e.message.startsWith('Test port')) throw e
  }
  await launch()
  assert.equal((await invoke('subscription_status')).connected, true, 'app-owned sign-in')
  const original = await invoke('read_page', { id: 'getting-started' })
  const threadId = await createComment(
    'Please check this booking instruction against the orientation recording.',
  )
  await readyRequest(
    'Replace the outdated spreadsheet instruction with one concise sentence supported by the selected recording excerpt.',
  )
  const model = await execute('return document.querySelector(arguments[0]).value', [
    '[aria-label="Suggestion model"]',
  ])
  console.log('Connected model:', model)
  checks.push('app-owned sign-in and live model catalog')

  await clickButton('Request suggestion')
  await until(
    async () =>
      (await discussion(threadId)).messages.some((m) => m.body.startsWith('Asked agent:')),
    'live request started',
  )
  await delay(750)
  await clickButton('Cancel request')
  await until(
    async () => (await value('.review-error'))?.includes('Request cancelled.'),
    'live request cancelled',
  )
  assert.equal((await discussion(threadId)).proposals.length, 0)
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    original.document,
  )
  checks.push('live request cancellation saves no proposal and leaves the page unchanged')
  console.log('PASS live request cancellation')

  await clickButton('Request suggestion')
  const first = await pendingProposal(threadId)
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    original.document,
  )
  outputs.push({ step: 'initial', ...first.candidate })
  checks.push('retry after cancellation returns a real cited proposal without publishing')
  console.log('PASS live proposal and explicit source evidence')
  await click('.proposal-card .text-link')
  assert.match(await value('.evidence-dialog[open]'), /booking calendar/)
  await clickButton('Back to discussion')

  await clickButton('Ask for revision')
  assert.ok((await value('.previous-proposal-context')).includes(first.candidate.proposedWording))
  await type(
    '[aria-label="Agent request"]',
    'Make that shorter, keeping the booking calendar and equipment page. Use a single concise sentence.',
  )
  await clickButton('Request suggestion')
  const revised = await pendingProposal(threadId, first.id)
  assert.equal(revised.parentId, first.id)
  assert.equal(
    (await discussion(threadId)).proposals.find((p) => p.id === first.id).status,
    'superseded',
  )
  outputs.push({ step: 'revised', ...revised.candidate })
  checks.push('live follow-up revises the disclosed prior draft with pinned evidence')
  console.log('PASS live follow-up revision')

  await clickButton('Reject')
  await until(
    async () =>
      (await discussion(threadId)).proposals.find((p) => p.id === revised.id).status === 'rejected',
    'live proposal rejected',
  )
  assert.deepEqual(
    (await invoke('read_page', { id: 'getting-started' })).document,
    original.document,
  )
  assert.equal((await invoke('page_history', { id: 'getting-started' })).length, 1)
  checks.push('rejecting live output preserves wording, citations and revision count')

  await readyRequest(
    'Write one brief replacement directing readers to the booking calendar linked from the equipment page, as stated in the chosen excerpt.',
  )
  await clickButton('Request suggestion')
  const acceptedCandidate = await pendingProposal(threadId)
  outputs.push({ step: 'accepted', ...acceptedCandidate.candidate })
  await clickButton('Accept edit')
  await until(
    async () => (await value('.metadata'))?.includes('Revision 2'),
    'live wording and citation accepted',
  )
  assert.equal(
    await value('[data-block-id=booking]'),
    acceptedCandidate.candidate.proposedWording + '[1]',
  )
  await click('[data-citation-id]')
  assert.match(await value('.evidence-dialog[open]'), /booking calendar/)
  await clickButton('Back to discussion')
  await clickButton('Page history')
  await until(
    async () =>
      (await value('.revision-content'))?.includes(acceptedCandidate.candidate.proposedWording),
    'live accepted revision in history',
  )
  assert.match(await value('.revision-discussions'), /accepted/)
  await clickButton('Back to page')
  checks.push('accepting live output saves exact wording and citation together with history')
  await stop()
  await launch()
  assert.equal(
    await value('[data-block-id=booking]'),
    acceptedCandidate.candidate.proposedWording + '[1]',
  )
  const restored = await discussion(threadId)
  assert.equal(restored.proposals.find((p) => p.id === revised.id).status, 'rejected')
  assert.equal(restored.proposals.find((p) => p.id === acceptedCandidate.id).status, 'accepted')
  checks.push('accepted page and all review decisions survive process restart')
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify({ passed: true, native: true, live: true, model, checks, outputs }, null, 2) +
      '\n',
  )
  console.log(
    'PASS live review route: cancellation, retry, revision, rejection, acceptance, history and restart',
  )
} catch (error) {
  await writeFile(
    join(artifacts, 'result.json'),
    JSON.stringify(
      { passed: false, native: true, live: true, checks, error: String(error) },
      null,
      2,
    ) + '\n',
  )
  throw error
} finally {
  // No screenshots, account identity, credentials, raw network data or application logs are saved by this live check.
  await stop()
}
