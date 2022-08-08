<script lang="ts">
  import type { Context } from 'speclang'
  import Parser from './Parser.svelte'
  // import Tester from './Tester.svelte'

  let context: Context
  // let matches: $Dictionary
</script>

<table>
  <tr>
    <td>
      <Parser bind:context />
    </td>
    <!-- {#if context}
      <td>
        <Tester {context} bind:matches />
      </td>
      {#if matches}
        <td>
          {#each Object.keys(matches) as key}
            {#each matches[key] as match}
              <table>
                <tr>
                  <td><pre>{JSON.stringify(match.value, null, 2)}</td>
                  <td></td>
                </tr>
                <tr>
                  <td>Name</td>
                  <td><strong>{key}</strong></td>
                </tr>
                <tr>
                  <td>ID</td>
                  <td>{match.id}</td>
                </tr>
                <tr>
                  <td>Parent ID</td>
                  <td>{match.parent}</td>
                </tr>
              </table>
            {/each}
          {/each}
        </td>
      {/if}
    {/if} -->
  </tr>
</table>

<style>
  table, tr, td {
    border-color: black;
  }
  table, tr {
    border-width: 2px;
    border-style: solid;
  }
  tr {
    vertical-align: top;
  }
</style>