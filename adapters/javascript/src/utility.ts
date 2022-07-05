export const isArray = (object: unknown) => object instanceof Array
export const isObject = (object: unknown) => object instanceof Object && !isArray(object)
