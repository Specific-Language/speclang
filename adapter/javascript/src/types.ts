export type Value =
  | Primitive
  | { [name: string]: Value }

export type Primitive =
  | string
  | number
  | boolean
  | Value[]

export type Map<T extends Value = Value> = {
  [name: string]: T
}
