import type { $Context, $Reference, $Value } from './types'
import { assign, create_definition, create_extend } from './dictionary'

export function specify(context: $Context, ref: $Reference, spec: $Value) {
  if (spec instanceof Array) {
    spec.forEach((entry) => {
      console.log(entry)
      throw Error('specify: handle array unimplemented')
    })
  } else if (spec instanceof Object) {
    const { define, ...extend } = spec
    define && handle_define(context, ref, define)
    extend && handle_extend(context, ref, extend)
  } else {
    handle_extend(context, ref, spec)
  }
}

function handle_define(context: $Context, ref: $Reference, define: $Value) {
  if (define instanceof Array) {
    define.forEach((value) => handle_define(context, ref, value))
  } else if (define instanceof Object) {
    Object.entries(define).forEach(([name, value]) => {
      if (value instanceof Array) {
        return value.forEach((child_value) => create_definition(context, ref, name, child_value))
      }
      return create_definition(context, ref, name, value)
    })
  } else {
    throw Error('The "define" keyword is reserved and cannot be redefined')
  }
}

function handle_extend(context: $Context, ref: $Reference, extend: $Value) {
  if (extend instanceof Object && !(extend instanceof Array)) {
    return Object.entries(extend).forEach(([name, value]) => {
      if (value instanceof Array) {
        return value.forEach((entry) => create_extend(context, ref, name, entry))
      }
      create_extend(context, ref, name, value)
    })
  }
  assign(context, ref, extend)
}
