import type { $Definition, $Specification, $Value } from "./types"

export function Definition(name: string, parent?: $Definition): $Definition {
  const id = `${name}-${String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')}` // todo : this obv isn't enough
  const definition: $Definition = {
    id,
    define: {},
    extend: {},
  }
  if (parent) {
    definition.parent = parent.id
  }
  return definition
}

export function Specification(value: $Value): $Specification {
  if (value instanceof Array) {
    throw Error('unhandled case: compose_spec: Array')
  }
  if (value instanceof Object) {
    return value
  }
  return {
    [typeof value]: value
  }
}
