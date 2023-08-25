import { invoke } from "@tauri-apps/api";

export type TagReference = {
  id: string;
};

export type Tag = {
  id?: string;
  value: string;
};

export type ETag = Tag | TagReference;

export type STLFile = {
  id: string;
  name: string;
  extension: string;
  path: string;
  tags: Tag[];
  thumbnail?: string;
};

export function getId(file: STLFile): string | undefined {
  return file.id.split(":")?.at(-1);
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

export async function getFile(id: string): Promise<STLFile> {
  return await invoke("get_file", { id: id });
}
