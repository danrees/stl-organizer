<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { scanLibrary, type STLLibrary } from "$lib/stl-library";
  import type { STLFile } from "$lib/stl-file";
  import { toastStore } from "@skeletonlabs/skeleton";

  let unlisten: () => void;

  let lines: string[] = [];
  export let library: STLLibrary;
  export let extension: string = "stl";

  $: text = lines.join("\n");
  onMount(async () => {
    unlisten = await appWindow.listen<STLFile>(
      "scanned-file",
      ({ payload }) => {
        lines = [...lines, payload.name];
      },
    );
  });

  onDestroy(async () => {
    if (unlisten) {
      unlisten();
    }
  });

  async function scan() {
    console.log(`Scanning ${library.id}`);
    try {
      const id = library.id.split(":").at(-1);
      if (id) {
        await scanLibrary(id, extension);
      } else {
        throw "invalid library id";
      }
    } catch (e: any) {
      toastStore.trigger({
        message: e,
      });
    }

    // await emit("scan-library", { ...library });
  }
</script>

<button class="btn variant-filled" on:click={async () => await scan()}
  >Scan</button
>

<div class="overflow-x-scroll max-h-screen w-full">
  <pre class="pre">
   {text}
  </pre>
</div>
