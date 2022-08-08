import { create_definition, define_reference, extend_reference, set_value } from './definition'
import type { $Definition, $Dictionary, $Object, $Reference, $Value } from './types'

export function specify(dictionary: $Dictionary, reference: $Reference, value: $Value) {
  if (value instanceof Array) {
    throw Error('unhandled case: define_value: array')
  }
  if (value instanceof Object) {
    const { define, ...extend } = Specification(value)
    define && define_spec(dictionary, reference, define)
    extend && extend_spec(dictionary, reference, extend)
    return
  }
  extend_spec(dictionary, reference, value)
}

function define_spec(dictionary: $Dictionary, reference: $Reference, value: $Value) {
  const definition = get(dictionary, reference)
  if (value instanceof Array) {
    throw Error('unhandled case: define_value: array')
  }
  if (value instanceof Object) {
    return Object.entries(value).forEach(([name, value]) => {
      const child_ref = create_definition(dictionary, name, value)
      define_reference(definition, child_ref)
    })
  }
  const child_ref = create_definition(dictionary, typeof value, value)
  define_reference(definition, child_ref)
}

export function extend_spec(dictionary: $Dictionary, reference: $Reference, input: $Value): void {
  const definition = get(dictionary, reference)
  if (input instanceof Array) {
    throw Error('unhandled case: extend_value: array')
  }
  if (input instanceof Object) {
    return Object.entries(input).forEach(([child_name, child_value]) => {
      const child_ref = create_definition(dictionary, child_name, child_value)
      extend_reference(definition, child_ref)
    })
  }
  set_value(definition, input)
}

export function get(dictionary: $Dictionary, [name, unique]: $Reference): $Definition {
  const result = dictionary[name] ?? {}
  return result[unique] ?? {}
}

export function Specification(value: $Value): $Object {
  if (value instanceof Array) {
    return {
      'array': value
    }
  }
  if (value instanceof Object) {
    return value
  }
  return {
    [typeof value]: value
  }
}
