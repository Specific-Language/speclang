export type Specification = {
  [x: string]: Value
}

export type Value =
  | string
  | number
  | boolean
  | Specification
  | Value[]
