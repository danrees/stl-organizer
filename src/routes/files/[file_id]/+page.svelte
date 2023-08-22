<script lang="ts">
  import Details from "$lib/components/files/Details.svelte";
  import type { PageData } from "./$types";

  export let data: PageData;

  $: file = data.file;
</script>

<Details {file} />
