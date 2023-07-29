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
        
        <li>
          <div>

          <span class="text-lg">{lib.name}</span>
            <span class="text-sm text-primary-400">{lib.path}</span>
          </div>
        </li>
      {/each}
  </ul>
  {/await}
</div>

