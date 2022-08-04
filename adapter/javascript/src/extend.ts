import { Context } from "./context"
import { Definition } from "./define"
import { $Definition, $Value } from "./types"

export function extend_value(context: Context, target: $Definition, input: $Value): void {
  if (input instanceof Array) {
    throw Error('unhandled case: extend_value: array')
  }
  if (input instanceof Object) {
    return Object.entries(input).forEach(([name, value]) => extend_pair(context, target, name, value))
  }
  context.extend(target, typeof input, input)
}

export function extend_pair(context: Context, target: $Definition, name: string, value: $Value) {
  if (value instanceof Array) {
    throw Error('unhandled case: extend_pair: array')
  }
  if (value instanceof Object) {
    const definition = Definition(name, target)
    context.define(name, definition)
    context.extend(target, name, definition.id)
    return context.specify(definition, value)
  }
  context.extend(target, name, value)
}
