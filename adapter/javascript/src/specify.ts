import * as speclang from '../../../pkg/speclang'
import type { $Definition, $Object, $Specification, $Value } from './types'
import { Context } from './context'
import { Definition, Specification } from './compose'

export async function parse(context: Context, input: string): Promise<void> {
  const raw_output = await speclang.parse(input)
  const output: $Specification = JSON.parse(raw_output)
  context.options.verbose && console.log('Parsed HCL2 input as JSON')
  const document = Definition('$')
  // context.define('$', document)
  specify(context, document, output)
  context.options.verbose && console.log('Successfully understood Specific Language\n')
}

function specify(context: Context, parent: $Definition, input: $Specification): void {
  const { 
    define: definitions,
    ...rest
  } = input
  definitions && define(context, parent, definitions)
  rest && extend(context, parent, rest)
}

function define(context: Context, parent: $Definition, input: $Specification) {
  Object.entries(input).forEach(([name, value]) => {
    const definition = Definition(name, parent)
    context.define(name, definition, parent)
    const spec = Specification(value)
    specify(context, definition, spec)
    context.options.verbose && console.log(`Defined "${name}" on ${parent.id}`)
  })
}

function extend(context: Context, parent: $Definition, input: $Value): void {
  if (input instanceof Array) {
    throw Error('unhandled case: extend: array')
  }
  if (input instanceof Object) {
    return Object.entries(input).forEach(([name, value]) => {
      if (value instanceof Array) {
        throw Error('unhandled case: extend: array')
      }
      if (value instanceof Object) {
        return extend_object(context, parent, name, value)
      }
      parent.extend[name] = value
      context.options.verbose && console.log(`Extended "${name}" on ${parent.id}`)
    })
  }
  const name = typeof input
  parent.extend[name] = input
  context.options.verbose && console.log(`Extended "${name}" on ${parent.id}`)
}

function extend_object(context: Context, parent: $Definition, name: string, value: $Object): void {
  const definition = Definition(name, parent)
  context.define(name, definition)
  parent.extend[name] = definition.id
  context.options.verbose && console.log(`Extended "${name}" on ${parent.id}`)
  extend(context, definition, value)
}
