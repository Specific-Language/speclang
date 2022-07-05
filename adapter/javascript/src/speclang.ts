import * as speclang from '../../../pkg/speclang'
import { Specification, Value } from './types'
import { getSchema } from './schema'
import { isArray, isObject } from './utility'

type ParserOutput = Specification & {
  document: {
    define: Specification[]
  }
}

export type Dictionary = Specification & {
  [x: string]: DictionaryEntry[]
}

export type DictionaryEntry = Specification & Value & {
  reference: string,
  pattern: Value
}

class Context {
  dictionary: Dictionary = {}

  // get
  add(parent: string, key: string, value: Value): void {
    const existingEntries = this.dictionary[key]
    const newEntry: DictionaryEntry = {
      reference: parent,
      pattern: value
    }
    if (existingEntries && isArray(existingEntries)) {
      this.dictionary[key] = [...existingEntries, newEntry]
    } else {
      this.dictionary[key] = [newEntry]
    }
  }
}

export const parse = {
  string: parseString
}

export const define = {
  string: defineString,
  specification: defineSpecification
}

export const lookup = {
    
}

export const forget = {
    
}

export const test = {


}

async function parseString(input: string): Promise<Context> {
  const context = new Context()
  const outputString = await speclang.parse(input)
  console.log(outputString)
  const output = JSON.parse(outputString) as ParserOutput
  const definitions = isArray(output.document.define) ? output.document.define : [output.document.define]
  await Promise.all(definitions.map((s) => defineString(context, 'input', s)))
  return context
}

async function defineString(context: Context, key: string, value: Value): Promise<void> {
  const schema = getSchema(value)
  if (isObject(schema)) {
    await defineSpecification(context, key, schema as Specification) 
  } else {
    context.add('document', key, schema)
  }
}

async function defineSpecification(context: Context, parent: string, spec: Specification): Promise<void> {
  const entries = Object.entries(spec)
  const definePromises = entries.map(async (pair: [string, Value]) => {
    context.add(parent, pair[0], pair[1])
    return defineString(context, pair[0], pair[1])
  })
  await Promise.all(definePromises)
}

// todo: test function?
