import type { $Primitive, $Object, $Value, $DefinitionTable } from './types'
import type { Context } from './context'

/** lookup
 * takes any JSON value
 * returns any matching definitions
 */
export function lookup(context: Context, value: $Value): $DefinitionTable {
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
export function lookup_object(context: Context, value: $Object): $DefinitionTable {
  const result: $DefinitionTable = {}
  Object.keys(value).forEach((key) => {
    const defs = context.dictionary[key]
    if (defs) {
      result[key] = array_join(result[key], defs)
      defs.forEach((def) => {
        if (def.parent_id) {
          const parent_definitions = lookup_id(context, def.parent_id)
          // todo: filter to matching definitions
          Object.keys(parent_definitions).forEach((key) => {
            result[key] = array_join(result[key], parent_definitions[key])
          })
        }
      })
    }
    const child_definitions = lookup(context, value[key])
    Object.keys(child_definitions).forEach((key) => {
      result[key] = array_join(result[key], child_definitions[key])
    })
  })
  return result
}

/** lookup_primitive
 * non-array, non-object values
 * primitives must have a parent
 */
export function lookup_primitive(context: Context, value: $Primitive): $DefinitionTable {
  const result: $DefinitionTable = {}
  const definitions = context.dictionary[typeof value]
  if (definitions) {
    result[typeof value] = array_join(result[typeof value], definitions)
    definitions.forEach((definition) => {
      if (!definition.parent_id) {
        throw Error('lookup_primitive: Expected primitive type to have a parent')
      }
      const parent_definitions = lookup_id(context, definition.parent_id)
      Object.keys(parent_definitions).forEach((key) => {
        result[key] = array_join(result[key], parent_definitions[key])
      })
    })
  }
  return result
}

/** lookup_id
 * return specific definition slice
 */
export function lookup_id(context: Context, id: string): $DefinitionTable {
  const event = context.events[id]
  if (!event) {
    throw Error('lookup_object: Expected to find event')
  }
  return {
    [event.name]: context.dictionary[event.name]
  }
}

// function reduce(table: $DefinitionTable, condition: Function<bool>): $DefinitionTable {

// }

function array_unique_push<T>(output: Array<T>, input: T) {
  if (!output.includes(input)) {
    output.push(input)
  }
}

function array_unique_concat<T>(output: Array<T>, input: Array<T>) {
  input.forEach((element) => array_unique_push(output, element))
}

function array_join<T>(array1: Array<T>, array2: Array<T>): Array<T> {
  if (!array1) {
    array1 = []
  }
  if (!array2) {
    array2 = []
  }
  array_unique_concat(array1, array2)
  return array1
}
