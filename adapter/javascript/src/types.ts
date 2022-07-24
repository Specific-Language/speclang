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

export type $DefinitionTable = {
  [key: string]: $Definition[]
}

export type $Event = {
  name: string,
  time: string,
}

export type $EventTable = {
  [name: string]: $Event
}
