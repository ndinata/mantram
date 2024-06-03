import react from "@astrojs/react";
import tailwind from "@astrojs/tailwind";
import { defineConfig } from "astro/config";
import wasm from "vite-plugin-wasm";

// https://astro.build/config
export default defineConfig({
  integrations: [react(), tailwind({ applyBaseStyles: false })],
  vite: {
    plugins: [wasm()],
  },
  site: "https://mantram.blog",
  prefetch: true,
});
