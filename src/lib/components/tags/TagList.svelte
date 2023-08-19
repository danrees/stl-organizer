<script lang="ts">
  import type { Tag } from "$lib/stl-file";
  import { isE } from "$lib/stl-library";
  import {
    modalStore,
    toastStore,
    type ModalSettings,
  } from "@skeletonlabs/skeleton";
  import { emit } from "@tauri-apps/api/event";

  export let tags: Tag[];
  export let deleteTag: (id: string) => Promise<Tag>;
  export let deleteTagChannel: string;

  const confirmDelete = async () => {
    return new Promise<boolean>((resolve) => {
      const modalSettings: ModalSettings = {
        type: "confirm",
        title: "Confirm Delete",
        body: "Are you sure you want to delete this tag?",
        response: (r: boolean) => {
          resolve(r);
        },
      };
      modalStore.trigger(modalSettings);
    });
  };
</script>

<ul class="list">
  {#each tags as tag}
    <li>
      <span>
        {tag.value}
      </span>
      <span>
        <button
          class="btn variant-filled"
          on:click={async () => {
            const willDelete = await confirmDelete();
            if (willDelete) {
              try {
                if (!tag.id) {
                  throw Error("tag has no id");
                }
                await deleteTag(tag?.id.split(":").at(-1) || "");
                emit(deleteTagChannel, { tag: tag });
              } catch (e) {
                if (isE(e)) {
                  toastStore.trigger({ message: e.message });
                } else {
                  throw e;
                }
              }
            }
          }}>Delete</button
        >
      </span>
    </li>
  {/each}
</ul>
