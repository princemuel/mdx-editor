/// <reference types="vitest" />
/// <reference types="vite/client" />

import react from "@vitejs/plugin-react-swc";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: { alias: [{ find: "@/", replacement: "/src/" }] },

  test: {
    globals: true,
    name: "happy-dom",
    environment: "happy-dom",
    setupFiles: "./src/utils/tests.setup.ts",
    reporters: ["default", "html"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
    },
    // you might want to disable it, if you don't have tests that rely on CSS
    // since parsing CSS is slow
    // css: true,
  },
});
