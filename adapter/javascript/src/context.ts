import type { $Definition, $DefinitionTable, $EventTable } from './types'

export class Context {
  dictionary = new Dictionary()
  events: $EventTable = {}
}

export class Dictionary {
  map: $DefinitionTable = {}

  get = (key: string) => this.map[key] ?? []
  push = (key: string, value: $Definition) => {
    if (this.map[key] === undefined) {
      this.map[key] = []
    }
    this.map[key].push(value)
  }
}
