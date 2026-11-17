import { describe, it, expect } from 'vitest'
import { ancestors, visibleTree } from '../src/wikiTree'
import type { WikiRecord } from '../src/generated/WikiRecord'
const make = (id: string, parentId: string | null): WikiRecord => ({
  id,
  title: id,
  parentId,
  kind: 'page',
  revision: 1,
  metadataVersion: 1,
  imported: false,
})
describe('saved wiki hierarchy', () => {
  const records = [
    make('root', null),
    make('child', 'root'),
    make('source', 'child'),
    make('other', null),
  ]
  it('keeps collapse independent from selection and reports child depth', () => {
    expect(visibleTree(records, new Set()).map((r) => [r.record.id, r.depth])).toEqual([
      ['root', 0],
      ['child', 1],
      ['source', 2],
      ['other', 0],
    ])
    expect(visibleTree(records, new Set(['child'])).map((r) => r.record.id)).toEqual([
      'root',
      'child',
      'other',
    ])
  })
  it('derives breadcrumbs from current parents and stops malformed loops', () => {
    expect(ancestors(records, 'source').map((r) => r.id)).toEqual(['root', 'child', 'source'])
    expect(ancestors([make('a', 'b'), make('b', 'a')], 'a')).toHaveLength(2)
  })
})
