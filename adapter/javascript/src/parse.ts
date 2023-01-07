import * as speclang from '../../../pkg/speclang'
import { specify } from './specify'
import { ascending_pairs } from './shared'
import type { $Context, $Map } from './types'

export async function parse(context: $Context, raw_spec: string): Promise<void> {
  const raw_output = await speclang.parse(raw_spec)
  const output: $Map = JSON.parse(raw_output)
  // specify(context, 'number', {
  //   maximum: {},
  //   minimum: {},
  // })
  // specify(context, 'list', {
  //   each: {},
  //   length: {
  //     extend: {
  //       number: {}
  //     }
  //   }
  // })
  ascending_pairs(output).forEach(([name, value]) => specify(context, name, value))
}
