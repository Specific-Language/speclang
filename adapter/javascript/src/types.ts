export type $Value =
  | $Primitive
  | $Object
  | $Value[]

export type $Primitive =
  | string
  | number
  | boolean
  | null

export type $Object = {
  [name: string]: $Value,
}

export type $Dictionary = {
  [name: string]: $Definition[],
}

export type $Definition = {
  id: $ID,
  parent?: $ID,
  define?: $Object,
  extend?: $Object,
}

export type $ID = [name: string, id: string]
