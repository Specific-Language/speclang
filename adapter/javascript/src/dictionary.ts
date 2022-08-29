import type { $Context, $Map, $Value } from './types'

export function lookup(context: $Context, name: string): $Context {
  const result: $Context = {}
  result.define ??= {}
  context.define ??= {}
  if (context.define[name] !== undefined) {
    result.define[name] = context.define[name]
  }
  return result
}

export function define_value(context: $Context, reference: string, value: $Value) {
  // if (['string', 'number', 'boolean'].includes(reference)) {
  //   if (typeof value !== reference) {
  //     throw Error('Literal values (string, number, boolean) are constrained by type')
  //   }
  // }
  context.define ??= {}
  context.define[reference] ??= []
  context.define[reference].push(value)
}

export function has_reference(name: string, value: $Map): boolean {
  return Object.keys(value).some((key) => {
    if (key === name) {
      return true
    }
    const child_value = value[key]
    if (child_value instanceof Object && !(child_value instanceof Array)) {
      return has_reference(name, child_value)
    }
    return false
  })
}

export function get_ordered_entries(value: $Map) {
  return Object.entries(value).sort((a, b) => {
    let result = 0
    const [name_a, value_a] = a
    const [name_b, value_b] = b
    if (value_a instanceof Object && !(value_a instanceof Array)) {
      has_reference(name_b, value_a) && result++
    }
    if (value_b instanceof Object && !(value_b instanceof Array)) {
      has_reference(name_a, value_b) && result--
    }
    return result
  })
}
