import type { $Reference, $ReferenceMap, $Value } from './types'
import type { $Dictionary } from './dictionary'
import * as speclang from '../../../pkg/speclang'
import { specify } from './specify'

export const DEFAULT_OPTIONS = {
  logging: {
    verbose: false
  }
}

export type $Context = {
  [name: string]: $Dictionary<$Value>
} & {
  option?: typeof DEFAULT_OPTIONS
  define?: $Dictionary<$ReferenceMap>
  extend?: $Dictionary<$ReferenceMap>
  relate?: $Dictionary<$ReferenceMap> // needs refactor
  assign?: $Dictionary<$Value>
}

export async function parse(context: $Context, input: string): Promise<$Reference> {
  const raw_output = await speclang.parse(input)
  const output: $Value = JSON.parse(raw_output)
  return set(context, '$-parse', output)
}

export function set(context: $Context, name: string, value: $Value): $Reference {
  const unique = String(Number(Math.random().toPrecision(5).substring(2))).padEnd(5, '0')
  context.define ??= {}
  context.define[name] ??= {}
  context.define[name][unique] ??= {}
  const reference: $Reference = [name, unique]
  specify(context, reference, value)
  return reference
}
