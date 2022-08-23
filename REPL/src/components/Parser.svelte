<script lang="ts">
  import type { $Context } from 'speclang'
  import { parse } from 'speclang'
  
  export let context: $Context

  let input: string = `define point {
  define x extend number {
    maximum = 5
  }
  define y extend number {}
}

define origin extend point {
  x = 0
  y = 0
}`
  let inputError: string = ''

  async function handleParse(input: string) {
    try {
      context = {
        option: {
          verbose: true
        }
      }
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
