import type { $Definition, $Dictionary, $Object } from './types'

export class Context {
  options: $Object = {
    verbose: false,
  }
  dictionary: $Dictionary = {}

  constructor(options?: $Object) {
    this.options = options ?? this.options
  }
  
  define(name: string, value: $Definition) {
    if (this.dictionary[name] === undefined) {
      this.dictionary[name] = []
    }
    this.dictionary[name].push(value)
  }
  lookupName(name: string): $Definition[] {
    return this.dictionary[name]
  }
  lookupID(name: string, id: string): $Definition[] {
    const result = this.lookupName(name)
    if (result instanceof Array) {
      return result.filter((d) => d.id === id)
    }
    return result
  }
}
