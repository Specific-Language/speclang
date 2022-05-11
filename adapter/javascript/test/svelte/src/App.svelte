<script>
  import init, { parse } from "../../../dist/speclang.es";

  let input = `test { \n  attr = 123\n}`;
  let output;

  async function test() {
    // todo: can we get around await init()
    await init();
    output = parse(input);
  }
</script>

<main>
  <h1>wasm parse test</h1>
  <div>
    <textarea bind:value={input} rows="12" />
    <button on:click={test}>parse</button>
  </div>
  <hr />
  <pre>{output}</pre>
</main>
