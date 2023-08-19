import { invoke } from "@tauri-apps/api";

export type TagReference = {
  id: string;
};

export type Tag = {
  id?: string;
  value: string;
};

export type ETag = Tag | TagReference;

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

export async function listTags(): Promise<Tag[]> {
  return await invoke("list_tags", {});
}

export async function deleteTag(id: string): Promise<Tag> {
  return await invoke("delete_tag", { id: id });
}
