import { PRIMITIVES } from './constants';
import { get_slice } from './dictionary';
import type { $Context, $Dictionary, $Reference, $ReferenceList, $ReferenceMap, $Value } from './types';

export function test(context: $Context, ref: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('test', ref, value)
  const slice = get_slice(context, ref)
  if (Object.entries(slice).length === 0) {
    throw Error(`No context exists for reference [${ref.join('-')}]`)
  }
  const { define, extend, assign, values } = slice
  console.log(slice)
  const define_result = define && test_define(context, define, ref, value)
  if (define_result === false) {
    context.option?.verbose && console.log(' * failed test_define', ref, value)
    return false
  }
  const extend_result = extend && test_extend(context, extend, ref, value)
  if (extend_result === false) {
    context.option?.verbose && console.log(' * failed test_extend', ref, value)
    return false
  }
  const assign_result = assign && test_assign(context, assign, ref, value)
  if (assign_result === false) {
    context.option?.verbose && console.log(' * failed test_assign', ref, value)
    return false
  }
  const values_result = values && test_values(context, values, ref, value)
  if (values_result === false) {
    context.option?.verbose && console.log(' * failed test_values', ref, value)
    return false
  }
  return test_primitive(context, ref, value)
}

function test_define(context: $Context, define: $Dictionary<$ReferenceMap>, [name, unique]: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('| test_define', define, name, unique, value)
  return Object.entries(define[name][unique]).every((child_ref) => test_property(context, child_ref, value))
}

function test_extend(context: $Context, extend: $Dictionary<$ReferenceList>, [name, unique]: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('| test_extend', name, unique, value)
  return Object.entries(extend[name][unique]).every(([child_name, child_refs]) => {
    return child_refs.every((child_unique) => {
      const child_ref: $Reference = [child_name, child_unique]
      return test(context, child_ref, value)
    })
  })
}

function test_assign(context: $Context, assign: $Dictionary<$ReferenceMap>, [name, unique]: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('| test_assign', name, unique, value)
  return Object.entries(assign[name][unique]).every((child_ref) => test_property(context, child_ref, value))
}

function test_values(context: $Context, values: $Dictionary<$Value>, [name, unique]: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('| test_values', name, unique, value)
  return JSON.stringify(value) === JSON.stringify(values[name][unique])
}

function test_primitive(context: $Context, [name, _]: $Reference, value: $Value): boolean {
  context.option?.verbose && console.log('| test_primitive', name, value)
  if (PRIMITIVES.includes(name)) {
    return typeof value === name
  }
  return true
}

function test_property(context: $Context, reference: $Reference, value: $Value) {
  const [name, _] = reference
  if (value instanceof Object && !(value instanceof Array)) {
    if (value[name] !== undefined) {
      return test(context, reference, value[name])
    }
  }
  return false
}
