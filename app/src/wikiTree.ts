import type { WikiRecord } from './generated/WikiRecord'
export function ancestors(records: WikiRecord[], id: string): WikiRecord[] {
  const out: WikiRecord[] = [],
    seen = new Set<string>()
  let next: string | null = id
  while (next && !seen.has(next)) {
    seen.add(next)
    const record = records.find((r) => r.id === next)
    if (!record) break
    out.unshift(record)
    next = record.parentId
  }
  return out
}
export function visibleTree(records: WikiRecord[], collapsed: Set<string>) {
  const out: { record: WikiRecord; depth: number; children: boolean }[] = []
  const seen = new Set<string>()
  function walk(parent: string | null, depth: number) {
    for (const record of records.filter((r) => r.parentId === parent)) {
      if (seen.has(record.id)) continue
      seen.add(record.id)
      const children = records.some((r) => r.parentId === record.id)
      out.push({ record, depth, children })
      if (!collapsed.has(record.id)) walk(record.id, depth + 1)
    }
  }
  walk(null, 0)
  return out
}
