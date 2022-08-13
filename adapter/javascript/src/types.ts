export type $Value =
  | $Map
  | $Primitive

export type $Primitive =
  | string
  | number
  | boolean
  | undefined
  | $Value[]

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
  [name: string]: $Value
  relate?: $ReferenceMap
  define?: $ReferenceMap
  extend?: $ReferenceMap
}
