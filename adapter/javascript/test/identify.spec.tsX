import { Context, parse, identify } from 'speclang'

describe('identify', () => {
  describe('primitives', () => {
    test('string', () => {
      const context = new Context()
      const result = identify(context, 'abc')
      expect(Object.entries(result).length).toBe(1)
      expect(result['string'].length).toBe(1)
      expect(result['string'][0].extend['primitive']).toBeTruthy()
      expect(result['string'][0].parent).toBe('speclang')
    })
    test('number', () => {
      const context = new Context()
      const result = identify(context, 48.5)
      expect(Object.entries(result).length).toBe(1)
      expect(result['number'].length).toBe(1)
      expect(result['number'][0].extend['primitive']).toBeTruthy()
      expect(result['number'][0].parent).toBe('speclang')
    })
    test('boolean', () => {
      const context = new Context()
      const result = identify(context, false)
      expect(Object.entries(result).length).toBe(1)
      expect(result['boolean'].length).toBe(1)
      expect(result['boolean'][0].extend['primitive']).toBeTruthy()
      expect(result['boolean'][0].parent).toBe('speclang')
    })
  })
  describe.only('contextual recognition', () => {
    test('string', async () => {
      const context = new Context({
        verbose: true
      })
      await parse(context, `define foo string {}`)
      console.log(JSON.stringify(context.dictionary, null, 2))
      const result = identify(context, 'abc')
      console.log(JSON.stringify(result, null, 2))
      expect(Object.entries(result).length).toBe(1)
      expect(result['foo'].length).toBe(1)
    })
  })
})
