import { z, defineCollection } from "astro:content";

const postsCollection = defineCollection({
  type: "content",
  schema: z.object({
    /** Title of the mantram. */
    title: z.string(),
    /** Subtitle of the mantram title. */
    subtitle: z.string(),
    /** Pinyin of the mantram title. */
    titlePinyin: z.string().optional(),
    /** Date the mantram was published. */
    publishDate: z.date(),
    /** Last time the mantram was updated. */
    lastUpdatedDate: z.date(),
    /** Optional key useful for e.g. ordering of posts. */
    priority: z.number().optional().default(0),
    /** Additional context to the mantram. */
    context: z
      .object({
        title: z.string(),
        body: z.string(),
      })
      .optional(),
  }),
});

export const collections = {
  posts: postsCollection,
};
