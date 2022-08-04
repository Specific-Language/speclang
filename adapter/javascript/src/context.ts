import { define_value } from './define'
import { extend_value } from './extend'
import type { $Definition, $Dictionary,  $Object,  $Value } from './types'

export class Context {
  dictionary: $Dictionary = {}
  
  specify(target: $Definition, input: $Object): void {
    const { define, ...extend } = input
    define && define_value(this, target, define)
    extend && extend_value(this, target, extend)
  }
  
  define(name: string, value: $Definition, parent?: $Definition): void {
    this.dictionary[name] ??= []
    this.dictionary[name].push(value)
    if (parent) {
      parent.define ??= {}
      parent.define[name] = value.id
    }
  }

  extend(parent: $Definition, name: string, value: $Value): void {
    parent.extend ??= {}
    parent.extend[name] = value
  }

  lookup(name: string): $Definition[] {
    return this.dictionary[name] ?? []
  }

  get(name: string, id: string): $Definition {
    const result = this.lookup(name).filter((definition) => {
      const [result_name, result_id] = definition.id
      return (result_name === name) && (result_id === id)
    })
    if (result.length !== 1) {
      throw Error(`get: Expected 1 result, got ${result.length}`)
    }
    return result[0]
  }
}

export function Specification(value: $Value): $Object {
  if (value instanceof Array) {
    throw Error('unhandled case: compose_spec: Array')
  }
  if (value instanceof Object) {
    return value
  }
  return {
    [typeof value]: value
  }
}
