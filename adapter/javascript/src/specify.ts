import type { $Reference, $Value } from './types'
import { $Context, set } from './context'
import { define, assign, relate, extend } from './dictionary'
import { handle_value } from './functions'

export function specify(context: $Context, ref: $Reference, value: $Value) {
  handle_value(value,
    (value) => {
      const { define, ...extend } = value
      define && handle_define(context, ref, define)
      extend && handle_extend(context, ref, extend)
    },
    (value) => {
      handle_extend(context, ref, value)
    }
  )
}

function handle_define(context: $Context, ref: $Reference, value: $Value) {
  handle_value(value,
    (value) => Object.entries(value).forEach(([child_name, child_value]) => {
      const child_ref = set(context, child_name, child_value)
      define(context, ref, child_ref)
      relate(context, child_ref, ref)
    }),
    (value) => {
      const child_ref = set(context, typeof value, value)
      define(context, ref, child_ref)
      relate(context, child_ref, ref)
    }
  )
}

function handle_extend(context: $Context, ref: $Reference, value: $Value) {
  handle_value(value,
    (value) => Object.entries(value).forEach(([child_name, child_value]) => {
      const child_ref = set(context, child_name, child_value)
      extend(context, ref, child_ref)
      relate(context, child_ref, ref)
    }),
    (value) => assign(context, ref, value)
  )
}
