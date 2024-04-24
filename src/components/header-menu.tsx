import { useCallback, useState } from "react";

import { Button } from "./ui/button";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "./ui/command";

// TODO: get list of mantrams from proper source
const MANTRAMS = [
  { title: "解結咒", pinyin: "jie jie zhou" },
  { title: "往生淨土神咒", pinyin: "wangsheng jingtu shen zhou" },
] as const;

export function HeaderMenu() {
  const [open, setOpen] = useState(false);

  const selectItem = useCallback((value: string) => {
    // TODO
    // console.log(`item selected: ${value}`);
    setOpen(false);
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
            {MANTRAMS.map((mantram) => (
              <CommandItem
                key={mantram.pinyin}
                value={mantram.pinyin}
                onSelect={selectItem}
                className="text-base"
              >
                {mantram.title} ({mantram.pinyin})
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
}
