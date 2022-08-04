import { Context } from "./context";
import { $Definition, $Dictionary, $Value } from "./types";

export function identify(context: Context, input: $Value): $Dictionary {
  console.log(`identifying ${input}`)
  const result: $Dictionary = {}
  if (input instanceof Array) {
    throw Error('unhandled: identify: array')
  }
  if (input instanceof Object) {
    // Object.entries(input).forEach(([name, value]) => {

    // })
    throw Error('unhandled: identify: object')
  }
  context.lookup(typeof input).forEach((definition) => match(context, result, definition, input))
  return result
}

export function match(context: Context, result: $Dictionary, definition: $Definition, input: $Value): void {
  const [name, _] = definition.id
  if (test(context, definition, input)) {
    if (!result[name]) {
      result[name] = []
    }
    result[name].push(definition)
  }
  if (definition.parent) {
    const [parent_name, parent_id] = definition.parent
    const parent_definition = context.get(parent_name, parent_id)
    match(context, result, parent_definition, input)
  }
}

// todo : flesh this out and results array / shim method for basic T/F
export function test(context: Context, definition: $Definition, input: $Value): boolean {
  console.log('testing', definition, input)
  context
  
  const result: boolean[] = []
  const { extend, define } = definition
  extend && Object.entries(extend).forEach(([name, _]) => testExtend(result, name, input))
  define && Object.entries(define).forEach(([name, _]) => testDefine(result, name, input))
  return result.every((value) => value === true)
}

function testExtend(result: boolean[], name: string, input: $Value) {
  if (input instanceof Array) {
    if (name === 'array') {
      result.push(true)
    }
    throw Error('unexpected array')
  }
  if (input instanceof Object) {
    if (name === 'object') {
      result.push(true)
    }
    throw Error('unexpected object')
  }
  if (name === 'primitive' 
    || name === typeof input) {
    result.push(true)
    return
  }
  result.push(false)
  // todo : verify the child property value?
}

function testDefine(result: boolean[], name: string, input: $Value) {
  if (input instanceof Array) {
    throw Error('unexpected array')
  }
  if (input instanceof Object) {
    if (input[name] === undefined) {
      result.push(false)
    }
    // todo : verify the child property value?
    result.push(true)
  }
  if (!(input instanceof Object)) {
    result.push(false)
  }
}
