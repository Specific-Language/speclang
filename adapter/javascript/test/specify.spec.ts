import { Context, parse } from 'speclang'

describe('parse', () => {
  describe('define', () => {
    test('n=1', async () => {
      const context = new Context()
      const input = `define something {}`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(1)
      expect(context.dictionary['something'].length).toBe(1)
    })
    test('n>1', async () => {
      const context = new Context()
      const input = `define something {} define somethingelse {}`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(2)
      expect(context.dictionary['something'].length).toBe(1)
      expect(context.dictionary['somethingelse'].length).toBe(1)
    })
    test('n=1 nested', async () => {
      const context = new Context()
      const input = `define something {
        define somethingelse {}
      }`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(2)
      expect(context.dictionary['something'].length).toBe(1)
      expect(context.dictionary['somethingelse'].length).toBe(1)
    })
    test('n>1 nested', async () => {
      const context = new Context()
      const input = `define something {
        define somethingelse {
          define somethingthird {}
        }
      }`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(3)
      expect(context.dictionary['something'].length).toBe(1)
      expect(context.dictionary['somethingelse'].length).toBe(1)
      expect(context.dictionary['somethingthird'].length).toBe(1)
    })
  })
  describe('extend', () => {
    test('n=1', async () => {
      const context = new Context()
      const input = `define something {
        somethingelse {}
      }`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(2)
      expect(context.dictionary['something'].length).toBe(1)
      const definition = context.dictionary['something'][0]
      expect(definition.extend instanceof Object).toBe(true)
      expect(Object.entries(definition.extend).length).toBe(1)
      if (!definition.extend['somethingelse']) {
        throw Error('Expected a definition for "somethingelse"')
      }
      if (typeof definition.extend['somethingelse'] !== 'string') {
        throw Error('Expected definition for "somethingelse" to be a lookup string')
      }
      expect(definition.extend['somethingelse'].startsWith('somethingelse-')).toBe(true)
    })
    test('n=1 direct assign number', async () => {
      const context = new Context()
      const input = `define something {
        somethingelse = 123
      }`
      await parse(context, input)
      expect(Object.entries(context.dictionary).length).toBe(1)
      expect(context.dictionary['something'].length).toBe(1)
      const definition = context.dictionary['something'][0]
      expect(definition.extend instanceof Object).toBe(true)
      expect(Object.entries(definition.extend).length).toBe(1)
      if (!definition.extend['somethingelse']) {
        throw Error('Expected a definition for "somethingelse"')
      }
      expect(definition.extend['somethingelse']).toBe(123)
    })
  })
})
