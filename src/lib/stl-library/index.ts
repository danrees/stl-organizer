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

export async function getLibrary(id: string): Promise<STLLibrary> {
  return await invoke("get_library", { id: id });
}

export async function deleteLibrary(id: string) {
  await invoke("delete_library", { id: id });
}

export async function scanLibrary(id: string, extension: string) {
  await invoke("scan_library_command", { id: id, extension: extension });
}

export interface STLFile {
  id: string;
  name: string;
  extension: string;
  path: string;
  tags: string[];
}
