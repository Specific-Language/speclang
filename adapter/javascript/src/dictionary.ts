import { specify } from './specify'
import type { $Definition, $Dictionary, $Reference, $Value } from './types'

export function get(dictionary: $Dictionary, name: string, unique: string): $Definition | undefined {
  return dictionary[name] 
    ? dictionary[name][unique] 
    : undefined
}

// todo : handle origin / parent
export function create_definition(dictionary: $Dictionary, name: string, value: $Value): $Reference {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  const child_ref: $Reference = [name, unique]
  dictionary[name] ??= {}
  dictionary[name][unique] ??= {}
  specify(dictionary, child_ref, value)
  return child_ref
}

// export function origin_reference(definition: $Definition, [name, unique]: $Reference) {
//   definition.origin ??= {}
//   definition.origin[name] = unique
// }

export function define_reference(definition: $Definition, [name, unique]: $Reference) {
  definition.define ??= {}
  definition.define[name] = unique
}

export function extend_reference(definition: $Definition, [name, unique]: $Reference) {
  definition.extend ??= {}
  definition.extend[name] = unique
}

export function set_value(definition: $Definition, value: $Value) {
  definition.value = value
}
