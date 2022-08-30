import { PRIMITIVES, PRIMITIVE_SPECS } from './constants'
import { set_value, get_ordered_entries, set_alias, get_value } from './dictionary'
import type { $Context, $Map, $Value } from './types'

export function specify(context: $Context, parent: string, alias: string, value: $Value): string {
  const reference = `${parent}-${alias}`
  context.option?.verbose && console.log('specify', { parent_ref: parent, alias, value, reference })
  set_alias(context, reference, alias)
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
      const extend_ref = get_extend_reference(context, child_name, child_value)
      const child_ref = specify(context, reference, child_name, child_value)
      set_value(context, reference, wrap_reference(child_ref))
      set_value(context, child_ref, wrap_reference(extend_ref))
    })
  } else {
    throw Error('The "extend" keyword is reserved and cannot be redefined')
  }
}

function get_extend_reference(context: $Context, name: string, value: $Value): string {
  if (PRIMITIVES.includes(name)) {
    if (typeof value !== name) {
      if (value instanceof Object) {
        const primitive_spec = PRIMITIVE_SPECS[name]
        console.log('@@@ need to test primitive spec', { name, value, primitive_spec })
      } else {
        throw Error(`Value did not match assigned primitive type: ${name}`)
      }
    }
    return name
  }
  const existing_ref = `$-input-${name}`
  const existing_value = get_value(context, existing_ref)
  if (existing_value) {
    console.log('@@@ need to test existing complex spec for match')
  } else {
    throw Error(`Expected to find a definition for ${name} within this document`)
  }
  return existing_ref
}

function wrap_reference(reference: string): string {
  return `\${${reference}}`
}
