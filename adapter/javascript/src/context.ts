import * as speclang from '../../../pkg/speclang'
import type { $Dictionary, $Value } from './types'
import { create_definition } from './definition'
// import { Language } from "./language"

export const DEFAULT_OPTIONS = {
  verbose: false,
}

export class Context {
  options: typeof DEFAULT_OPTIONS
  // language: $Dictionary
  dictionary: $Dictionary

  constructor(options: typeof DEFAULT_OPTIONS = DEFAULT_OPTIONS) {
    this.options = options
    // this.language = Language()
    this.dictionary = {}
  }
}

export async function parse(context: Context, input: string): Promise<void> {
  const raw_output = await speclang.parse(input)
  const output: $Value = JSON.parse(raw_output)
  context.options.verbose && console.log('Parsed HCL2 input as JSON')
  create_definition(context.dictionary, '$-parse', output)
  context.options.verbose && console.log('Successfully understood input as Specific Language')
}
