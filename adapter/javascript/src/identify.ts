import { get } from "./dictionary";
import { $Definition, $Dictionary, $Reference, $Value } from "./types";

export function test(dictionary: $Dictionary, test_ref: $Reference, value: $Value): boolean {
  // todo : some kind of fallthrough chain
  const definition = get(dictionary, test_ref)
  if (!definition) {
    return false
  }
  const [test_name, _] = test_ref
  if (definition instanceof Object && Object.entries(definition).length === 0) {
    if (['string', 'number', 'boolean'].includes(test_name)) {
      return test_primitive(test_name, value)
    }
  }
    return test_spec(dictionary, definition, value)
}

function test_primitive(name: string, value: $Value): boolean {
  if (typeof value === name) {
    return true
  }
  return false
}

function test_spec(dictionary: $Dictionary, definition: $Definition, value: $Value): boolean {
  const { 
    extend = {}, 
    define = {} 
  } = definition
  const define_result = Object.entries(define).every((define_ref) => test_define(dictionary, define_ref, value))
  const extend_result = Object.entries(extend).every((extend_ref) => test_extend(dictionary, extend_ref, value))
  return define_result && extend_result
}

function test_define(dictionary: $Dictionary, reference: $Reference, value: $Value): boolean {
  const [define_name, _] = reference
  if (value instanceof Object && !(value instanceof Array)) {
    if (value[define_name] === undefined) {
      return false
    }
    return test(dictionary, reference, value)
  }
  return false
}

function test_extend(dictionary: $Dictionary, reference: $Reference, value: $Value): boolean {
  dictionary
  reference
  value
  throw Error('unimplemented')
}
