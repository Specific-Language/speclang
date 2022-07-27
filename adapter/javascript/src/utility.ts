import { $Dictionary } from "./types"

export function array_unique_push<T>(output: Array<T>, input: T): Array<T> {
  if (!output.includes(input)) {
    output.push(input)
  }
  return output
}

export function array_unique_concat<T>(output: Array<T>, input: Array<T>): Array<T> {
  input.forEach((element) => array_unique_push(output, element))
  return output
}

export function array_safe_join<T>(array1?: Array<T>, array2?: Array<T>): Array<T> {
  if (!array1) {
    array1 = []
  }
  if (!array2) {
    array2 = []
  }
  return array_unique_concat(array1, array2)
}

export function dictionary_safe_join(object1: $Dictionary, object2: $Dictionary): $Dictionary {
  if (!object1) {
    object1 = {}
  }
  if (!object2) {
    object2 = {}
  }
  Object.keys(object2).forEach((key) => {
    array_safe_join(object1[key], object2[key])
  })
  return object1
}
