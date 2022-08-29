import { set_value, get_ordered_entries, set_alias } from './dictionary'
import type { $Context, $Map, $Value } from './types'

export function specify(context: $Context, parent: string, alias: string, value: $Value): string {
  const reference = `${parent}-${alias}`
  context.option?.verbose && console.log('| specify', { parent_ref: parent, alias, value, reference })
  set_alias(context, reference, alias)
  if (value instanceof Array) {
    value.forEach((sibling) => specify(context, parent, alias, sibling))
  } else if (value instanceof Object) {
      const { extend, ...define } = value
      specify_define(context, reference, define)
      extend && specify_extend(context, reference, extend)
  } else {
    set_value(context, reference, value)
  }
  return reference
}

function specify_define(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_define', { reference, value })
  if (value instanceof Object && !(value instanceof Array)) {
    const value_ref: $Map = {}
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const child_ref = specify(context, reference, child_name, child_value)
      value_ref[child_name] = `\${${child_ref}}`
    })
    set_value(context, reference, value_ref)
  } else {
    set_value(context, reference, value)
  }
}

function specify_extend(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_extend', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify_extend(context, reference, sibling))
  } else if (value instanceof Object) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      // test child_name
      const child_ref = specify(context, reference, child_name, child_value)
      console.log('extending ', child_ref)
      set_value(context, reference, child_ref)
      set_value(context, child_ref, child_name)
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}
