import { invoke } from '@tauri-apps/api/core'
import type { SourceSettings } from './generated/SourceSettings'
import type { SourceProvider } from './generated/SourceProvider'
import type { SourceListing } from './generated/SourceListing'
import type { ImportPreview } from './generated/ImportPreview'
import type { ImportKind } from './generated/ImportKind'
import type { ImportedSource } from './generated/ImportedSource'
import type { Page } from './generated/Page'
export const sourceApi = {
  settings: () => invoke<SourceSettings>('source_settings'),
  saveSettings: (settings: SourceSettings) => invoke<void>('save_source_settings', { settings }),
  installedTool: () => invoke<string | null>('installed_drive_tool'),
  list: (provider: SourceProvider, folderId: string | null = null) =>
    invoke<SourceListing>('list_import_sources', { provider, folderId }),
  preview: (provider: SourceProvider, key: string) =>
    invoke<ImportPreview>('preview_source_import', { provider, key }),
  confirm: (token: string, title: string, kind: ImportKind, parentId: string | null) =>
    invoke<Page>('confirm_source_import', { token, title, kind, parentId }),
  cancel: (token: string) => invoke<void>('cancel_source_import', { token }),
  imported: () => invoke<ImportedSource[]>('imported_sources'),
  openOriginal: (id: string) => invoke<void>('open_import_original', { id }),
}
