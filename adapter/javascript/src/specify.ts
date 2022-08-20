import { apply_define, apply_extend, apply_value } from './dictionary'
import { generate_reference, get_ordered_entries } from './reference'
import type { $Context, $Map, $Reference, $Value } from './$types'

export function specify(context: $Context, name: string, value: $Value): $Reference {
  const reference = generate_reference(name)
  if (value instanceof Object && !(value instanceof Array)) {
    const { define, extend, ...values } = value
    context.option?.verbose && console.log('| specify', reference, value)
    define && specify_reserved(context, apply_define, reference, define)
    extend && specify_reserved(context, apply_extend, reference, extend)
    values && specify_values(context, reference, values)
  } else {
    apply_value(context, reference, value)
  }
  return reference
}

function specify_reserved(context: $Context, apply_fn: typeof apply_define | typeof apply_extend, ref: $Reference, value: $Value) {
  if (value instanceof Object && !(value instanceof Array)) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      if (child_value instanceof Array) {
        child_value.forEach((child_value_entry) => {
          const child_ref = specify(context, child_name, child_value_entry)
          apply_fn(context, child_ref, ref)
        })
      } else {
        const child_ref = specify(context, child_name, child_value)
        apply_fn(context, child_ref, ref)
      }
    })
  } else {
    throw Error('The "define" and "extend" keywords are reserved and cannot be redefined')
  }
}

function specify_values(context: $Context, target_ref: $Reference, values: $Map) {
  context.option?.verbose && console.log('| specify_values', target_ref, values)
  get_ordered_entries(values).forEach(([name, value]) => {
    const child_ref = generate_reference(name)
    apply_value(context, child_ref, value, target_ref)
  })
}
