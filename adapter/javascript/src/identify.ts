import { get } from "./dictionary";
import { $Dictionary, $Reference, $Value } from "./types";

export function test(dictionary: $Dictionary, test_ref: $Reference, value: $Value): boolean {
  const { 
    extend = {}, 
    define = {}
  } = get(dictionary, test_ref)
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
  const [extend_name, _] = reference
  if (extend_name === 'number') {
    if (typeof value !== 'number') {
      return false
    }
  }
  return true
}
