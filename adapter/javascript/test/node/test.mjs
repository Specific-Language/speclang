import { parse } from "speclang";

const testInput = `test {
  hello world {}
  foo = "bar"
}`;

const parsed = await parse(testInput);

console.log(parsed);
