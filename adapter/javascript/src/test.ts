import { PRIMITIVES } from './constants';
import { lookup } from './dictionary';
import type { $Context, $Value } from './types';

export function test(context: $Context, reference: string, value: $Value): boolean {
  context.option?.verbose && console.log('test', reference, value)
  const subcontext = lookup(context, reference)
  if (Object.entries(subcontext).length === 0) {
    throw Error(`No context exists for reference [${reference}]`)
  }
  const define_result = test_define(context, reference, value)
  if (define_result === false) {
    context.option?.verbose && console.log(' * failed test_define', reference, value)
    return false
  }
  const values_result = test_values(context, reference, value)
  if (values_result === false) {
    context.option?.verbose && console.log(' * failed test_values', reference, value)
    return false
  }
  return test_primitive(context, reference, value)
}

function test_define(context: $Context, reference: string, value: $Value): boolean {
  if (!context.define || !context.define[reference]) {
    throw Error(`Expected to find define spec for ${reference}`)
  }
  context.option?.verbose && console.log('| test_define', reference, value)
  const spec = context.define[reference]
  return Object.entries(spec).every(([name, _]) => test_property(context, name, value))
}

function test_values(context: $Context, reference: string, value: $Value): boolean {
  context.option?.verbose && console.log('| test_values', reference, value)
  if (!context.value || !context.value[reference]) {
    throw Error(`Expected to find define spec for ${reference}`)
  }
  const spec = context.value[reference]
  return JSON.stringify(value) === JSON.stringify(spec)
}

function test_primitive(context: $Context, reference: string, value: $Value): boolean {
  context.option?.verbose && console.log('| test_primitive', reference, value)
  if (PRIMITIVES.includes(reference)) {
    return typeof value === reference
  }
  return true
}

function test_property(context: $Context, reference: string, value: $Value) {
  if (value instanceof Object && !(value instanceof Array)) {
    if (value[reference] !== undefined) {
      return test(context, reference, value[reference])
    }
  }
  return false
}
