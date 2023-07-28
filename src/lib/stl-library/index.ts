import { invoke } from "@tauri-apps/api/tauri";

export interface STLLibrary {
  id: string;
  name: string;
  path: string;
}

export async function saveLibrary(name: string, path: string) {
  await invoke("save_library", { name: name, path: path });
}

export async function listLibraries(): Promise<STLLibrary[]> {
  return await invoke("list_libraries");
}

export async function deleteLibrary(id: string) {
  await invoke("delete_library", { id: id });
}

export async function scanLibrary(id: string, extension: string) {
  await invoke("scan_library", { id: id, extension: extension });
}
