import { set_value, ascending_pairs, wrap_reference } from './shared'
import { test } from './test'
import type { $Context, $Map, $Value } from './types'

// need to simplify -- even if less dense
// less dense is better
// density is only good when objective
// this code is my subjective attempt at the truth

export function specify(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('specify', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify(context, reference, sibling))
  } else if (value instanceof Object) {
    const { extend: constraints, ...values } = value
    define(context, reference, values)
    constraints && extend(context, reference, constraints)
  } else {
    set_value(context, reference, value)
  }
}

export function define(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('define', { reference, value })
  if (value instanceof Object && !(value instanceof Array)) {
    const value_ref: $Map = {}
    ascending_pairs(value).forEach(([child_name, child_value]) => {
      const child_ref = `${reference}-${child_name}`
      specify(context, child_ref, child_value)
      value_ref[child_name] = wrap_reference(child_ref)
    })                                                                      
    set_value(context, reference, value_ref)
  } else {
    set_value(context, reference, value)
  }
}

export function extend(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('extend', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => extend(context, reference, sibling))
  } else if (value instanceof Object) {
    ascending_pairs(value).forEach(([child_name, child_value]) => {
      const child_ref = `${reference}-${child_name}`
      test(context, child_name, child_value)
      specify(context, child_ref, child_value)
      set_value(context, child_ref, wrap_reference(child_name))
      set_value(context, reference, wrap_reference(child_ref))
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}
