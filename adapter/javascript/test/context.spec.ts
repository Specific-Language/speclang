import { Context } from 'speclang'

describe('context', () => {
  test('constructor', () => {
    const context = new Context({
      verbose: true
    })
    testContext(context)
  })
})

function testContext(context: Context) {
  expect(context instanceof Context).toBe(true)
  expect(context.dictionary).toStrictEqual({})
}

// function testLanguage(language: Context) {
//   expect(language instanceof Context).toBe(true)
//   const {
//     speclang
//   } = language.dictionary
//   expect(speclang instanceof Array).toBe(true)
//   expect(language.dictionary).toStrictEqual({})
// }
