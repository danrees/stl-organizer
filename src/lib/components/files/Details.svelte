<script lang="ts">
  import { getId, type STLFile } from "$lib/stl-file";
  import { loadSTL, saveThumbnail } from "$lib/preview";
  import { onMount } from "svelte";
  import { createScene, saveImage } from "$lib/scene";
  import { isE } from "$lib/stl-library";
  import { toastStore } from "@skeletonlabs/skeleton";
  import { browser } from "$app/environment";

  export let file: STLFile;
  export let previewWidth: number = 800;
  export let previewHeight: number = 600;
  let el: HTMLCanvasElement;

  function createThumbnail() {
    el.toBlob(async (blob) => {
      const id = getId(file);
      if (blob && id) {
        try {
          console.log(blob.size);
          saveThumbnail(id, blob);
        } catch (e) {
          if (isE(e)) {
            toastStore.trigger({ message: e.message });
          }
        }
      }
    }, "image/png");
  }

  onMount(async () => {
    if (browser) {
      try {
        const data = await loadSTL(file.path);
        const animate = createScene(el, data.buffer);
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
  <canvas bind:this={el} width={previewWidth} height={previewHeight}></canvas>
  <button on:click={() => createThumbnail()}>Thumbnail</button>
</div>
