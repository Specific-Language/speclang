(async () => {
  var speclang = require("speclang");

  const testInput = `test {
    hello world {}
    foo = "bar"
  }`;

  var parsed = await speclang.parse(testInput);

  console.log(parsed);
})();
