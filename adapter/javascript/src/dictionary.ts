import type { $Context, $Map, $Value } from './types'

export function lookup(context: $Context, name: string): $Context {
  const result: $Context = {}
  result.value ??= {}
  context.value ??= {}
  if (context.value[name] !== undefined) {
    result.value[name] = context.value[name]
  }
  return result
}

export function set_alias(context: $Context, reference: string, alias: string) {
  context.alias ??= {}
  context.alias[alias] ??= []
  context.alias[alias].push(reference)
}

export function set_value(context: $Context, reference: string, value: $Value) {
  // if (['string', 'number', 'boolean'].includes(reference)) {
  //   if (typeof value !== reference) {
  //     throw Error('Literal values (string, number, boolean) are constrained by type')
  //   }
  // }
  context.value ??= {}
  context.value[reference] ??= []
  context.value[reference].push(value)
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
