import { invoke } from "@tauri-apps/api";

export async function loadSTL(path: string): Promise<Uint8Array> {
  const encoded: string = await invoke("load_stl", { path: path });
  const b = atob(encoded);
  let bytes = Uint8Array.from(b, (m) => m.charCodeAt(0));
  return bytes;
}

// TODO: Try to make this function more robust
export function saveThumbnail(id: string, b: Blob) {
  try {
    const reader = new FileReader();

    reader.onloadend = async () => {
      try {
        await invoke("save_thumbnail", { id: id, image: reader.result });
      } catch (e) {
        console.log(e);
      }
    };
    reader.readAsDataURL(b);
  } catch (e) {
    console.log(e);
  }
}
