export type Value =
  | Primitive
  | Value[]
  | { [name: string]: Value }

export type Primitive =
  | string
  | number
  | boolean

export type Map<T = Value> = {
  [name: string]: T
}

function isObject(value: any): value is Map {
  return (value instanceof Object) && !(value instanceof Array) && !(value instanceof Map) && !(value instanceof Set) && !(value instanceof Date);
}
function isArray(value: any): value is any[] {
  return value instanceof Array;
}
function isString(value: any): value is string {
  return typeof value === 'string';
}
function isNumber(value: any): value is number {
  return typeof value === 'number';
}
function isBoolean(value: any): value is boolean {
  return typeof value === 'boolean';
}

type ComputedFunctions<S> = {
  [P in keyof S as S[P] extends Function ? P : never]: S[P]
};

type PropertyFunctions<S> = {
  [P in keyof S as S[P] extends Function ? never : P]: <I>(input?: Model<I>) => Model<S[P]>
};

export type Model<Spec> = Map<Function> & PropertyFunctions<Spec> & ComputedFunctions<Spec>;

function Specify<Spec extends Map, Input extends Value>(spec: Spec): (input: Input) => Model<Spec> {  
  function constructor(input: Input): Model<Spec> {
    const model: Model<any> = {};
    applyMap(model, model, spec);
    if (isObject(input)) {
      Object.keys(input).forEach((key) => {
        model[key] = () => input[key]
      })
    } else {
      throw Error('unhhaneld inpjut amid the chaios')
    }
    return <Model<Spec>> model;
  }

  function applyMap(root: Map<Function>, model: Map<Function>, map: Map) {
    const keys = Object.keys(map).sort((k1, k2) => +(k2 === 'value') - +(k1 === 'value')); // todo : real sort
    keys.forEach((key: keyof typeof map) => {
      const value = map[key];
      if (isString(value)) {
        model[key] = applyString(root, model, key.toString(), value);
        return;
      }
      if (isNumber(value)) {
        model[key] = () => value;
        return;
      }
      if (isBoolean(value)) {
        model[key] = () => value;
        return;
      }
      if (isArray(value)) {
        throw Error('unhandled array lol')
      }
      if (isObject(value)) {
        const result = {};
        applyMap(root, result, value);
        model[key] = () => result;
        return;
      }
      throw Error('it is unhandled lol');
    })
  }

  function applyString<T>(root: Map<Function>, model: Map<Function>, key: string, value: string): (input?: Model<T>) => string {
    if (key === 'run') {
      if (!value.startsWith('${') || !value.endsWith('}')) {
        throw Error('expected interpol')
      }
      model.value = (input?: Model<unknown>) => {
        let evalString: string = '';
        Object.entries(root).forEach(([inputKey, inputValue]) => {
          if (inputKey === key) {
            return;
          }
          evalString += `const ${inputKey}="${inputValue()}";\n`
        });
        if (input) {
          evalString += 'const input={\n'
          Object.entries(input).forEach(([inputKey, inputValue]) => {
            evalString += `  ${inputKey}: "${inputValue()}",\n`
          });
          evalString += '};\n'
        }
        evalString += value.slice(2, -1);
        return eval(evalString);
      };
    }
    return () => value;
  }

  return constructor;
}

// point {
//   x number {}
//   y number {}
//
//   distance {
//     point2 point {}
//     value = sqrt((point2.x - point1.x)^2 + (point2.y - point1.y)^2)
//   }
// }

const Point2DSpec = {
  x: Number(),
  y: Number(),
  distance: {
    run: "${Math.sqrt((input.x - x)**2 + (input.y - y)**2)}",
    value: Number(),
  },
};

const Point2D = Specify(Point2DSpec);

const pointA = Point2D({
  x: 1,
  y: 2,
});

const pointB = Point2D({
  x: 3,
  y: 4,
});

console.log(pointB.distance().value(pointA)) // 2.8284271247461903
console.log(pointA.distance().value(pointB)) // 2.8284271247461903
console.log(pointA.distance().value(pointA)) // 0
console.log(pointB.distance().value(pointB)) // 0
