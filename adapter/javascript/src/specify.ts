import type { $Specification, $Dictionary, $Reference, $Value, $Map } from "./types"
import { define_spec, define_reference, extend_reference, set_value } from "./dictionary"
import { get } from "./dictionary"

export function specify(dictionary: $Dictionary, reference: $Reference, value: $Value) {
  const spec = get(dictionary, ...reference)
  if (spec === undefined) {
    throw Error('expected reference to exist')
  }
  handle_value(value,
    (/* object */) => {
      const { define, ...extend } = value as $Map
      define && handle_define(dictionary, spec, define)
      extend && handle_extend(dictionary, spec, extend)
    },
    (/* primitive */) => {
      handle_extend(dictionary, spec, value)
    }
  )
}

function handle_define(dictionary: $Dictionary, spec: $Specification, value: $Value) {
  handle_value(value,
    (/* object */) => Object.entries(value).forEach(([name, value]) => {
      const child_ref = define_spec(dictionary, name, value)
      define_reference(spec, child_ref)
    }),
    (/* primitive */) => {
      const child_ref = define_spec(dictionary, typeof value, value)
      define_reference(spec, child_ref)
    }
  )
}

function handle_extend(dictionary: $Dictionary, spec: $Specification, value: $Value) {
  handle_value(value,
    (/* object */) => Object.entries(value).forEach(([child_name, child_value]) => {
      const child_ref = define_spec(dictionary, child_name, child_value)
      extend_reference(spec, child_ref)
    }),
    (/* primitive */) => {
      set_value(spec, value)
    }
  )
}


function handle_value(value: $Value, handle_object: () => void, handle_primitive: () => void) {
  if (value instanceof Object && !(value instanceof Array)) {
    return handle_object()
  }
  return handle_primitive()
}
