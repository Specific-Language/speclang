import type { $Context, $Map, $Reference, $Value } from './types'
import { apply_define, apply_extend, apply_value, create_reference } from './dictionary'

export function specify(context: $Context, ref: $Reference, spec: $Value) {
  if (spec instanceof Array) {
    throw Error('specify: handle array unimplemented')
  } else if (spec instanceof Object) {
    const { define, ...extend } = spec
    define && specify_define(context, ref, define)
    extend && specify_extend(context, ref, extend)
  } else {
    specify_extend(context, ref, spec)
  }
}

function specify_define(context: $Context, ref: $Reference, define: $Value) {
  if (define instanceof Array) {
    define.forEach((value) => specify_define(context, ref, value))
  } else if (define instanceof Object) {
    ordered_entries(define).forEach(([name, value]) => {
      if (value instanceof Array) {
        value.forEach((child_value) => {
          const child_ref = create_reference(context, name, child_value)
          apply_define(context, child_ref, ref)
        })
      } else {
        const child_ref = create_reference(context, name, value)
        apply_define(context, child_ref, ref)
      }
    })
  } else {
    throw Error('The "define" keyword is reserved and cannot be redefined')
  }
}

function specify_extend(context: $Context, ref: $Reference, extend: $Value) {
  if (extend instanceof Object && !(extend instanceof Array)) {
    ordered_entries(extend).forEach(([name, value]) => {
      if (value instanceof Array) {
        value.forEach((entry) => {
          const child_ref = create_reference(context, name, entry)
          apply_extend(context, child_ref, ref)
        })
      } else {
        const child_ref = create_reference(context, name, value)
        apply_extend(context, child_ref, ref)
      }
    })
  } else {
    apply_value(context, ref, extend)
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
