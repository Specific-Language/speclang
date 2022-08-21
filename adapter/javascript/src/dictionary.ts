import type { $Context, $Reference, $Value } from './types'

export function lookup(context: $Context, name: string): $Context {
  const result: $Context = {}
  Object.keys(context).forEach((dictionary) => {
    if (context[dictionary][name] !== undefined) {
      result[dictionary] ??= {}
      result[dictionary][name] = context[dictionary][name]
    }
  })
  return result
}

export function slice(context: $Context, [name, unique]: $Reference): $Context {
  const result: $Context = {}
  const name_context = lookup(context, name)
  Object.keys(name_context).forEach((dictionary) => {
    if (context[dictionary][name] && context[dictionary][name][unique] !== undefined) {
      result[dictionary] ??= {}
      result[dictionary][name] ??= {}
      result[dictionary][name][unique] = context[dictionary][name][unique] 
    }
  })
  return result
}

export function set_define(context: $Context, target: $Reference, parent?: $Reference) {
  context.define ??= {}
  const [name, unique] = target
  if (parent) {
    const [parent_name, parent_unique] = parent
    context.define[parent_name] ??= {}
    context.define[parent_name][parent_unique] ??= {}
    context.define[parent_name][parent_unique][name] = unique
  }
  context.define[name] ??= {}
  context.define[name][unique] ??= {}
}

export function set_extend(context: $Context, target: $Reference, parent: $Reference) {
  context.extend ??= {}
  const [parent_name, parent_unique] = parent
  context.extend[parent_name] ??= {}
  context.extend[parent_name][parent_unique] ??= {}
  const [target_name, target_unique] = target
  context.extend[parent_name][parent_unique][target_name] ??= []
  context.extend[parent_name][parent_unique][target_name].push(target_unique)
}

export function set_value(context: $Context, target: $Reference, value: $Value, parent?: $Reference) {
  const [name, unique] = target
  if (['string', 'number', 'boolean'].includes(name)) {
    if (typeof value !== name) {
      throw Error('Literal values (string, number, boolean) are constrained by type')
    }
  }
  context.values ??= {}
  context.values[name] ??= {}
  context.values[name][unique] = value
  context.assign ??= {}
  if (parent) {
    const [parent_name, parent_unique] = parent
    context.assign[parent_name] ??= {}
    context.assign[parent_name][parent_unique] ??= {}
    context.assign[parent_name][parent_unique][name] = unique
  }
}
