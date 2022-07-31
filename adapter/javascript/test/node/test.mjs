import { Context, parse, identify } from 'speclang';

// const language = `
// object primitive {}

// specification object {}
// `

const testInput = `
define data {
  define type {}
  define value {}
  define tony tony chopper {}
}
define string data chopper {
  type = "string"
}
// define number data {
//   type = "number"
// }
// define boolean data {
//   type = "boolean"
// }
// define object data {
//   type = "object"
// }
`

const context = new Context({
  verbose: true
})
await parse(context, testInput)
console.log(JSON.stringify(context, null, 2))

const result = identify(context, 5)
console.log(JSON.stringify(result, null, 2))

// define point {
//   define x number {
//     maximum = 10 
//   }
//   define y number {}
// }