export type $Value =
  | $Primitive
  | $Map

export type $Primitive =
  | string
  | number
  | boolean
  | $Value[]
  | undefined

export type $Map = {
  [name: string]: $Value
}

export type $Reference = [name: string, unique: string]
export type $ReferenceMap = Record<$Reference[0], $Reference[1]>
