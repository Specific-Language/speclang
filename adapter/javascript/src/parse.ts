import * as speclang from '../../../pkg/speclang'
import { specify } from './specify'
import type { $Context, $Map } from './types'

export async function parse(context: $Context, raw_spec: string): Promise<void> {
  const raw_output = await speclang.parse(raw_spec)
  const output: $Map = JSON.parse(raw_output)
  output['timestamp'] = new Date().toISOString()
  context.option?.verbose && console.log(JSON.stringify(output, null, 2))
  specify(context, '$', 'parse', output)
}
