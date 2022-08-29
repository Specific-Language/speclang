import { define_value, get_ordered_entries } from './dictionary'
import type { $Context, $Map, $Value } from './types'

export function specify(context: $Context, parent_ref: string, alias: string, value: $Value): string {
  const reference = `${parent_ref}-${alias}`
  context.option?.verbose && console.log('| specify', { parent_ref, alias, value, reference })
  if (value instanceof Array) {
    value.forEach((sibling) => specify(context, parent_ref, alias, sibling))
  } else if (value instanceof Object) {
      const { extend, ...define } = value
      specify_define(context, reference, define)
      extend && specify_extend(context, reference, extend)
  } else {
    define_value(context, reference, value)
  }
  return reference
}

function specify_define(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_define', { reference, value })
  if (value instanceof Object && !(value instanceof Array)) {
    const value_ref: $Map = {}
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const child_ref = specify(context, reference, child_name, child_value)
      value_ref[child_name] = child_ref
    })
    define_value(context, reference, value_ref)
  } else {
    define_value(context, reference, value)
  }
}

function specify_extend(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_extend', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify_extend(context, reference, sibling))
  } else if (value instanceof Object) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const child_ref = specify(context, reference, child_name, child_value)
      define_value(context, reference, child_ref)
      define_value(context, child_ref, child_name)
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}
