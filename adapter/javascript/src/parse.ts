import * as speclang from '../../../pkg/speclang'
import type { $Object } from './types'
import { Context } from './context'
import { define } from './define'

export async function parse(context: Context, input: string): Promise<void> {
  const parserOutput = await speclang.parse(input)
  console.log('Parser output:\n', parserOutput, '\n')

  const spec: $Object = JSON.parse(parserOutput)
  console.log('Specification:\n', JSON.stringify(spec, null, 4), '\n')
  
  Object.entries(spec).forEach(([key, value]) => define(context, key, value))
  console.log('Context dictionary:\n', JSON.stringify(context.dictionary, null, 4))
  // console.log('Context meta:\n', JSON.stringify(context.meta, null, 4))
}
