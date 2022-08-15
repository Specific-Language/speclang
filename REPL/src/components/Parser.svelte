<script lang="ts">
  import type { $Context } from 'speclang'
  import { parse } from 'speclang'
  
  export let context: $Context
  
  let input: string = `define point {
  define x number {}
  define y number {}
}`
  let inputError: string = ''

  async function handleParse(input: string) {
    try {
      context = {}
      await parse(context, input)
      inputError = ''
      context = context
    } catch (err: unknown) {
      if (err instanceof Error) {
        console.log('parse error', err.message)
      }
      inputError = `${err}`
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

<h3>context</h3>
<pre>{JSON.stringify(context, null, 2)}</pre>

<style>
  pre {
    white-space: pre-wrap;
  }
</style>
