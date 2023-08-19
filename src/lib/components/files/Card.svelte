<script lang="ts">
  import type { STLFile } from "$lib/stl-file";
  import { createEventDispatcher } from "svelte";

  export let file: STLFile;

  const dispatch = createEventDispatcher();
  function clickTag(value: string) {
    dispatch("tag-clicked", {
      value: value,
    });
  }
</script>

<div class="card card-hover p-4 w-full">
  <dl class="list-dl">
    <dt>{file.name}</dt>
    <dd>{file.path}</dd>
  </dl>
  <div>
    {#each file.tags as tag}
      <span class="chip variant-filled" on:click={() => clickTag(tag.value)}
        >{tag.value}</span
      >
    {/each}
  </div>
</div>
