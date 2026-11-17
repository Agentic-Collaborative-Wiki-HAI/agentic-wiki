import { invoke } from '@tauri-apps/api/core'
import type { PassageAnchor } from './generated/PassageAnchor'
import type { Discussion } from './generated/Discussion'
import type { Evidence } from './generated/Evidence'
import type { EvidenceSelection } from './generated/EvidenceSelection'
import type { ConnectionStatus } from './generated/ConnectionStatus'
import type { AvailableModel } from './generated/AvailableModel'
import type { Page } from './generated/Page'

export interface AskRequest {
  runId: string
  threadId: string
  requestText: string
  sources: EvidenceSelection[]
  anchor: PassageAnchor
  baseRevision: number
  parentId: string | null
  model: string
  messageId?: string | null
}
export const reviewApi = {
  threads: (id: string) => invoke<Discussion[]>('list_discussions', { id }),
  start: (id: string, baseRevision: number, anchor: PassageAnchor, body: string) =>
    invoke<Discussion>('start_discussion', { id, baseRevision, anchor, body }),
  reply: (id: string, body: string) => invoke<Discussion>('reply_discussion', { id, body }),
  resolve: (id: string, resolved: boolean) =>
    invoke<Discussion>('resolve_discussion', { id, resolved }),
  decide: (id: string, accept: boolean) => invoke<Discussion>('decide_proposal', { id, accept }),
  revise: (id: string, wording: string, reason: string) =>
    invoke<Discussion>('revise_proposal', { id, wording, reason }),
  sources: (id: string) => invoke<Evidence[]>('discussion_evidence', { id }),
  citations: (id: string, revision: number) =>
    invoke<Evidence[]>('page_citations', { id, revision }),
  revision: (id: string, revision: number) => invoke<Page>('read_revision', { id, revision }),
  connection: () => invoke<ConnectionStatus>('subscription_status'),
  signIn: () => invoke<void>('subscription_sign_in'),
  cancelSignIn: () => invoke<void>('subscription_cancel_sign_in'),
  models: () => invoke<AvailableModel[]>('subscription_models'),
  ask: (request: AskRequest) => invoke<Discussion>('ask_agent', { request }),
  cancel: (runId: string) => invoke<boolean>('cancel_agent', { runId }),
}
