export type $Value =
  | $Primitive
  | $Map
  | $Value[]

export type $Primitive =
  | string
  | number
  | boolean
  | undefined

export type $Map = {
  [name: string]: $Value
}

export type $Dictionary<T extends $Value> = {
  [name: string]: {
    [unique: string]: T
  },
}

export type $Reference = [name: string, unique: string]
export type $ReferenceMap = Record<$Reference[0], $Reference[1]>
export type $ReferenceList = Record<$Reference[0], $Reference[1][]>

export type $Context = {
  [name: string]: $Dictionary<$Value>
} & {
  define?: $Dictionary<$ReferenceMap>
  extend?: $Dictionary<$ReferenceList>
  parent?: $Dictionary<$ReferenceMap>
  values?: $Dictionary<$Value>
}
