<script lang="ts">
  import type { STLFile } from "$lib/stl-file";
  import Card from "./Card.svelte";

  export let files: STLFile[];
</script>

<div>
  <ul class="list space-y-2">
    {#each files as file}
      <li><Card {file} /></li>
    {/each}
  </ul>
</div>
