import * as speclang from '../../../pkg/speclang'
import type { $Context, $Map, $Reference } from './types'
import { specify } from './specify'

export async function parse(context: $Context, raw_spec: string): Promise<$Reference> {
  const raw_output = await speclang.parse(raw_spec)
  const output: $Map = JSON.parse(raw_output)
  output['timestamp'] = new Date().toISOString()
  context.option?.verbose && console.log(JSON.stringify(output, null, 2))
  return specify(context, '$-parse', output)
}
