import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";
import { defineConfig } from "vitest/config";

const vitestBrowserConditionPlugin = {
  name: "vite-plugin-vitest-browser-condition",
  config({ resolve }) {
    if (process.env.VITEST) {
      resolve.conditions.unshift("browser");
    }
  },
};

// https://vite.dev/config/
export default defineConfig({
  clearScreen: true,
  plugins: [vitestBrowserConditionPlugin as any, svelte()],
  build: {
    sourcemap: true,
    minify: true,
    cssMinify: true,
    reportCompressedSize: false,
  },
  resolve: {
    alias: {
      "@lib": path.resolve(__dirname, "src/lib"),
    },
  },
});
