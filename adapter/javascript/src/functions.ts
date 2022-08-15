import type { $Map, $Primitive, $Value } from './types'

export function handle_value<M extends $Map, P extends $Primitive, Result>(
  value: $Value, 
  handler_M: (value: M) => Result, 
  handler_P: (value: P) => Result
): Result {
  if (value instanceof Object && !(value instanceof Array)) {
    return handler_M(value as M)
  }
  return handler_P(value as P)
}
