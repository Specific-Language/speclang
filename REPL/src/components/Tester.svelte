<script lang="ts">
  import type { $Context, $Reference } from 'speclang'
  import { test } from "speclang"
  
  export let context: $Context
  export let result: boolean

  let rawInput = {
    x: 5,
    y: 3
  }
  let input: string = JSON.stringify(rawInput, null, 2)
  let inputError: string = ''

  let testName: string

  async function handleTest(context: $Context, testName: string, testInput: string) {
    try {
      const reference = testName.split('-') as $Reference
      if (reference[0] && reference[1] && reference[1].length === 5) {
        const parsed = JSON.parse(testInput)
        result = test(context, reference, parsed)
        inputError = ''
      } else {
        inputError = 'Test reference must match pattern "<name>-<unique>"'
      }
    } catch (err: unknown) {
      console.log('An error occurred during the test', err)
      inputError = `${err}`
    }
  }
  $: input && handleTest(context, testName, input)
</script>

<h3>test</h3>
<input type="text" bind:value={testName} placeholder="Test name" />

{#if testName && testName.split('-').length > 0}
  <h4>{inputError}</h4>
  <br />
  <textarea
    bind:value={input}
    rows=12
    cols=36
    />
  {#if !inputError}
    <h3>matches</h3>
    <pre>{JSON.stringify(result, null, 4)}</pre>
  {/if}
{/if}
