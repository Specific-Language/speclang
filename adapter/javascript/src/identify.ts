import { get } from "./dictionary";
import { $Specification, $Dictionary, $Reference, $Value } from "./types";

export const PRIMITIVES = [
  'string', 
  'number', 
  'boolean'
]

export function test(dictionary: $Dictionary, test_ref: $Reference, value: $Value): boolean {
  const [name, unique] = test_ref
  const spec = get(dictionary, name, unique)
  if (!spec) {
    throw Error('expected to find spec at reference')
  }
  return test_spec(dictionary, name, spec, value)
}

function test_spec(dictionary: $Dictionary, name: string, spec: $Specification, value: $Value): boolean {
  const entries = Object.entries(spec)
  if (PRIMITIVES.includes(name) && entries.length === 0) {
    return test_primitive(name, value)
  }
  const extend = spec.extend ?? {}
  const define = spec.define ?? {}
  const extend_result = Object.entries(extend).every((reference) => test(dictionary, reference, value))
  const define_result = Object.entries(define).every((reference) => test_definition(dictionary, reference, value))
  return define_result && extend_result
}

function test_definition(dictionary: $Dictionary, reference: $Reference, value: $Value): boolean {
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
  if (typeof value === test_name) {
    return true
  }
  return false
}
