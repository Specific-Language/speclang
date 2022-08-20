import type { $Context, $Dictionary, $Reference, $ReferenceList, $ReferenceMap, $Value } from './types';
import { get_slice } from './dictionary';

export function test(context: $Context, ref: $Reference, value: $Value): boolean {
  console.log('test', ref, value)
  const slice = get_slice(context, ...ref)
  if (Object.entries(slice).length === 0) {
    throw Error(`No context exists for reference [${ref.join('-')}]`)
  }
  const { define, extend, values } = slice
  const define_result = define && test_define(context, define, ref, value)
  if (define_result === false) {
    console.log(' * failed test_define')
    return false
  }
  const extend_result = extend && test_extend(context, extend, ref, value)
  if (extend_result === false) {
    console.log(' * failed test_extend')
    return false
  }
  const values_result = values && test_values(values, ref, value)
  if (values_result === false) {
    console.log(' * failed test_values')
    return false
  }
  return test_primitive(ref, value)
}

function test_values(values: $Dictionary<$Value>, [name, unique]: $Reference, value: $Value): boolean {
  console.log('| test_values', name, unique, value)
  return value === values[name][unique]
}

function test_define(context: $Context, define: $Dictionary<$ReferenceMap>, [name, unique]: $Reference, value: $Value): boolean {
  console.log('| test_define', name, unique, value)
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
  console.log('| test_extend', name, unique, value)
  return Object.entries(extend[name][unique]).every(([child_name, child_refs]) => {
    return child_refs.every((child_unique) => {
      const child_ref: $Reference = [child_name, child_unique]
      return test(context, child_ref, value)
    })
  })
}

function test_primitive([name, _]: $Reference, value: $Value): boolean {
  console.log('| test_primitive', name, value)
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
