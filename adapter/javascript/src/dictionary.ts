import type { $Context, $Reference, $Value } from './types'
import { specify } from './specify'

export function create_reference(name: string): $Reference {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  return [name, unique]
}

export function create_definition(context: $Context, target: $Reference, name: string, value: $Value) {
  const child_ref = create_reference(name)
  add_define(context, child_ref, value, target)
}

export function create_extend(context: $Context, target: $Reference, name: string, value: $Value) {
  const child_ref = create_reference(name)
  add_extend(context, child_ref, value, target)
}

export function add_define(context: $Context, target: $Reference, value: $Value, parent?: $Reference) {
  const [name, unique] = target
  context.define ??= {}
  if (parent) {
    const [parent_name, parent_unique] = parent
    context.define[parent_name] ??= {}
    context.define[parent_name][parent_unique] ??= {}
    context.define[parent_name][parent_unique][name] = unique
    add_parent(context, target, parent)
  }
  context.define[name] ??= {}
  context.define[name][unique] ??= {}
  specify(context, target, value)
}

export function add_extend(context: $Context, target: $Reference, value: $Value, parent: $Reference) {
  const [target_name, target_unique] = target
  context.extend ??= {}
  const [parent_name, parent_unique] = parent
  context.extend[parent_name] ??= {}
  context.extend[parent_name][parent_unique] ??= {}
  context.extend[parent_name][parent_unique][target_name] ??= []
  context.extend[parent_name][parent_unique][target_name].push(target_unique)
  add_parent(context, target, parent)
  context.extend[target_name] ??= {}
  context.extend[target_name][target_unique] ??= {}
  specify(context, target, value)
}

export function add_parent(context: $Context, target: $Reference, value: $Reference) {
  const [name, unique] = value
  const [target_name, target_unique] = target
  context.parent ??= {}
  context.parent[target_name] ??= {}
  context.parent[target_name][target_unique] ??= {}
  context.parent[target_name][target_unique][name] = unique
}

export function assign(context: $Context, [name, unique]: $Reference, value: $Value) {
  context.assign ??= {}
  context.assign[name] ??= {}
  context.assign[name][unique] = value
}

export function slice(context: $Context, [name, unique]: $Reference): $Context {
  return Object.keys(context).reduce<$Context>((result, dictionary) => {
    if (context[dictionary] && context[dictionary][name]) {
      result[dictionary] ??= {}
      result[dictionary][name] ??= {}
      result[dictionary][name][unique] = context[dictionary][name][unique] 
    }
    return result
  }, {})
}
