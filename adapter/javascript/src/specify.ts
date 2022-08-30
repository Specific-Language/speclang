import { PRIMITIVES } from './constants'
import { set_value, get_ordered_entries, get_value } from './dictionary'
import type { $Context, $Map, $Value } from './types'

export function specify(context: $Context, parent: string, alias: string, value: $Value): string {
  const reference = `${parent}-${alias}`
  context.option?.verbose && console.log('specify', { parent_ref: parent, alias, value, reference })
  if (value instanceof Array) {
    value.forEach((sibling) => specify(context, parent, alias, sibling))
  } else if (value instanceof Object) {
      const { extend, ...define } = value
      define && specify_define(context, reference, define)
      extend && specify_extend(context, reference, extend)
  } else {
    set_value(context, reference, value)
  }
  return reference
}

function specify_define(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('- specify_define', { reference, value })
  if (value instanceof Object && !(value instanceof Array)) {
    const value_ref: $Map = {}
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const child_ref = specify(context, reference, child_name, child_value)
      value_ref[child_name] = wrap_reference(child_ref)
    })
    set_value(context, reference, value_ref)
  } else {
    set_value(context, reference, value)
  }
}

function specify_extend(context: $Context, reference: string, value: $Value) {
  context.option?.verbose && console.log('- specify_extend', { reference, value })
  if (value instanceof Array) {
    value.forEach((sibling) => specify_extend(context, reference, sibling))
  } else if (value instanceof Object) {
    get_ordered_entries(value).forEach(([child_name, child_value]) => {
      const extend_ref = PRIMITIVES.includes(child_name)
        ? test_primitive(context, child_name, child_value)
        : test_spec(context, `$-input-${child_name}`, child_value)
      const child_ref = specify(context, reference, child_name, child_value)
      set_value(context, reference, wrap_reference(child_ref))
      set_value(context, child_ref, wrap_reference(extend_ref))
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}

function test_primitive(context: $Context, name: string, value: $Value): string {
  context.option?.verbose && console.log(' - test_primitive', { name, value })
  const reference = `$-${name}`
  const type = value instanceof Array ? 'list' : typeof value
  if (type === name) {
    return reference
  }
  if (type === 'object') {
    if (Object.entries(value).length === 0) {
      return reference
    }
    return test_spec(context, `$-${name}`, value)
  }
  throw Error(`Expected value to match primitive specification for ${name}`)
}

function test_spec(context: $Context, reference: string, value: $Value): string {
  context.option?.verbose && console.log(' - test_spec', { reference, value })
  const existing_value = get_value(context, reference)
  if (existing_value) {
    console.log({ reference, existing_value, value })
    console.log('@@@ need to test existing complex spec for match')
  } else {
    throw Error(`Expected to find a definition for ${reference} within this document`)
  }
  return reference
}

function wrap_reference(reference: string): string {
  return `\${${reference}}`
}
