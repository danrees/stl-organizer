import { invoke } from "@tauri-apps/api";

export interface Tag {
  id: string;
  value: string;
}
export interface STLFile {
  id: string;
  name: string;
  extension: string;
  path: string;
  tags: Tag[];
}

export async function listFiles(): Promise<STLFile[]> {
  return await invoke("list_files", {});
}
