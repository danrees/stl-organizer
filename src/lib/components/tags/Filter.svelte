<script lang="ts">
  import { InputChip } from "@skeletonlabs/skeleton";
  export let tagFilters: string[] | undefined = [];
</script>

<div>
  <h3 class="h3">Filter by Tags</h3>
  <InputChip
    name="tag-filter"
    placeholder="Enter tags"
    bind:value={tagFilters}
  />
</div>
