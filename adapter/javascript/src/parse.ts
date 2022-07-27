import * as speclang from '../../../pkg/speclang'
import type { $Definition, $Object, $Value } from './types'
import { Context } from './context'

export async function parse(context: Context, input: string, options?: $Object): Promise<$Object> {
  const parserOutput = await speclang.parse(input)
  const specification: $Object = JSON.parse(parserOutput)
  Object.entries(specification).forEach(([key, value]) => define(context, key, value))
  if (options && options.verbose) {
    console.log('Successful parse', specification)
  }
  return specification
}

/** define
 * pushes a new definition to the dictionary w/the ID
 * recursively defines any children
 */
function define(context: Context, name: string, value: $Value, parent?: $Definition): void {
  const id = `${name}-${String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')}` // todo : this obv isn't enough
  const definition: $Definition = {
    value,
    id,
  }
  if (parent) {
    definition.parent_id = parent.id
  }
  context.define(name, definition)
  // handle array
  if (value instanceof Array) {
    throw Error('unhandled case: define: array')
  }
  // handle object
  if (value instanceof Object) {
    return Object.keys(value).forEach((key) => define(context, key, value[key], definition))
  }
  // handle primitive
  context.define(typeof value, definition)
}
