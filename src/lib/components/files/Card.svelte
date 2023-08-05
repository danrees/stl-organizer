<script lang="ts">
  import type { STLFile } from "$lib/stl-file";

  export let file: STLFile;
</script>

<div class="card w-full">
  <dl class="list-dl">
    <dt>{file.name}</dt>
    <dd>{file.path}</dd>
  </dl>
  <div>
    {#each file.tags as tag}
      <span class="chip variant-filled">{tag.value}</span>
    {/each}
  </div>
</div>
