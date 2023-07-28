<script lang="ts">
  import {listLibraries, type STLLibrary} from "$lib/stl-library"


  const libraries: Promise<STLLibrary[]> = listLibraries();
</script>


<div>
  {#await libraries}
<div>Loading</div>
    {:then libs}    

  <ul>
      {#each libs as lib}
        
    <li>{lib.name}</li>
      {/each}
  </ul>
  {/await}
</div>

