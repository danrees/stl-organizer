<script lang="ts">
  import type { STLLibrary, Filter } from "$lib/stl-library";
  export let filters: Filter[] = [];
</script>

<div class="card p-4">
  <h2 class="h3">Filter by Libraries</h2>
  {#each filters as f}
    <label class="flex items-center space-x-2">
      <input type="checkbox" class="checkbox" bind:value={f.selected} />
      <p>{f.name}</p>
    </label>
  {/each}
</div>
