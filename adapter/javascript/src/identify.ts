import { get } from "./dictionary";
import { $Definition, $Dictionary, $Reference, $Value } from "./types";

export function test(dictionary: $Dictionary, test_ref: $Reference, value: $Value): boolean {
  const [name, unique] = test_ref
  const definition = get(dictionary, name, unique)
  if (!definition) {
    throw Error('expected to find definition at reference')
  }
  return test_definition(dictionary, name, definition, value)
}

function test_definition(dictionary: $Dictionary, test_name: string, definition: $Definition, value: $Value): boolean {
  const entries = Object.entries(definition)
  if (entries.length === 0) {
    if (['string', 'number', 'boolean'].includes(test_name)) {
      return test_primitive(test_name, value)
    }
  }
  const extend = definition.extend ?? {}
  const define = definition.define ?? {}
  const extend_result = Object.entries(extend).every((reference) => test(dictionary, reference, value))
  const define_result = Object.entries(define).every((reference) => test_property(dictionary, reference, value))
  return define_result && extend_result
}

function test_property(dictionary: $Dictionary, reference: $Reference, value: $Value): boolean {
  const [define_name, _] = reference
  if (value instanceof Object && !(value instanceof Array)) {
    if (value[define_name] !== undefined) {
      return test(dictionary, reference, value[define_name])
    }
  }
  return false
}

function test_primitive(test_name: string, value: $Value): boolean {
  if (value instanceof Array) {
    return test_name === 'array'
  }
  if (value instanceof Object) {
    return test_name === 'object'
  }
  if (typeof value === test_name) {
    return true
  }
  return false
}
