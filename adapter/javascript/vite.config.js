// vite.config.js
const path = require("path");
const { defineConfig } = require("vite");

module.exports = defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "src/main.ts"),
      name: "speclang",
      fileName: (format) => `speclang.${format}.js`,
    },
    rollupOptions: {
      external: ["typescript"],
      output: {
        exports: "named",
      },
    },
  },
});
