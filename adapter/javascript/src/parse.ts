import { parse_str } from 'speclang'
import type { Map } from './types'

/**
 * Parse Speclang HCL2 to JSON object
 */
export async function parse(input: string): Promise<Map> {
  const spec: string = await parse_str(input)
  console.log(1000, spec)
  const output: Map = JSON.parse(spec)
  return output;
}
