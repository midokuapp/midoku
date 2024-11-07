import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";

const TAURI_ENV_DEBUG = Deno.env.get("TAURI_ENV_DEBUG");
const TAURI_DEV_HOST = Deno.env.get("TAURI_DEV_HOST");
const TAURI_ENV_PLATFORM = Deno.env.get("TAURI_ENV_PLATFORM");

export default defineConfig({
  plugins: [react()],
  css: {
    postcss: {
      plugins: [tailwindcss, autoprefixer],
    },
  },
  clearScreen: false,
  server: {
    strictPort: true,
    host: TAURI_DEV_HOST || false,
    port: 5173,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target: TAURI_ENV_PLATFORM == "windows" ? "chome105" : "safari13",
    minify: !TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!TAURI_ENV_DEBUG,
  },
});
