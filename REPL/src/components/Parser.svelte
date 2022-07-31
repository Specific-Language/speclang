<script lang="ts">
  import { Context, parse } from "speclang";
  import type { $Dictionary } from "speclang/dist/types";
  
  export let context: Context
  
  let input: string = `define point {
  x number {}
  y number {}
}`
  let inputError: string = ''
  let output: $Dictionary | undefined

  async function handleParse(input: string) {
    try {
      context = new Context()
      await parse(context, input, {
        verbose: true
      })
      output = context.dictionary
      inputError = ''
      context = context
    } catch (err) {
      console.log('parse input error', err)
      inputError = `${err}`
      output = undefined
    }
  }
  $: handleParse(input)
</script>

<h3>input</h3>
<h4>{inputError}</h4>
<textarea
  bind:value={input}
  rows=12
  cols=36
  />

<h3>dictionary</h3>
<pre>{JSON.stringify(output, null, 2)}</pre>

<style>
  pre {
    white-space: pre-wrap;
  }
</style>
