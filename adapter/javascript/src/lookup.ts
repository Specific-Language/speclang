import type { $Definition, $Primitive, $Object, $Value } from './types'
import type { Context } from './context'

/** lookup
 * takes any JSON value
 * returns any matching definitions
 */
export function lookup(context: Context, value: $Value): Array<$Definition> {
  if (value instanceof Array) {
    throw Error('unhandled case: lookup: array')
  }
  if (value instanceof Object) {
    return lookup_object(context, value)
  }
  return lookup_primitive(context, value)
}

/** lookup_object
 * for each key, look up definitions
 * for each definition, check its parent for a match
 */
export function lookup_object(context: Context, value: $Object): Array<$Definition> {
  const result = new Array<$Definition>()
  Object.keys(value).forEach((key) => {
    context.dictionary.get(key).forEach((definition) => {
      array_push_unique(result, definition)
      if (definition.parent_id) {
        const parent_definitions = lookup_id(context, definition.parent_id)
        // todo: filter to matching definitions
        array_concat_unique(result, parent_definitions)
      }
    })
    const child_definitions = lookup(context, value[key])
    array_concat_unique(result, child_definitions)
  })
  return result
}

/** lookup_primitive
 * non-array, non-object values
 * primitives must have a parent
 */
export function lookup_primitive(context: Context, value: $Primitive): Array<$Definition> {
  const result = new Array<$Definition>()
  const definitions = context.dictionary.get(typeof value)
  array_concat_unique(result, definitions)
  definitions.forEach((definition) => {
    if (!definition.parent_id) {
      throw Error('lookup_primitive: Expected primitive type to have a parent')
    }
    const parent_definitions = lookup_id(context, definition.parent_id)
    array_concat_unique(result, parent_definitions)
  })
  return result
}

export function lookup_id(context: Context, id: string): Array<$Definition> {
  const event = context.events[id]
  if (!event) {
    throw Error('lookup_object: Expected to find event')
  }
  const definitions = context.dictionary.get(event.name)
  return definitions
}

function array_push_unique<T>(output: Array<T>, input: T) {
  if (!output.includes(input)) {
    output.push(input)
  }
}

function array_concat_unique<T>(output: Array<T>, input: Array<T>) {
  input.forEach((element) => array_push_unique(output, element))
}
