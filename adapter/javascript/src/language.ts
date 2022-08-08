import { $Dictionary, $Definition } from "./types"
// import { $Object } from "./types"

export function Language() {
  const dictionary: $Dictionary = {
    speclang: {
      'v0': LANGUAGE_DEFINITION
    }
  }
  // specify(dictionary, LANGUAGE_DEFINITION, LANGUAGE_SPEC)
  // define(dictionary, 'speclang', LANGUAGE_DEFINITION)
  return dictionary
}

// const LANGUAGE_SPEC: $Object = {
//   define: {
//     string: {
//       primitive: 'string',
//     },
//     number: {
//       primitive: 'number',
//     }
//   }
// }

const LANGUAGE_DEFINITION: $Definition = {
  define: {},
  extend: {},
}
