import { invoke } from '@tauri-apps/api/core'
import type { EvidenceSelection } from './generated/EvidenceSelection'
import type { EvidenceContext } from './generated/EvidenceContext'
import type { ChatTurn } from './generated/ChatTurn'
import type { AnswerDraft } from './generated/AnswerDraft'
import type { Page } from './generated/Page'
import type { RichDocument } from './generated/RichDocument'
export const chatApi = {
  context: (id: string, query: string) => invoke<EvidenceContext>('chat_context', { id, query }),
  turns: () => invoke<ChatTurn[]>('chat_turns'),
  ask: (request: {
    id: string
    pageId: string
    question: string
    sources: EvidenceSelection[]
    model: string
  }) => invoke<ChatTurn>('ask_chat', { request }),
  drafts: () => invoke<AnswerDraft[]>('answer_drafts'),
  createDraft: (turnId: string) => invoke<AnswerDraft>('create_answer_draft', { turnId }),
  saveDraft: (
    id: string,
    version: number,
    title: string,
    parentId: string | null,
    document: RichDocument,
  ) => invoke<AnswerDraft>('save_answer_draft', { id, version, title, parentId, document }),
  publish: (id: string, version: number) => invoke<Page>('publish_answer_draft', { id, version }),
  origin: (id: string) => invoke<ChatTurn | null>('chat_page_origin', { id }),
}
