<script lang="ts">
  import type { $Dictionary, $Reference } from 'speclang'
  import { Context, test } from "speclang"
  
  export let context: Context
  export let matches: $Dictionary = []

  let rawInput = {
    x: -0.314159265358979323846264338,
    y: 100.212
  }
  let input: string = JSON.stringify(rawInput, null, 2)
  let inputError: string = ''

  let testName: string

  async function handleTest(context: Context, testName: string, testInput: string) {
    try {
      const reference = testName.split('-') as $Reference
      const parsed = JSON.parse(testInput)
      console.log(test(context.dictionary, reference, parsed))
      inputError = ''
    } catch (err) {
      console.log('test input error', err)
      inputError = `${err}`
    }
  }
  $: handleTest(context, testName, input)
</script>

<h3>test</h3>
<h4>{inputError}</h4>
<input type="text" bind:value={testName} placeholder="Test name" />
<br />
<textarea
  bind:value={input}
  rows=12
  cols=36
  />

<h3>matches</h3>
<pre>{JSON.stringify(matches, null, 4)}</pre>
