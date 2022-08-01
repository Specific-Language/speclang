import type { $Value } from 'speclang'
import { Context, Definition } from 'speclang'

describe('context', () => {
  describe('define', () => {
    test('string', () => {
      const name = 'foo'
      const value = 'abc'
      test_single_definition(name, value)
    })
    test('boolean', () => {
      const name = 'foo'
      const value = true
      test_single_definition(name, value)
    })
    test('number', () => {
      const name = 'foo'
      const value = 123
      test_single_definition(name, value)
    })
  })
})

function test_single_definition(name: string, value: $Value) {
  const context = new Context()
  const definition = Definition(name)
  definition.extend[typeof value] = value
  expect(context.dictionary[name]).toBe(undefined)
  context.define(name, definition)
  expect(context.dictionary[name] instanceof Array).toBe(true)
  expect(context.dictionary[name].length).toBe(1)
  expect(context.dictionary[name][0].extend instanceof Object).toBe(true)
  expect(context.dictionary[name][0].extend[typeof value]).toBe(value)
}
