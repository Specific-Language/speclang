import type { $Context, $Reference, $Value } from './types';
import { slice } from './dictionary';

export const PRIMITIVES = [
  'string', 
  'number', 
  'boolean',
]

export function test(context: $Context, ref: $Reference, value: $Value): boolean {
  const [name, unique] = ref
  const { define, extend, assign } = slice(context, ref)
  if (assign) {
    if (value !== assign[name][unique]) {
      return false
    }
  }
  if (define) {
    const result = Object.entries(define[name][unique]).every((child_ref) => {
      return test_property(context, child_ref, value)
    })
    if (result === false) {
      return false
    }
  }
  if (extend) {
    const result = Object.entries(extend[name][unique]).every(([child_name, child_refs]) => {
      return child_refs.every((child_unique) => {
        const child_ref: $Reference = [child_name, child_unique]
        return test(context, child_ref, value)
      })
    })
    if (result === false) {
      return false
    }
  }
  if (PRIMITIVES.includes(name)) {
    if (typeof value !== name) {
      return false
    }
  }
  return true
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
