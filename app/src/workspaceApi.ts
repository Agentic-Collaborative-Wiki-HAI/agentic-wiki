import { invoke } from '@tauri-apps/api/core'
import type { WikiRecord } from './generated/WikiRecord'
import type { Page } from './generated/Page'
import type { PassageResult } from './generated/PassageResult'
import type { Evidence } from './generated/Evidence'
import type { EvidenceSelection } from './generated/EvidenceSelection'
import type { EvidenceContext } from './generated/EvidenceContext'
import type { PassageAnchor } from './generated/PassageAnchor'
import type { WorkspaceCopy } from './generated/WorkspaceCopy'
export const workspaceApi = {
  records: () => invoke<WikiRecord[]>('wiki_records'),
  create: (id: string, title: string, parentId: string | null) =>
    invoke<Page>('create_wiki_page', { id, title, parentId }),
  organize: (id: string, metadataVersion: number, title: string, parentId: string | null) =>
    invoke<Page>('organize_wiki_page', { id, metadataVersion, title, parentId }),
  search: (query: string, objectIds: string[] | null = null) =>
    invoke<PassageResult[]>('search_passages', { query, objectIds }),
  evidence: (source: EvidenceSelection) => invoke<Evidence>('select_evidence', { source }),
  context: (id: string, query: string) =>
    invoke<EvidenceContext>('default_evidence', { id, query }),
  cite: (
    id: string,
    pageId: string,
    baseRevision: number,
    anchor: PassageAnchor,
    source: EvidenceSelection,
  ) => invoke<Page>('add_manual_citation', { id, pageId, baseRevision, anchor, source }),
  export: (path: string) => invoke<string>('export_workspace', { path }),
  restore: (path: string) => invoke<WorkspaceCopy>('restore_workspace', { path }),
  copies: () => invoke<WorkspaceCopy[]>('workspace_copies'),
  select: (id: string) => invoke<void>('select_workspace', { id }),
}
