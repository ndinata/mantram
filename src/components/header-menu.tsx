import { useCallback, useState } from "react";
import { navigate } from "astro:transitions/client";
import type { CollectionEntry } from "astro:content";

import { Button } from "./ui/button";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./ui/command";

type Props = {
  posts: CollectionEntry<"posts">[];
};

export function HeaderMenu({ posts }: Props) {
  const [open, setOpen] = useState(false);

  const selectItem = useCallback((value: string) => {
    setOpen(false);
    navigate(`/posts/${value}`);
  }, []);

  return (
    <>
      <Button
        variant="outline"
        className="relative h-8 w-full justify-start rounded-md text-sm font-normal text-muted-foreground shadow-none md:w-40 lg:w-64"
        onClick={() => setOpen(true)}
      >
        <span className="inline-flex">Search...</span>
      </Button>

      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput placeholder="Search for mantram..." />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandGroup heading="Mantrams">
            {posts.map((mantram, i) => (
              <CommandItem
                key={i}
                value={mantram.slug}
                onSelect={selectItem}
                className="text-base"
              >
                {mantram.data.title} ({mantram.data.subtitle})
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
}
