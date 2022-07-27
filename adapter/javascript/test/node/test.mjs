import { Context, parse, identify } from 'speclang';

// const language = `
// object primitive {}

// specification object {}
// `

const testInput = `
define point {
  x number {}
  y number {}
}
// location point {
//   x gps-lat {}
//   y gps-lon {}
// }
// origin point {
//   x = 0
//   y = 0
// }
`

const context = new Context()
await parse(context, testInput, {
  verbose: true
})
console.log(JSON.stringify(context))

const result = identify(context, 5)
console.log(JSON.stringify(result, null, 4))
