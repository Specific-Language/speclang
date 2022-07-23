import { Context } from '.'
import { $Definition, $Primitive, $Object, $Value, $Match } from '../types'

/** lookup
 * takes any JSON value
 * returns any matching definitions
 */
export function lookup(context: Context, value: $Value): Array<$Match> {
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
export function lookup_object(context: Context, value: $Object): Array<$Match> {
  const result = new Array<$Match>()
  Object.keys(value).forEach((key) => {
    context.dictionary.get(key).forEach((definition) => {
      const match = {
        [key]: definition
      }
      array_push_unique(result, match)
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
export function lookup_primitive(context: Context, value: $Primitive): Array<$Match> {
  const result = new Array<$Match>()
  const definitions = context.dictionary.get(typeof value)
  const matches = definitions.map((d) => composeMatch(typeof value, d))
  array_concat_unique(result, matches)
  definitions.forEach((definition) => {
    if (!definition.parent_id) {
      throw Error('lookup_primitive: Expected primitive type to have a parent')
    }
    const parent_definitions = lookup_id(context, definition.parent_id)
    array_concat_unique(result, parent_definitions)
  })
  return result
}

export function lookup_id(context: Context, id: string): Array<$Match> {
  const event = context.events[id]
  if (!event) {
    throw Error('lookup_object: Expected to find event')
  }
  const definitions = context.dictionary.get(event.name)
  const matches = definitions.map((d) => composeMatch(event.name, d))
  return matches
}

function array_push_unique<T>(output: Array<T>, input: T) {
  if (!output.includes(input)) {
    output.push(input)
  }
}

function array_concat_unique<T>(output: Array<T>, input: Array<T>) {
  input.forEach((element) => array_push_unique(output, element))
}

function composeMatch(name: string, definition: $Definition): $Match {
  return {
    [name]: definition
  }
}
