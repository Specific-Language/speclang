import { create_definition, define_reference, extend_reference, set_value } from "./dictionary"
import { get } from "./dictionary"
import { $Definition, $Dictionary, $Reference, $Value } from "./types"

export function specify(dictionary: $Dictionary, reference: $Reference, value: $Value) {
  const definition = get(dictionary, reference)
  if (!definition) {
    throw Error('expected reference to exist')
  }
  if (value instanceof Object && !(value instanceof Array)) {
    const { define, ...extend } = value
    define && specify_define(dictionary, definition, define)
    extend && specify_extend(dictionary, definition, extend)
    return
  }
  specify_extend(dictionary, definition, value)
}

function specify_define(dictionary: $Dictionary, definition: $Definition, value: $Value) {
  if (value instanceof Object && !(value instanceof Array)) {
    return Object.entries(value).forEach(([name, value]) => {
      const child_ref = create_definition(dictionary, name, value)
      define_reference(definition, child_ref)
    })
  }
  const child_ref = create_definition(dictionary, typeof value, value)
  define_reference(definition, child_ref)
}

function specify_extend(dictionary: $Dictionary, definition: $Definition, input: $Value): void {
  if (input instanceof Object && !(input instanceof Array)) {
    return Object.entries(input).forEach(([child_name, child_value]) => {
      const child_ref = create_definition(dictionary, child_name, child_value)
      extend_reference(definition, child_ref)
    })
  }
  set_value(definition, input)
}
