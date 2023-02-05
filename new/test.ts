function generateClas(definition: any) {
  function buildClass(instance: Definition, definition: any) {
    for (const name in definition) {
      const spec = definition[name];
      console.log('name is', name, '\n', 'spec is', spec, '\n');
      if (typeof spec === 'object') {
        if (spec.hasOwnProperty('value')) {
          if (typeof spec.value === 'string') {
            const match = spec.value.match(/\$\{(.*)\}/);
            if (match) {
              console.log('spec.value string eval', name, spec, '\n');
              instance[name] = () => eval(match[1]);
            } else {
              console.log('spec.value string', name, spec, '\n');
              instance[name] = () => spec.value;
            }
          } else {
            console.log('spec.value', name, spec, '\n');
            instance[name] = () => spec.value;
          }
        } else {
          console.log('recursive', name, spec, '\n');
          instance[name] = () => buildClass(instance, spec[0]);
        }
      } else {
        console.log('direct value', name, spec, '\n');
        instance[name] = () => spec;
      }
    }
  }
  class Definition implements Record<string, unknown> {
    [name: string]: (...args: unknown[]) => any;

    constructor() {
      buildClass(this, definition);
    }
  }
  return Definition;
}

// point {
//   x number {}
//   y number {}
  
//   distance {
//     point2 point {}
//     value = sqrt((point2.x - point1.x)^2 + (point2.y - point1.y)^2)
//   }
// }

const Point2D = generateClas({
      x: [{}],
      y: [{}],
      distance: [
        {
          point2: [
            {
              point: [{}],
            },
          ],
          value: "${sqrt((point2.x - point1.x)^2 + (point2.y - point1.y)^2)}",
        },
      ],
    });

function get(...args: unknown[]) {

}

const point1 = new Point2D();
point1.x = () => 0;
point1.y = () => 0;
const point2 = new Point2D();
point2.x = () => 3;
point2.y = () => 4;
console.log(point1);
console.log(point1.distance(point2)) // outputs 5

