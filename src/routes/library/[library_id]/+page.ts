import type { PageLoad } from "./$types";

export const load = (({ params }) => {
  const library_id = params.library_id;
  return {
    library_id: library_id,
  };
}) satisfies PageLoad;
