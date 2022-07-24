import type { $DefinitionTable, $EventTable } from './types'

export class Context {
  dictionary: $DefinitionTable = {}
  events: $EventTable = {}
}
