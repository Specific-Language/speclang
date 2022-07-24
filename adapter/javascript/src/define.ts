import type { $Definition, $Object, $Value } from './types'
import type { Context } from './context'

/** define
 * pushes a new definition to the dictionary w/the ID
 * adds a new event to events table w/the ID
 * recursively defines any children
 */
export function define(context: Context, name: string, value: $Value, metadata: $Object = {}): void {
  const definition = compose$definition(value, metadata)
  if (context.dictionary[name] === undefined) {
    context.dictionary[name] = []
  }
  context.dictionary[name].push(definition)
  context.events[definition.id] = {
    name,
    time: new Date().toISOString(),
  }
  if (value instanceof Array) {
    throw Error('unhandled case: define: array')
  }
  if (value instanceof Object) {
    Object.keys(value).forEach((key) => define(context, key, value[key], {
      parent_id: definition.id
    }))
  }
}

export function compose$definition(value: $Value, metadata: $Object): $Definition {
  const id = Math.random().toPrecision(5).substring(2).padEnd(6, '0') // todo : this obv isn't enough
  const definition: $Definition = {
    value,
    id,
    ...metadata,
  }
  return definition
}
