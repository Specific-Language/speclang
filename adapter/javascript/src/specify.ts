import { create_definition, define_reference, extend_reference, set_value } from "./dictionary"
import { get } from "./dictionary"
import { $Dictionary, $Reference, $Value } from "./types"

export function specify(dictionary: $Dictionary, reference: $Reference, value: $Value) {
  if (value instanceof Array) {
    throw Error('unhandled case: specify: array')
  }
  if (value instanceof Object) {
    const { define, ...extend } = value
    define && specify_define(dictionary, reference, define)
    extend && specify_extend(dictionary, reference, extend)
    return
  }
  specify_extend(dictionary, reference, value)
}

function specify_define(dictionary: $Dictionary, reference: $Reference, value: $Value) {
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

function specify_extend(dictionary: $Dictionary, reference: $Reference, input: $Value): void {
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
