import { specify } from './specify'
import type { $Context, $Reference, $Value } from './types'

export function create_reference(context: $Context, name: string, value: $Value): $Reference {
  // const { define } = get_slice(context, name)
  // if (define !== undefined) {
  //   // todo : handle multiple matches
  //   const match = Object.keys(define[name]).find((unique) => test(context, [name, unique], value))
  //   if (match) {
  //     // return [name, match]
  //   }
  // }
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  const reference: $Reference = [name, unique]
  specify(context, reference, value)
  return reference
}

export function get_slice(context: $Context, name: string, unique?: string): $Context {
  const result: $Context = {}
  Object.keys(context).forEach((dictionary) => {
    if (unique !== undefined) {
      if (context[dictionary][name] && context[dictionary][name][unique] !== undefined) {
        result[dictionary] ??= {}
        result[dictionary][name] ??= {}
        result[dictionary][name][unique] = context[dictionary][name][unique] 
      }
    } else {
      if (context[dictionary][name] !== undefined) {
        result[dictionary] ??= {}
        result[dictionary][name] = context[dictionary][name]
      }
    }
  })
  return result
}

export function apply_define(context: $Context, target: $Reference, parent?: $Reference) {
  context.define ??= {}
  const [name, unique] = target
  if (parent) {
    const [parent_name, parent_unique] = parent
    context.define[parent_name] ??= {}
    context.define[parent_name][parent_unique] ??= {}
    context.define[parent_name][parent_unique][name] = unique
    apply_parent(context, target, parent)
  }
  context.define[name] ??= {}
  context.define[name][unique] ??= {}
}

export function apply_extend(context: $Context, target: $Reference, parent: $Reference) {
  context.extend ??= {}
  const [target_name, target_unique] = target
  const [parent_name, parent_unique] = parent
  context.extend[parent_name] ??= {}
  context.extend[parent_name][parent_unique] ??= {}
  context.extend[parent_name][parent_unique][target_name] ??= []
  context.extend[parent_name][parent_unique][target_name].push(target_unique)
  apply_parent(context, target, parent)
  context.extend[target_name] ??= {}
  context.extend[target_name][target_unique] ??= {}
}

export function apply_parent(context: $Context, target: $Reference, value: $Reference) {
  context.parent ??= {}
  const [name, unique] = value
  const [target_name, target_unique] = target
  context.parent[target_name] ??= {}
  context.parent[target_name][target_unique] ??= {}
  context.parent[target_name][target_unique][name] = unique
}

export function apply_value(context: $Context, [name, unique]: $Reference, value: $Value) {
  context.values ??= {}
  context.values[name] ??= {}
  context.values[name][unique] = value
}
