import type { $Primitive, $Reference, $ReferenceMap, $Value } from './types';
import type { $Context } from './context';
import { slice } from './dictionary';
import { handle_value } from './functions';

export const PRIMITIVES = [
  'string', 
  'number', 
  'boolean',
  'array'
]

export function test(context: $Context, ref: $Reference, value: $Value): boolean {
  const define_result = test_define(context, ref, value)
  const extend_result = test_extend(context, ref, value)
  return define_result && extend_result 
}

// doesn't entirely work for nested extend/assign

function test_define(context: $Context, ref: $Reference, value: $Value): boolean {
  const { define } = slice(context, ref)
  if (define === undefined) {
    return false
  }
  const test_name = ref[0]
  if (PRIMITIVES.includes(test_name) && Object.entries(define).length === 0) {
    return test_primitive(test_name, value)
  }
  return handle_value<$ReferenceMap, $Primitive, boolean>(define,
    (spec) => Object.entries(spec).every((child_ref) => test_property(context, child_ref, value)),
    (spec) => spec === value
  )
}

function test_extend(context: $Context, ref: $Reference, value: $Value): boolean {
  const { extend } = slice(context, ref)
  if (extend === undefined) {
    return true
  }
  return Object.entries(extend).every((child_ref) => test(context, child_ref, value))
}

function test_property(context: $Context, child_ref: $Reference, value: $Value): boolean {
  const name = child_ref[0]
  if (value instanceof Object && !(value instanceof Array)) {
    if (value[name] !== undefined) {
      return test(context, child_ref, value[name])
    }
  }
  return false
}

function test_primitive(type: string, value: $Value): boolean {
  if (value instanceof Array) { 
    return type === 'array'
  }
  if (typeof value === type) {
    return true
  }
  return false
}
