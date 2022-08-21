import { set_define, set_extend, set_value } from './dictionary'
import { generate_reference, get_ordered_entries } from './reference'
// import { test } from './test'
import type { $Context, $Map, $Reference, $Value } from './types'

export function specify(context: $Context, name: string, value: $Value): $Reference {
  const reference = generate_reference(name)
  if (value instanceof Object && !(value instanceof Array)) {
    const { define, extend, ...values } = value
    context.option?.verbose && console.log('| specify', reference, value)
    define && specify_define(context, reference, define)
    extend && specify_extend(context, reference, extend)
    values && specify_values(context, reference, values)
  } else {
    set_value(context, reference, value)
  }
  return reference
}

function specify_define(context: $Context, ref: $Reference, value: $Value) {
  if (value instanceof Array) {
    value.forEach((sibling_value) => specify_define(context, ref, sibling_value))
  } else if (value instanceof Object) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      if (child_value instanceof Array) {
        child_value.forEach((child_value_entry) => {
          const child_ref = specify(context, child_name, child_value_entry)
          set_define(context, child_ref, ref)
        })
      } else {
        const child_ref = specify(context, child_name, child_value)
        set_define(context, child_ref, ref)
      }
    })
  } else {
    throw Error('The "define" keyword is reserved and cannot be redefined')
  }
}

function specify_extend(context: $Context, ref: $Reference, value: $Value) {
  if (value instanceof Array) {
    value.forEach((sibling_value) => specify_extend(context, ref, sibling_value))
  } else if (value instanceof Object && !(value instanceof Array)) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      if (child_value instanceof Array) {
        child_value.forEach((child_value_entry) => {
          const child_ref = specify(context, child_name, child_value_entry)
          set_extend(context, child_ref, ref)
        })
      } else {
        const child_ref = specify(context, child_name, child_value)
        set_extend(context, child_ref, ref)
      }
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}

function specify_values(context: $Context, target_ref: $Reference, values: $Map) {
  context.option?.verbose && console.log('| specify_values', target_ref, values)
  get_ordered_entries(values).forEach(([name, value]) => {
    const child_ref = generate_reference(name)
    set_value(context, child_ref, value, target_ref)
  })
}
