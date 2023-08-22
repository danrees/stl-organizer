<script lang="ts">
  import type { STLFile } from "$lib/stl-file";

  export let file: STLFile;
</script>

<div>
  <h3 class="h3">{file.name}</h3>
  {#if file.thumbnail}
    <img alt={file.name} src={file.thumbnail} />
  {/if}
</div>
