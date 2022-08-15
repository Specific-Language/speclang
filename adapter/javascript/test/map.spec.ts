import { $Context, parse } from 'speclang'

describe('map', () => {
  test('single level', async () => {
    const context: $Context = {}
    const input = `define point {
      define x number {}
      define y number {}
    }`
    await parse(context, input)
    // const { define, extend, relate} = context
    console.log(JSON.stringify(context, null, 2))
  })
})
