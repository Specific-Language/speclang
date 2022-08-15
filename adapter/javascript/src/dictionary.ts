import type { $Reference, $ReferenceMap, $Value } from './types'
import type { $Context } from './context'

export type $Dictionary<T extends $Value> = {
  [name: string]: {
    [unique: string]: T
  },
}

export function slice(context: $Context, reference: $Reference) {
  const define = get_value(context, 'define', reference)
  const extend = get_value(context, 'extend', reference)
  const relate = get_value(context, 'relate', reference)
  const assign = get_value(context, 'assign', reference)
  return {
    define,
    extend,
    relate,
    assign,
  }
}

export function define(context: $Context, target: $Reference, value: $Reference) {
  context.define ??= {}
  return set_reference(context.define, target, value)
}

export function extend(context: $Context, target: $Reference, value: $Reference) {
  context.extend ??= {}
  return set_reference(context.extend, target, value)
}

export function relate(context: $Context, target: $Reference, value: $Reference) {
  context.relate ??= {}
  return set_reference(context.relate, target, value)
}

export function assign(context: $Context, target: $Reference, value: $Value) {
  context.assign ??= {}
  return set_value(context.assign, target, value)
}

function set_reference(dictionary: $Dictionary<$ReferenceMap>, target: $Reference, value: $Reference) {
  const [name, unique] = value
  const [target_name, target_unique] = target
  dictionary[target_name] ??= {}
  dictionary[target_name][target_unique] ??= {}
  dictionary[target_name][target_unique][name] = unique
}

function set_value(dictionary: $Dictionary<$Value>, [target_name, target_unique]: $Reference, value: $Value) {
  dictionary[target_name] ??= {}
  dictionary[target_name][target_unique] = value
}

function get_value(context: $Context, partition: string, [name, unique]: $Reference): $Value | undefined {
  if (context[partition] && context[partition][name]) {
    return context[partition][name][unique] 
  }
  return undefined
}
