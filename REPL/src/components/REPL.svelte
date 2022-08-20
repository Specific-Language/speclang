<script lang="ts">
  import type { $Context, $Map } from 'speclang'
  import Parser from './Parser.svelte'
  import Tester from './Tester.svelte';

  let context: $Context

  const display: $Map = {
    assign: true,
    define: true,
    extend: true,
    values: true,
  }
</script>

<table>
  <tr>
    <td>
      <Parser bind:context />
    </td>
     {#if context}
      <td>
        <Tester {context} />
      </td>
    {/if}
  </tr>
</table>

{#if context}
  <table class="output">
    <tr>
      {#each Object.keys(context)
        .sort((a, b) => {
          return JSON.stringify(context[b]).length - JSON.stringify(context[a]).length
        }) as dictionary}
        <td>
          <h3>
            <label>
              {dictionary}
              <input type="checkbox" bind:checked={display[dictionary]}>
            </label>
          </h3>
          {#if display[dictionary]}
            <pre>{JSON.stringify(context[dictionary], null, 2)}</pre>
          {/if}
        </td>
      {/each}
    </tr>
  </table>
{/if}

<style>
  pre {
    white-space: pre-wrap;
  }
  table {
    margin: 1.5em;
    border-spacing: 1.5em;
  }
  table, tr, td {
    border-color: black;
  }
  table, tr {
    border-width: 2px;
    border-style: solid;
  }
  tr, td {
    vertical-align: top;
  }
</style>