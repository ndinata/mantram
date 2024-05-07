import { getCollection } from "astro:content";

/** Returns all items in the "posts" collection sorted by priority (default: desc). */
export async function getPostsCollectionByPriority(
  ord: "asc" | "desc" = "desc",
) {
  return (await getCollection("posts")).toSorted((a, b) => {
    return ord === "asc"
      ? a.data.priority - b.data.priority
      : b.data.priority - a.data.priority;
  });
}
