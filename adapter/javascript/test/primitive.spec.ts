import { $Context, parse } from 'speclang'

describe('primitive', () => {
  test('number', async () => {
    const context: $Context = {}
    const input = `define number {}`
    await parse(context, input)
    const { define, extend, relate} = context
    console.log(JSON.stringify(context, null, 2))
    expect(Object.entries(define.number).length).toBe(1)
    expect(extend).toBe(undefined)
    expect(Object.entries(relate.number).length).toBe(1)
  })
  test('nested number', async () => {
    const context: $Context = {}
    const input = `define minimum number {}`
    await parse(context, input)
    const { define, extend, relate} = context
    console.log(JSON.stringify(context, null, 2))
    expect(Object.entries(define.number).length).toBe(1)
    expect(Object.entries(define.minimum).length).toBe(1)
    expect(Object.entries(extend.minimum).length).toBe(1)
    expect(Object.entries(relate.number).length).toBe(1)
    expect(Object.entries(relate.minimum).length).toBe(1)
  })
})
