import { invoke } from '@tauri-apps/api/core'
import type { AgentInstructions } from './generated/AgentInstructions'
import type { ReferenceEntry } from './generated/ReferenceEntry'
import type { RichDocument } from './generated/RichDocument'
import type { EvidenceSelection } from './generated/EvidenceSelection'
export const authoringApi = {
  instructions: () => invoke<AgentInstructions>('read_agent_instructions'),
  defaults: () => invoke<AgentInstructions>('default_agent_instructions'),
  saveInstructions: (instructions: AgentInstructions) =>
    invoke<void>('save_agent_instructions', { instructions }),
  citation: (source: EvidenceSelection) => invoke<ReferenceEntry>('register_citation', { source }),
  note: (title: string, body: string) =>
    invoke<ReferenceEntry>('register_margin_note', { title, body }),
  references: (document: RichDocument) =>
    invoke<ReferenceEntry[]>('document_references', { document }),
}
