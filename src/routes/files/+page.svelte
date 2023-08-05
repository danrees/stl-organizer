<script lang="ts">
  import List from "$lib/components/files/List.svelte";
  import { listFiles, type STLFile } from "$lib/stl-file";

  let stlFiles: Promise<STLFile[]> = listFiles();
</script>

<div>
  <h1>Files</h1>
  {#await stlFiles}
    <p>Loading ...</p>
  {:then files}
    <div class="overflow-x-scroll max-h-screen w-full">
      <List {files} />
    </div>
  {/await}
</div>
