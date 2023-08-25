import { invoke } from "@tauri-apps/api";

export async function loadSTL(path: string): Promise<Uint8Array> {
  const encoded: string = await invoke("load_stl", { path: path });
  const b = atob(encoded);
  let bytes = Uint8Array.from(b, (m) => m.charCodeAt(0));
  return bytes;
}

export function saveThumbnail(id: string, b: Blob) {
  const reader = new FileReader();

  reader.onloadend = async () => {
    await invoke("save_thumbnail", { id: id, data: reader.result });
  };
  reader.readAsDataURL(b);
}
