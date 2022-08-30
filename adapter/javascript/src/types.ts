export type $Value =
  | $Primitive
  | { [name: string]: $Value }

export type $Primitive =
  | string
  | number
  | boolean
  | $Value[]

export type $Map<T extends $Value = $Value> = {
  [name: string]: T
}

export type $Context = {
  [name: string]: $Map<$Value> | undefined
} & {
  option?: $Map<$Value>
  extend?: $Map<$Value[]>
}
