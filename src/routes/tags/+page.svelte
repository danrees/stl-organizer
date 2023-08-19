<script lang="ts">
  import { deleteTag, listTags, type Tag } from "$lib/stl-file";
  import TagList from "$lib/components/tags/TagList.svelte";
  import { onDestroy, onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { toastStore } from "@skeletonlabs/skeleton";
  import { isE } from "$lib/stl-library";

  let tags: Tag[] = [];
  let unlisten: UnlistenFn;

  onMount(async () => {
    tags = await listTags();
    unlisten = await listen("tags-changed", async () => {
      try {
        tags = await listTags();
      } catch (e) {
        if (isE(e)) {
          toastStore.trigger({ message: e.message });
        }
      }
    });
  });
  onDestroy(async () => {
    unlisten();
  });
</script>

<TagList {tags} {deleteTag} deleteTagChannel="tags-changed" />
