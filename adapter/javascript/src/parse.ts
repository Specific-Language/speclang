import * as speclang from '../../../pkg/speclang'
import { specify } from './specify'
import type { $Context, $Map } from './types'

export async function parse(context: $Context, raw_spec: string): Promise<void> {
  const raw_output = await speclang.parse(raw_spec)
  const output: $Map = JSON.parse(raw_output)
  context.option?.verbose && console.log(JSON.stringify(output, null, 2))
  specify(context, '$', 'number', {
    minimum: { extend: { number: {} } },
    maximum: { extend: { number: {} } },
  })
  specify(context, '$', 'string', {})
  specify(context, '$', 'boolean', {})
  specify(context, '$', 'list', {
    each: {},
  })
  specify(context, '$', 'input', output)
}
