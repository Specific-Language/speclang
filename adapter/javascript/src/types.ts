export type $Value =
  | $Primitive
  | $Value[]
  | $Map

export type $Primitive =
  | string
  | number
  | boolean

export type $Map = { 
  [name: string]: $Value 
}

export type $Reference = [string, string]

export type $ReferenceMap = Record<$Reference[0], $Reference[1]>

export type $Dictionary = {
  [name: string]: {
    [unique: string]: $Specification
  },
}

export type $Specification = {
  value?: $Value
  origin?: $Map
  define?: $ReferenceMap
  extend?: $ReferenceMap
}
