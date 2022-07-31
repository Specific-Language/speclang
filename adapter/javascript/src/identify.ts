import type { $Primitive, $Object, $Value, $Dictionary, $Definition } from './types'
import type { Context } from './context'
import { array_safe_join, dictionary_safe_join } from './utility'

/** identify
 * takes any JSON value
 * returns any matching definitions
 */
export function identify(context: Context, value: $Value): $Dictionary {
  if (value instanceof Array) {
    throw Error('unhandled case: lookup: array')
  }
  if (value instanceof Object) {
    return identify_object(context, value)
  }
  return identify_primitive(context, value)
}

/** identify_object
 * for each key, lookup definitions
 * for each definition, check its parent for a match
 */
export function identify_object(context: Context, value: $Object): $Dictionary {
  let result: $Dictionary = {}
  Object.keys(value).forEach((key) => {
    const child_definitions = lookup_name(context, key)
    result[key] = array_safe_join(result[key], child_definitions)
    const child_lookup = identify(context, value[key])
    result = dictionary_safe_join(result, child_lookup)
  })
  return result
}

/** identify_primitive
 * non-array, non-object values
 * primitives must have a parent
 */
export function identify_primitive(context: Context, value: $Primitive): $Dictionary {
  const result: $Dictionary = {}
  if (typeof value === 'string') {
    const value_definitions = lookup_name(context, value)
    result[value] = array_safe_join(result[value], value_definitions)
  }
  const type_definitions = lookup_name(context, typeof value)
  result[typeof value] = array_safe_join(result[typeof value], type_definitions)
  return result
}

function lookup_name(context: Context, name: string): $Definition[] | undefined {
  let definitions = context.lookupName(name)
  if (!definitions) {
    return
  }
  definitions.forEach((d) => {
    if (!d.parent_id) {
      return
    }
    // todo: this is bad lol
    const parent_split = d.parent_id.split('-')
    const parent_name = parent_split[0]
    const parent_id = parent_split[1]
    const parent = context.lookupID(parent_name, parent_id)
    definitions = array_safe_join(definitions, parent)
  })
  return definitions
}
