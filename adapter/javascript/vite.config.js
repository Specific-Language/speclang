import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      name: "speclang",
      fileName: (format) => "index" + (format === "es" ? ".mjs" : ".js"),
    },
    rollupOptions: {
      external: ["typescript"],
    },
  },
});
