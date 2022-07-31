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

export type $Definition = {
  input: $Value,
  define: string[],
  extend: string[],
  id: string,
  parent_id?: string,
}

export type $Dictionary = {
  [name: string]: $Definition[],
}

export type $Specification = $Object & {
  define?: $Specification,
}

// export type $Identifier = `${string}-${string}`
