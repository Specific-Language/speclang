import * as speclang from '../../../pkg/speclang'
import type { $Definition, $Specification, $Value } from './types'
import { Context } from './context'

export async function parse(context: Context, raw_input: string): Promise<void> {
  const raw_output = await speclang.parse(raw_input)
  const output: $Specification = JSON.parse(raw_output)
  const document = compose_definition('$', {
    time: new Date().toISOString(),
    raw_input,
    raw_output,
    output,
  })
  specify(context, document, output)
  context.define('$', document)
}

function specify(context: Context, parent: $Definition, spec: $Specification): void {
  const { define, ...constrain } = spec
  if (define) {
    Object.entries(define).forEach(([name, value]) => {
      const definition = compose_definition(name, value, parent)
      parent.define.push(definition.id)
      context.define(name, definition)
      specify(context, definition, compose_spec(value))
    })
    context.options.verbose && console.log('Definitions complete')
  }
  if (constrain) {
    extend(context, parent, constrain)
    context.options.verbose && console.log('Extensions complete')
  }
}

function extend(context: Context, parent: $Definition, constrain: $Value) {
  if (constrain instanceof Array) {
    throw Error('unhandled case: extend: array')
  }
  if (constrain instanceof Object) {
    Object.entries(constrain).forEach(([name, value]) => {
      const definition = compose_definition(name, value, parent)
      parent.extend.push(definition.id)
      context.define(name, definition)
      extend(context, definition, value)
    })
  }
}

function compose_definition(name: string, input: $Value, parent?: $Definition): $Definition {
  const id = `${name}-${String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')}` // todo : this obv isn't enough
  const definition: $Definition = {
    input,
    define: [],
    extend: [],
    id,
  }
  if (parent) {
    definition.parent_id = parent.id
  }
  return definition
}

function compose_spec(value: $Value): $Specification {
  if (value instanceof Array) {
    throw Error('unhandled case: compose_spec: Array')
  }
  if (value instanceof Object) {
    return value
  }
  return {
    [typeof value]: value
  }
}
