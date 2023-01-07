import { parse as parse_spec } from 'speclang'
import type { Map } from './types'

/**
 * Parse Speclang HCL2 to JSON object
 */
export async function parse(input: string): Promise<Map> {
  const spec: string = await parse_spec(input)
  const output: Map = JSON.parse(spec)
  return output;
}
