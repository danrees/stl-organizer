<script lang="ts">
  import type{  STLLibrary} from "$lib/stl-library"
  import {appWindow, } from "@tauri-apps/api/window"
    import { onDestroy, onMount } from "svelte";

  export let libraries: STLLibrary[];
  let unlisten: () => void;
  onMount(async () => {
    unlisten = await appWindow.listen<STLLibrary>("library-save", ({payload}) => {
      libraries = [...libraries, payload] 
    });
  })
  onDestroy(async () => {
    if (unlisten) {
      unlisten()
    }
  })
</script>


<div>

  <ul>
      {#each libraries as lib}
        
        <li>
          <div>

          <span class="text-lg">{lib.name}</span>
            <span class="text-sm text-primary-400">{lib.path}</span>
          </div>
        </li>
      {/each}
  </ul>
</div>

