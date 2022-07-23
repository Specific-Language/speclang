import { Context, parse, lookup } from 'speclang';

// const language = `
// object primitive {}

// specification object {}
// `

const testInput = `
// point {
//   x number {}
//   y number {}
// }
z number {}
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
await parse(context, testInput)

const result = lookup(context, 5)
console.log(JSON.stringify(result, null, 4))
