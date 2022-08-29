import { $Context, parse } from 'speclang'

describe('primitive', () => {
  test('number', async () => {
    const context: $Context = {}
    const input = `negative extend number {
      maximum < 0
    }`
    await parse(context, input)
    const { define, extend } = context
    console.log(JSON.stringify(context, null, 2))

  })
})
