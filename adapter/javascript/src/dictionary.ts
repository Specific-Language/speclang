import { specify } from './specify'
import type { $Specification, $Dictionary, $Reference, $Value, $ReferenceMap } from './types'

export function get(dictionary: $Dictionary, name: string, unique: string): $Specification | undefined {
  return dictionary[name] 
    ? dictionary[name][unique] 
    : undefined
}

export function define_spec(dictionary: $Dictionary, name: string, value: $Value): $Reference {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  const reference: $Reference = [name, unique]
  dictionary[name] ??= {}
  dictionary[name][unique] ??= {}
  specify(dictionary, reference, value)
  return reference
}

export function set_value(dictionary: $Dictionary, spec_ref: $Reference, value: $Value) {
  const spec = get(dictionary, ...spec_ref)
  if (spec === undefined) throw Error('expected to find value')
  spec.value = value
}

export function add_reference(property: KeyWithType<$Specification, $ReferenceMap | undefined>, dictionary: $Dictionary, target: $Reference, value: $Reference) {
  const spec = get(dictionary, ...target)
  if (spec === undefined) throw Error('expected to find value')
  spec[property] ??= {}
  const referenceMap = spec[property]
  if (referenceMap === undefined) throw Error('really expected to find value (typescript bug shim)')
  const [name, unique] = value
  referenceMap[name] = unique
}

// https://stackoverflow.com/a/49752227/679184 :^)
type KeyWithType<T, V> = keyof {
  [P in keyof T as T[P] extends V ? P : never]: unknown
}
