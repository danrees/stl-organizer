<script lang="ts">
  import { saveLibrary } from "$lib/stl-library";
  import { open } from "@tauri-apps/api/dialog";
  import { path } from "@tauri-apps/api";
  import { modalStore, type ModalSettings } from "@skeletonlabs/skeleton";

  let name: string | undefined;
  let libPath: string | null;
  const modalSettings: ModalSettings = {
    type: "alert",
    title: "Loading",
    body: "Loading ...",
  };

  const dirPicker = async () => {
    modalStore.trigger(modalSettings);
    const resp = await open({ directory: true });
    if (Array.isArray(resp)) {
      return;
    }
    modalStore.close();
    libPath = resp;
    name = resp?.split(path.sep).at(-1);
  };

  const save = async (name: string, path: string) => {
    await saveLibrary(name, path);
    name = "";
    libPath = "";
  };
</script>

<div class="space-y-5">
  <input class="input" type="text" bind:value={name} />
  <button
    class="btn variant-soft-primary"
    on:click={async () => await dirPicker()}
    name="Directory">Directory</button
  >
</div>

<div>
  <button
    class="btn variant-soft-primary"
    on:click={async () => {
      if (name && libPath) {
        await save(name, libPath);
      }
    }}>Save</button
  >
</div>
