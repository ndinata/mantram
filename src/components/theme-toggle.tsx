import * as React from "react";
import { Moon, Sun } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
} from "@/components/ui/dropdown-menu";

const STORAGE_KEY_THEME = "theme";

export function ThemeToggle() {
  const [open, setOpen] = React.useState(false);
  const [theme, setThemeState] = React.useState<"light" | "dark" | "system">(
    () => {
      if (
        typeof window !== "undefined" &&
        localStorage.getItem(STORAGE_KEY_THEME)
      ) {
        return localStorage.getItem(STORAGE_KEY_THEME) as "light" | "dark";
      } else {
        return "system";
      }
    },
  );

  const selectTheme = React.useCallback((newTheme: string) => {
    if (newTheme === "system") {
      setThemeState("system");
      document.documentElement.classList[
        window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "add"
          : "remove"
      ]("dark");
      localStorage.removeItem(STORAGE_KEY_THEME);
    } else {
      setThemeState(newTheme as "light" | "dark");
      document.documentElement.classList[
        newTheme === "dark" ? "add" : "remove"
      ]("dark");
      localStorage.setItem(STORAGE_KEY_THEME, newTheme);
    }
  }, []);

  return (
    <DropdownMenu open={open} onOpenChange={setOpen}>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          size="icon"
          className="h-8 w-8"
          onClick={() => setOpen((o) => !o)}
        >
          <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuRadioGroup value={theme} onValueChange={selectTheme}>
          <DropdownMenuRadioItem value="light">Light</DropdownMenuRadioItem>
          <DropdownMenuRadioItem value="dark">Dark</DropdownMenuRadioItem>
          <DropdownMenuRadioItem value="system">System</DropdownMenuRadioItem>
        </DropdownMenuRadioGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
