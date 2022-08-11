export type $Value =
  | $Primitive
  | $Value[]
  | { [name: string]: $Value }

export type $Primitive =
  | string
  | number
  | boolean

export type $Object = Record<string, $Value>

export type $Reference = [string, string]

export type $Definition = {
  value?: $Value
  origin?: $Object
  define?: Record<$Reference[0], $Reference[1]>
  extend?: Record<$Reference[0], $Reference[1]>
}

export type $Dictionary = {
  [name: string]: {
    [unique: string]: $Definition
  },
}
