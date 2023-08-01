<script lang="ts">
  import type { STLLibrary } from "$lib/stl-library";
  import { appWindow } from "@tauri-apps/api/window";
  import { onDestroy, onMount } from "svelte";

  export let libraries: STLLibrary[];
  let unlisten: () => void;
  $: {
    console.log(libraries);
  }
  onMount(async () => {
    unlisten = await appWindow.listen<STLLibrary>(
      "library-save",
      ({ payload }) => {
        libraries = [...libraries, payload];
      },
    );
  });
  onDestroy(async () => {
    if (unlisten) {
      unlisten();
    }
  });
</script>

<div>
  <dl class="list-dl">
    {#each libraries as lib}
      <div class="flex bg-surface-100-800-token">
        <span class="flex-auto">
          <dt class="text-lg">{lib.name}</dt>
          <dd class="text-sm text-primary-400">{lib.path}</dd>
        </span>
        <span>
          <a class="btn" href={`/library/${lib.id.split(":").at(-1)}`}>Edit</a>
          <button class="btn btn-sm variant-filled-warning">Delete</button>
        </span>
      </div>
    {/each}
  </dl>
</div>
