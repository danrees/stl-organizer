<script lang="ts">
  import Scan from "$lib/components/library/Scan.svelte";
  import type { STLLibrary } from "$lib/stl-library";
  import { getLibrary } from "$lib/stl-library";
  import type { PageData } from "./$types";

  export let data: PageData;

  let libraryToBe: Promise<STLLibrary>;
  $: {
    libraryToBe = getLibrary(data.library_id);
  }
</script>

{#await libraryToBe}
  <p>Loading ...</p>
{:then library}
  <Scan {library} />
{/await}
