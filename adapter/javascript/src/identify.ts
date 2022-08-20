import type { $Context, $Dictionary, $Reference, $ReferenceList, $ReferenceMap, $Value } from './types';
import { slice } from './dictionary';

export function test(context: $Context, ref: $Reference, value: $Value): boolean {
  const { define, extend, assign } = slice(context, ref)
  const assign_result = assign && test_assign(assign, ref, value)
  if (assign_result === false) {
    return false
  }
  const define_result = define && test_define(context, define, ref, value)
  if (define_result === false) {
    return false
  }
  const extend_result = extend && test_extend(context, extend, ref, value)
  if (extend_result === false) {
    return false
  }
  return test_primitive(ref, value)
}

function test_assign(assign: $Dictionary<$Value>, [name, unique]: $Reference, value: $Value): boolean {
  return value !== assign[name][unique]
}

function test_define(context: $Context, define: $Dictionary<$ReferenceMap>, [name, unique]: $Reference, value: $Value): boolean {
  return Object.entries(define[name][unique]).every((child_ref) => {
    const [child_name, _] = child_ref
    if (value instanceof Object && !(value instanceof Array)) {
      if (value[child_name] !== undefined) {
        return test(context, child_ref, value[child_name])
      }
    }
    return false
  })
}

function test_extend(context: $Context, extend: $Dictionary<$ReferenceList>, [name, unique]: $Reference, value: $Value): boolean {
  return Object.entries(extend[name][unique]).every(([child_name, child_refs]) => {
    return child_refs.every((child_unique) => {
      const child_ref: $Reference = [child_name, child_unique]
      return test(context, child_ref, value)
    })
  })
}

function test_primitive([name, _]: $Reference, value: $Value): boolean {
  const PRIMITIVES = [
    'string', 
    'number', 
    'boolean',
  ]
  if (PRIMITIVES.includes(name)) {
    return typeof value === name
  }
  return true
}
