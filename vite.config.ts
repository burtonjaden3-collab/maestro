/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // In Tauri dev, Rust build artifacts can explode in file count (target/, AppDir, etc.).
      // Watching them causes ENOSPC on Linux and can also create UI jank from heavy polling.
      ignored: [
        "**/.git/**",
        "**/dist/**",
        "**/src-tauri/**",
        "**/target/**",
        "**/website/_site/**",
      ],
    },
  },
  test: {
    globals: true,
    environment: "happy-dom",
    setupFiles: ["./src/test/setup.ts"],
  },
}));
