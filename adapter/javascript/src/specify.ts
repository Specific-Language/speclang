import type { $Context, $Map, $Reference, $Value } from './types'
import { apply_define, apply_extend, apply_value, create_reference } from './dictionary'

export function specify(context: $Context, ref: $Reference, value: $Value) {
  if (value instanceof Object && !(value instanceof Array)) {
    const { define, extend, ...values } = value
    console.log(ref, 'values', values)
    define && specify_reserved(context, apply_define, ref, define)
    extend && specify_reserved(context, apply_extend, ref, extend)
    values && ordered_entries(values).forEach(([name, value]) => {
      const child_ref = create_reference(context, name, value)
      apply_value(context, child_ref, value, ref)
    })
  } else {
    apply_value(context, ref, value)
  }
}

function specify_reserved(context: $Context, apply_fn: typeof apply_define | typeof apply_extend, ref: $Reference, value: $Value) {
  if (value instanceof Object && !(value instanceof Array)) {
    ordered_entries(value).forEach(([child_name, child_value]) => {
      if (child_value instanceof Array) {
        child_value.forEach((child_value_entry) => {
          const child_ref = create_reference(context, child_name, child_value_entry)
          apply_fn(context, child_ref, ref)
        })
      } else {
        const child_ref = create_reference(context, child_name, child_value)
        apply_fn(context, child_ref, ref)
      }
    })
  } else {
    throw Error('The "define" and "extend" keywords are reserved and cannot be redefined')
  }
}

function ordered_entries(spec: $Map) {
  return Object.entries(spec).sort((a, b) => {
    const [name_a, value_a] = a
    const [name_b, value_b] = b
    if (value_a instanceof Object && !(value_a instanceof Array)) {
      if (has_reference(name_b, value_a)) {
        return 1
      }
    }
    if (value_b instanceof Object && !(value_b instanceof Array)) {
      if (has_reference(name_a, value_b)) {
        return -1
      }
    }
    return 0
  })
}

function has_reference(name: string, spec: $Map): boolean {
  return Object.keys(spec).some((key) => {
    if (key === name) {
      return true
    }
    const value = spec[key]
    if (value instanceof Object && !(value instanceof Array)) {
      return has_reference(name, value)
    }
    return false
  })
}
