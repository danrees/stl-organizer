<script lang="ts">
  import { deleteTag, listTags } from "$lib/stl-file";
  import TagList from "$lib/components/tags/TagList.svelte";

  let tags = listTags();
</script>

{#await tags}
  <div>Loading...</div>
{:then tags}
  <TagList {tags} {deleteTag} updateTags={listTags} />
{/await}
