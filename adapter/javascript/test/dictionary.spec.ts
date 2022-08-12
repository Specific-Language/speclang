import type { $Value, $Dictionary } from 'speclang'
import { define_spec } from 'speclang'

describe('context', () => {
  describe('define', () => {
    test('string', () => {
      const name = 'foo'
      const value = 'abc'
      test_single_spec(name, value)
    })
    test('boolean', () => {
      const name = 'foo'
      const value = true
      test_single_spec(name, value)
    })
    test('number', () => {
      const name = 'foo'
      const value = 123
      test_single_spec(name, value)
    })
  })
})

function test_single_spec(name: string, value: $Value) {
  const dictionary: $Dictionary = {}
  const [ref_name, ref_unique] = define_spec(dictionary, name, value)
  expect(ref_name).toBe(name)
  expect(ref_unique.length > 0).toBe(true)
  expect(dictionary[name] instanceof Object).toBe(true)
  const entries = Object.entries(dictionary[name])
  expect(entries.length).toBe(1)
  const [unique, spec] = entries[0]
  expect(unique).toBe(ref_unique)
  expect(spec.value).toBe(value)
  expect(spec.extend).toBe(undefined)
  expect(spec.define).toBe(undefined)
}
