import { getFile } from "$lib/stl-file";
import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
  const file_id = params.file_id;

  const file = await getFile(file_id);
  return {
    file: file,
  };
}) satisfies PageLoad;
