import { specify } from './specify'
import type { $Specification, $Dictionary, $Reference, $Value } from './types'

export function get(dictionary: $Dictionary, name: string, unique: string): $Specification | undefined {
  return dictionary[name] 
    ? dictionary[name][unique] 
    : undefined
}

// todo : handle origin / parent
export function define_spec(dictionary: $Dictionary, name: string, value: $Value): $Reference {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  const reference: $Reference = [name, unique]
  dictionary[name] ??= {}
  dictionary[name][unique] ??= {}
  specify(dictionary, reference, value)
  return reference
}

// export function origin_reference(spec: $Specification, [name, unique]: $Reference) {
//   spec.origin ??= {}
//   spec.origin[name] = unique
// }

export function define_reference(spec: $Specification, [name, unique]: $Reference) {
  spec.define ??= {}
  spec.define[name] = unique
}

export function extend_reference(spec: $Specification, [name, unique]: $Reference) {
  spec.extend ??= {}
  spec.extend[name] = unique
}

export function set_value(spec: $Specification, value: $Value) {
  spec.value = value
}
