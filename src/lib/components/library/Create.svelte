<script lang="ts">
  import {saveLibrary} from "$lib/stl-library";
  import {open} from "@tauri-apps/api/dialog";
  import {path} from "@tauri-apps/api";
    import { FileButton } from "@skeletonlabs/skeleton";

  let name: string | undefined;
  let libPath: string | null;

  const dirPicker = async () => {
    const resp = await open({directory: true, multiple: false});
    if (Array.isArray(resp)) {
      return
    }
    libPath = resp;
    name = resp?.split(path.sep).at(-1);
  }

  const save = async (name: string, path: string) => {
    await saveLibrary(name, path);
  }
</script>

<div>
  <FileButton on:click={async () => dirPicker()} name="Directory"></FileButton> 
  <input type="text" bind:value={name}/>
</div>

<div><button on:click={async () => { 
  if(name && libPath) {
  await save(name, libPath) 
  }
}}>Save</button></div>
