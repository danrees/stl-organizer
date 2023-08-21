import { invoke } from "@tauri-apps/api/tauri";
import { writable } from "svelte/store";

export interface STLLibrary {
  id: string;
  name: string;
  path: string;
}

export type Filter = {
  selected: boolean;
} & STLLibrary;

export interface E {
  message: string;
}

export const filterStore = writable<Filter[]>([]);

export function isE(obj: unknown): obj is E {
  return (obj as E)?.message !== undefined;
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
