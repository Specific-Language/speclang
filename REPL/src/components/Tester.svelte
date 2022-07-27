<script lang="ts">
  import type { Context } from "speclang"
  import type { $Dictionary } from "speclang/dist/types";
  import { identify } from "speclang";
  
  export let context: Context
  export let matches: $Dictionary

  let rawInput = {
    x: -0.314159265358979323846264338,
    y: 100.212
  }
  let input: string = JSON.stringify(rawInput, null, 2)
  let inputError: string = ''

  async function handleTest(context: Context, testInput: string) {
    try {
      const test = JSON.parse(testInput)
      matches = identify(context, test)
      inputError = ''
    } catch (err) {
      console.log('test input error', err)
      inputError = `${err}`
      matches = []
    }
  }
  $: handleTest(context, input)
</script>

<h3>test</h3>
<h4>{inputError}</h4>
<textarea
  bind:value={input}
  rows=12
  cols=36
  />

<h3>matches</h3>
<pre>{JSON.stringify(matches, null, 4)}</pre>
