import { parse } from 'speclang'

describe('primitive', () => {
  test('string', async () => {
    const input = `test {}`
    const result = await parse(input)
    console.log('Result', JSON.stringify(result, null, 2))
  })
})
