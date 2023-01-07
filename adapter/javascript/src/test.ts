import { get_value } from "./shared"
import { $Context, $Value } from "./types"

export const PRIMITIVES: string[] = [
  'number',
  'string',
  'boolean',
]

export function test(context: $Context, reference: string, value: $Value): boolean {
  const definition = get_value(context, reference)
  context.option?.verbose && console.log(' * test', { reference, value, definition })
  if (definition === undefined) {
    throw Error(`Expected to find a definition for ${reference}`)
  }
  const result = definition.every((extend_value) => test_extend(context, reference, extend_value, value))
  if (result === false) {
    throw Error(`Failed test for ${reference}`)
  }
  return true
}

// no
function test_extend(context: $Context, reference: string, extend: $Value, value: $Value): boolean {
  context.option?.verbose && console.log(' * test_extend', { reference, extend, value })
  if (PRIMITIVES.includes(reference)) {
    return test_primitive(context, reference, extend, value)
  }
  // if (value instanceof Object && !(value instanceof Array)) {
  //   return Object.entries(value).every(([child_name, child_value]) => {
  //     if (extend instanceof Object && !(extend instanceof Array)) {
  //       return test_extend(context, `${reference}-${child_name}`, extend[child_name], child_value)
  //     }
  //     throw Error('egads')
  //   })
  // }
  // if (extend instanceof Object && !(extend instanceof Array)) {
  //   if (Object.entries(extend).length === 0) {
  //     return true
  //   }
  // }
  // if (typeof extend === 'string') {
  //   if (extend.startsWith('${') && extend.endsWith('}')) {
  //     const child_ref = extend.substring(2, extend.length - 1)
  //     return test(context, child_ref, value)
  //   }
  // }
  throw Error('not sposed to get this far')
}

function test_primitive(context: $Context, reference: string, extend: $Value, value: $Value): boolean {
  context.option?.verbose && console.log(' * test_primitive', { reference, extend, value })
  if (value instanceof Object && !(value instanceof Array)) {
    if (Object.entries(value).length === 0) {
      return true
    } //??
  }
  if (extend instanceof Object && !(extend instanceof Array)) {
    if (Object.entries(extend).length === 0) {
      return true
    } //??
  }
  const typed_value = cast(reference, value)
  return JSON.stringify(typed_value) === JSON.stringify(value)
}

function cast(reference: string, value: $Value): $Value {
  switch (reference) {
    case 'number':
      return value as number
    case 'boolean':
      return value as boolean
    case 'string':
      return value as string
    default:
      throw Error('expected to cast a primitive reference')
  }
}
