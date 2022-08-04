import { Context, Recognizer } from 'speclang'

describe('recognizer', () => {
  test('constructor', () => {
    const recog = new Recognizer({
      verbose: true
    })
    const { context, language } = recog 
    testContext(context)
    testLanguage(language)
  })
})

function testContext(context: Context) {
  expect(context instanceof Context).toBe(true)
  expect(context.dictionary).toStrictEqual({})
}

function testLanguage(language: Context) {
  expect(language instanceof Context).toBe(true)
  const {
    speclang
  } = language.dictionary
  expect(speclang instanceof Array).toBe(true)
  expect(language.dictionary).toStrictEqual({})
}
