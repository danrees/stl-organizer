<script lang="ts">
  import List from "$lib/components/files/List.svelte";
  import { listFiles, type STLFile } from "$lib/stl-file";
  import { InputChip } from "@skeletonlabs/skeleton";
  import { onMount } from "svelte";

  let files: STLFile[] = [];
  let visibleFiles: STLFile[] = [];
  let tagFilters: string[] = [];

  onMount(() => {
    listFiles().then((f) => {
      files = [...f];
      visibleFiles = [...f];
    });
  });

  $: {
    visibleFiles = files.filter((item) => {
      if (tagFilters.length == 0) {
        return true;
      }
      return item.tags.some((t) => tagFilters.some((i) => i === t.value));
    });
  }

  function onTagClicked(value: CustomEvent<{ value: string }>) {
    tagFilters = [...tagFilters, value.detail.value];
  }
</script>

<div>
  <h1 class="h1">Files</h1>
  <InputChip
    name="tag-filter"
    placeholder="Enter tags"
    bind:value={tagFilters}
  />
  <div class="overflow-x-scroll max-h-screen w-full">
    <List files={visibleFiles} on:tag-clicked={onTagClicked} />
  </div>
</div>
