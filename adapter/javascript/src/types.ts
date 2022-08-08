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
  value?: $Value
  origin?: $Object
  define?: $Object
  extend?: $Object
}

export type $Dictionary = {
  [name: string]: {
    [unique: string]: $Definition
  },
}

export type $Reference = [name: string, unique: string]
