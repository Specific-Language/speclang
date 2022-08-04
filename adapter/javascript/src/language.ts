import { Context } from "./context"
import { $Definition, $Object } from "./types"

export function LanguageContext() {
  const context = new Context()
  context.specify(LANGUAGE_DEFINITION, LANGUAGE_SPEC)
  context.define('speclang', LANGUAGE_DEFINITION)
  return context
}

const LANGUAGE_SPEC: $Object = {
  define: {
    string: {
      primitive: 'string',
    },
    number: {
      primitive: 'number',
    }
  }
}

const LANGUAGE_DEFINITION: $Definition = {
  id: ['speclang', 'v0.1'],
  define: {},
  extend: {},
}
