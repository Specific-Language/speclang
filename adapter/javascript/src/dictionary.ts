import type { $Context, $Map, $Value } from './types'

export function get_value(context: $Context, reference: string): $Value | undefined {
  context.option?.verbose && console.log(' * get_value', { reference })
  context.extend ??= {}
  return context.extend[reference]
}

export function set_value(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log(' * set_value', { reference, value })
  context.extend ??= {}
  context.extend[reference] ??= []
  context.extend[reference].push(value)
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
