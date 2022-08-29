export type $Value =
  | $Primitive
  | $Value[]
  | { [name: string]: $Value }

export type $Primitive =
  | string
  | number
  | boolean
  | undefined

export type $Map<T extends $Value = $Value> = {
  [name: string]: T
}

export type $Context = {
  [name: string]: $Map<$Value>
} & {
  option?: $Map<$Value>
  define?: $Map<$Value[]>
  alias?: $Map<string[]>
}
