import { readFileSync } from 'fs'
import { parse } from "speclang"

// const filename = './language.spec'
// const filename = './2d.spec'
const filename = './declare.spec'

const testInput = readFileSync(filename).toString()

const parsed = await parse(testInput)

console.log(
  JSON.stringify(
    JSON.parse(parsed), 
    null, 2
  ))
