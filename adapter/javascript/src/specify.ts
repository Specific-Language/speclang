import { set_alias, set_define, set_extend, set_value } from './dictionary'
import { get_ordered_entries } from './reference'
import type { $Context, $Map, $Value } from './types'

export function specify(context: $Context, parent_ref: string, alias: string,  value: $Value): string {
  const reference = compose_reference(alias, parent_ref)
  context.option?.verbose && console.log('| specify', { alias, reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify(context, parent_ref, alias, sibling))
  } else if (value instanceof Object) {
    const { define, extend, ...values } = value
    define && specify_define(context, reference, define)
    extend && specify_extend(context, reference, extend)
    values && set_value(context, reference, values)
  } else {
    set_value(context, reference, value)
    set_alias(context, reference, alias)
  }
  return reference
}

function specify_define(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_define', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify_define(context, reference, sibling))
  } else if (value instanceof Object) {
    const value_ref: $Map = {}
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const child_ref = specify(context, reference, child_name, child_value)
      value_ref[child_name] = child_ref
      set_define(context, child_ref)
      set_alias(context, child_ref, child_name)
    })
    set_value(context, reference, value_ref)
  } else {
    throw Error('The "define" keyword is reserved and cannot be redefined')
  }
}

function specify_extend(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('| specify_extend', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify_extend(context, reference, sibling))
  } else if (value instanceof Object) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      // todo : try matching to alias
      const child_ref = compose_reference(child_name, reference)
      set_extend(context, reference, child_ref)
      set_alias(context, child_ref, child_name)
      set_value(context, child_ref, child_value)
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}

function compose_reference(alias: string, parent_ref: string): string {
  return `${parent_ref}-${alias}`
}
