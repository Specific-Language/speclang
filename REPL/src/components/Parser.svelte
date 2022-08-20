<script lang="ts">
  import type { $Context } from 'speclang'
  import { parse } from 'speclang'
  
  export let context: $Context
  
  let input: string = `define point {
  define x number {}
  define y number {}
}

define origin point {
  x = 0
  y = 0
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

<table>
  <tr>
    {#each Object.keys(context).sort() as dictionary}
      <td>
        <h3>{dictionary}</h3>
        <pre>{JSON.stringify(context[dictionary], null, 2)}</pre>
      </td>
    {/each}
  </tr>
</table>

<style>
  pre {
    white-space: pre-wrap;
  }
  tr {
    vertical-align: top;
  }
</style>
