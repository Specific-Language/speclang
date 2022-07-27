import type { $Definition, $Dictionary } from './types'

export class Context {
  dictionary: $Dictionary = {}

  define(name: string, value: $Definition) {
    if (this.dictionary[name] === undefined) {
      this.dictionary[name] = []
    }
    this.dictionary[name].push(value)
  }
  lookup(name: string, id?: string): $Definition[] {
    const result = this.dictionary[name]
    if (id && result instanceof Array) {
      return result.filter((d) => d.id === id)
    }
    return result
  }
  lookupID(id: string): $Definition[] {
    const [name, uuid] = id.split('-')
    return this.lookup(name, uuid)
  }
}
