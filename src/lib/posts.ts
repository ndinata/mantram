import { getCollection } from "astro:content";

/** Returns all items in the "posts" collection sorted by priority (default: desc). */
export async function getPostsCollectionByPriority(
  ord: "asc" | "desc" = "desc",
) {
  return (await getCollection("posts"))
    .toSorted((a, b) => {
      return ord === "asc"
        ? a.data.priority - b.data.priority
        : b.data.priority - a.data.priority;
    })
    .map((item, index) => ({ ...item, position: adjustPosition(index) }));
}

/**
 * Sets the rule for post numbering/positions.
 *
 * This allows for custom numbering rules instead of the default (index + 1),
 * e.g. '3b' instead of a '4' for post positions.
 */
function adjustPosition(index: number): string {
  switch (index + 1) {
    case 4:
      return "3b";
    default:
      return (index + 1).toString();
  }
}
