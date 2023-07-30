<script lang="ts">
  import Container from "$lib/components/layout/Container.svelte";
  import Create from "$lib/components/library/Create.svelte";
  import List from "$lib/components/library/List.svelte";
  import { listLibraries, type STLLibrary } from "$lib/stl-library";

  const libraries: Promise<STLLibrary[]> = listLibraries();
</script>

<Container>
  {#await libraries}
    <div>Loading ...</div>
  {:then libs}
    <h3 class="h3">Libraries</h3>
    <List libraries={libs} />
  {/await}
  <h3 class="h3">Add Library</h3>
  <Create />
</Container>
