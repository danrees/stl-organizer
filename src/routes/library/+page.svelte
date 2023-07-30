<script lang="ts">
    import Container from "$lib/components/layout/Container.svelte";
    import Create from "$lib/components/library/Create.svelte";
    import List from "$lib/components/library/List.svelte";
    import { listLibraries, type STLLibrary } from "$lib/stl-library";

    const libraries: Promise<STLLibrary[]> = listLibraries();
</script>

<Container>
  {#await libraries}
  <div>Loading ... </div> 
  {:then libs} 
    
  <List libraries={libs}/>
  {/await}
<Create />
</Container>
