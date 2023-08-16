<script lang="ts">
  import type { Tag } from "$lib/stl-file";
  import {
    modalStore,
    toastStore,
    type ModalSettings,
  } from "@skeletonlabs/skeleton";

  export let tags: Tag[];
  export let deleteTag: (id: string) => Promise<Tag>;
  export let updateTags: () => Promise<Tag[]>;
  interface E {
    message: string;
  }
  function isE(obj: unknown): obj is E {
    return (obj as E)?.message !== undefined;
  }
  const modalSettings: ModalSettings = {
    type: "confirm",
    title: "Confirm Delete",
    body: "Are you sure you want to delete this tag? This will remove this tag from all files as well",
    response: (r: boolean) => {
      if (r) {
        console.log();
      }
    },
  };
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
                await deleteTag(tag.id.split(":").at(-1) || "");
              } catch (e) {
                if (isE(e)) {
                  toastStore.trigger({ message: e.message });
                } else {
                  throw e;
                }
              }
            }
            updateTags();
          }}>Delete</button
        >
      </span>
    </li>
  {/each}
</ul>
