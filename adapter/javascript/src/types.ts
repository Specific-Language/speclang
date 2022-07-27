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
  [name: string]: $Value
}

export type $Definition = {
  value: $Value,
  id: string,
  parent_id?: string
}

export type $Dictionary = {
  [name: string]: $Definition[]
}
