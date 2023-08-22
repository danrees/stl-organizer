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
    <dt>
      <div class="w-full flex place-content-between">
        <span>{file.name}</span>
        <a
          class="btn btn-sm variant-filled"
          href={`/files/${file.id.split(":").at(-1)}`}>Details</a
        >
      </div>
    </dt>
    <dd>{file.path}</dd>
  </dl>
  <div>
    {#each file.tags as tag}
      <button class="chip variant-filled" on:click={() => clickTag(tag.value)}
        >{tag.value}</button
      >
    {/each}
  </div>
</div>
