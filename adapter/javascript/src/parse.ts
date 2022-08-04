import * as speclang from '../../../pkg/speclang'
import type { $Object } from './types'
import { Recognizer } from './recognizer'
import { Definition } from './define'

export async function parse(recog: Recognizer, input: string): Promise<void> {
  const output_string = await speclang.parse(input)
  const output: $Object = JSON.parse(output_string)
  recog.options.verbose && console.log('Parsed HCL2 input as JSON')
  const parse_definition = Definition('speclang-parse')
  recog.context.specify(parse_definition, output)
  recog.context.define('speclang-parse', parse_definition)
  recog.options.verbose && console.log('Successfully understood input as Specific Language')
}
