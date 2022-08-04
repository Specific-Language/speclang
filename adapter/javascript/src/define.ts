import { Context, Specification } from "./context"
import { $Definition, $Value } from "./types"

export function Definition(name: string, parent?: $Definition): $Definition {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  const definition: $Definition = {
    id: [name, unique]
  }
  if (parent) {
    definition.parent = parent.id
  }
  return definition
}

export function define_value(context: Context, target: $Definition, input: $Value) {
  if (input instanceof Array) {
    throw Error('unhandled case: define_value: array')
  }
  if (input instanceof Object) {
    return Object.entries(input).forEach(([name, value]) => define_pair(context, target, name, value))
  }
  define_pair(context, target, typeof input, input)
}

export function define_pair(context: Context, target: $Definition, name: string, value: $Value) {
  const definition = Definition(name, target)
  context.define(name, definition, target)
  const spec = Specification(value)
  context.specify(definition, spec)
}
