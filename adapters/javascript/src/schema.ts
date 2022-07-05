import { Specification, Value } from './types'
import { isObject } from './utility'

export function getSchema(input: Value): Value {
  if (input === 'any') {
    return 'any'
  }
  const type = typeof input
  if (isObject(input)) {
    return getObjectSchema(input as Specification)
  }
  return type
}

function getObjectSchema(input: Specification): string | Specification {
  const entries = Object.entries(input)
  if (entries.length === 0) {
    return 'any'
  }
  const schema: Specification = {}
  entries.forEach((next: [string, Value]) => {
    const [key, value] = next
    schema[key] = getSchema(value)
  })
  return schema
}
