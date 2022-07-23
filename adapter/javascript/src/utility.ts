export const isArray = (object: unknown) => object instanceof Array
export const isObject = (object: unknown) => object instanceof Object && !isArray(object)
export const areEqual = (object1: unknown, object2: unknown) => JSON.stringify(object1) === JSON.stringify(object2)
