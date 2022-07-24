import * as speclang from '../../../pkg/speclang'
import type { $Object } from './types'
import { Context } from './context'
import { define } from './define'

export async function parse(context: Context, input: string): Promise<void> {
  const parserOutput = await speclang.parse(input)
  console.log('Parser output:\n', parserOutput, '\n')
  const spec: $Object = JSON.parse(parserOutput)
  console.log('Specification:\n', JSON.stringify(spec, null, 2), '\n')
  Object.entries(spec).forEach(([key, value]) => define(context, key, value))
  console.log('Done parsing!')
}
