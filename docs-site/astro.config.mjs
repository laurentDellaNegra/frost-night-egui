import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";

export default defineConfig({
  site: "https://laurentdellanegra.github.io",
  base: "/frost-night-egui",
  integrations: [mdx()],
});
