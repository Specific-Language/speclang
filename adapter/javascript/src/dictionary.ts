import type { $Context, $Value } from './types'

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

export function set_alias(context: $Context, reference: string, alias: string) {
  context.alias ??= {}
  context.alias[alias] ??= []
  context.alias[alias].push(reference)
}

export function set_define(context: $Context, reference: string) {
  context.define ??= {}
  context.define[reference] ??= []
}

export function set_extend(context: $Context, reference: string, extend_ref: string) {
  context.define ??= {}
  context.define[reference] ??= []
  context.define[reference].push(extend_ref)
}

export function set_value(context: $Context, reference: string, value: $Value) {
  if (JSON.stringify(value) === '{}') {
    return
  }
  if (['string', 'number', 'boolean'].includes(reference)) {
    if (typeof value !== reference) {
      throw Error('Literal values (string, number, boolean) are constrained by type')
    }
  }
  context.define ??= {}
  context.define[reference] ??= []
  context.define[reference].push(value)
}
