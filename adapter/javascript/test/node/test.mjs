import { parse } from "speclang";

const testInput = `
list primitive {
  // -> magic -> JS array
}

object primitive {
  // -> magic -> JS object
}

hcl2 {
  block {
    name string {}
    value {}
  }
  attribute {
    name string {}
    value hcl2-expression {}
  }
  expression string {
    // functions, variables, primitive values, etc.
    // anything after the =
  }
}

specification hcl2-block {
  value-entries each constraint {}
}

constraint {
  value {}
  pattern {
    test = value
  }
}

pattern {
  test speclang-expression {}
  match speclang-function {
    input = test
    output = true
  }
}
`;

const context = await parse.string(testInput);

console.log(JSON.stringify(context, null, 4));

// const result = await context.lookup({
//   x: 123,
//   y: 321,
// })
// console.log(JSON.stringify(result, null, 4));
