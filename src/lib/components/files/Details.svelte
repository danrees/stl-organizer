<script lang="ts">
  import type { STLFile } from "$lib/stl-file";
  import { loadSTL } from "$lib/preview";
  import { onMount } from "svelte";
  import { createScene } from "$lib/scene";
  import { isE } from "$lib/stl-library";
  import { toastStore } from "@skeletonlabs/skeleton";
  import { browser } from "$app/environment";

  export let file: STLFile;
  let el: HTMLCanvasElement;

  onMount(async () => {
    if (browser) {
      try {
        const data = await loadSTL(file.path);
        const animate = createScene(el, window, data.buffer);
        animate();
      } catch (e) {
        if (isE(e)) {
          toastStore.trigger({ message: e.message });
        } else {
          console.log(e);
        }
      }
    } else {
      toastStore.trigger({ message: "not browser" });
    }
  });
</script>

<div>
  <h3 class="h3">{file.name}</h3>
  {#if file.thumbnail}
    <img alt={file.name} src={file.thumbnail} />
  {/if}
  <canvas bind:this={el}></canvas>
</div>
