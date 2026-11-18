import { invoke, isTauri } from '@tauri-apps/api/core'
import type { Page } from './generated/Page'
import type { RichDocument } from './generated/RichDocument'
import type { Draft } from './generated/Draft'
import type { HistoryEntry } from './generated/HistoryEntry'

function call<T>(command: string, args: Record<string, unknown>): Promise<T> {
  if (!isTauri())
    return Promise.reject(
      new Error(
        'Open the desktop app with npm run desktop. This browser preview has no database connection.',
      ),
    )
  return invoke<T>(command, args)
}
export const api = {
  read: (id: string) => call<Page>('read_page', { id }),
  save: (id: string, baseRevision: number, document: RichDocument, title?: string) =>
    call<Page>('save_page', { id, baseRevision, document, title }),
  draft: (id: string) => call<Draft | null>('read_draft', { id }),
  putDraft: (id: string, baseRevision: number, document: RichDocument, title?: string) =>
    call<void>('save_draft', { id, baseRevision, document, title }),
  discard: (id: string) => call<void>('discard_draft', { id }),
  history: (id: string) => call<HistoryEntry[]>('page_history', { id }),
}
