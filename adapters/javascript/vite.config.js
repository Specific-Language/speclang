import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, "src/speclang.ts"),
      name: "speclang",
      fileName: (format) => "speclang" + (format === "es" ? ".mjs" : ".js"),
    },
    rollupOptions: {
      external: ["typescript"],
    },
  },
});
