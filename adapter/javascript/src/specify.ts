import type { $Dictionary, $Reference, $Value, $Map, $Primitive } from "./types"
import { define_spec, add_reference, set_value } from "./dictionary"

export function specify(dictionary: $Dictionary, spec_ref: $Reference, value: $Value) {
  handle_value(value,
    (/* object */) => {
      const { define, ...extend } = value as $Map
      define && handle_define(dictionary, spec_ref, define)
      extend && handle_extend(dictionary, spec_ref, extend)
    },
    (/* primitive */) => {
      handle_extend(dictionary, spec_ref, value)
    }
  )
}

function handle_define(dictionary: $Dictionary, spec_ref: $Reference, value: $Value) {
  handle_value(value,
    (value) => Object.entries(value).forEach(([name, value]) => {
      const child_ref = define_spec(dictionary, name, value)
      add_reference('define', dictionary, spec_ref, child_ref)
      add_reference('relate', dictionary, child_ref, spec_ref)
    }),
    (value) => { /* primitive */
      const child_ref = define_spec(dictionary, typeof value, value)
      add_reference('define', dictionary, spec_ref, child_ref)
    }
  )
}

function handle_extend(dictionary: $Dictionary, spec_ref: $Reference, value: $Value) {
  handle_value(value,
    (/* object */) => Object.entries(value as $Map).forEach(([child_name, child_value]) => {
      const child_ref = define_spec(dictionary, child_name, child_value)
      add_reference('extend', dictionary, spec_ref, child_ref)
      add_reference('relate', dictionary, child_ref, spec_ref)
    }),
    (/* primitive */) => {
      set_value(dictionary, spec_ref, value)
    }
  )
}

function handle_value(value: $Value, handle_object: (value: $Map) => void, handle_primitive: (value: $Primitive) => void) {
  if (value instanceof Object && !(value instanceof Array)) {
    return handle_object(value)
  }
  return handle_primitive(value)
}
