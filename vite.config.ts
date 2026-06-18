import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { resolve } from "path";

const host = process.env.TAURI_DEV_HOST;
const src = resolve(__dirname, "./src");

// The Tauri CLI sets TAURI_ENV_PLATFORM while running `tauri dev`/`tauri build`
// (via beforeDevCommand/beforeBuildCommand). Plain `bun run dev` does not, so we
// use it to switch between the real Tauri APIs (Rust backend) and the mock layer.
const isTauri = !!process.env.TAURI_ENV_PLATFORM;

// Mock stubs that stand in for the real @tauri-apps/* packages during web-only
// development (`bun run dev` in a browser, with no Rust backend present).
const tauriAliases = [
  { find: "@tauri-apps/api/core", replacement: resolve(src, "mock/core.ts") },
  { find: "@tauri-apps/api/event", replacement: resolve(src, "mock/event.ts") },
  { find: "@tauri-apps/api/app", replacement: resolve(src, "mock/app.ts") },
  {
    find: "@tauri-apps/api/webviewWindow",
    replacement: resolve(src, "mock/webviewWindow.ts"),
  },
  { find: "@tauri-apps/plugin-os", replacement: resolve(src, "mock/os.ts") },
  {
    find: "@tauri-apps/plugin-opener",
    replacement: resolve(src, "mock/opener.ts"),
  },
  {
    find: "@tauri-apps/plugin-dialog",
    replacement: resolve(src, "mock/dialog.ts"),
  },
  { find: "@tauri-apps/plugin-fs", replacement: resolve(src, "mock/fs.ts") },
  {
    find: "@tauri-apps/plugin-updater",
    replacement: resolve(src, "mock/updater.ts"),
  },
  {
    find: "@tauri-apps/plugin-process",
    replacement: resolve(src, "mock/process.ts"),
  },
];

export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],

  resolve: {
    // Under real Tauri, let imports resolve to the installed @tauri-apps/*
    // packages so they reach the Rust command surface. Otherwise (web dev),
    // redirect every @tauri-apps/* import to the in-memory mocks in src/mock.
    alias: isTauri
      ? [{ find: "@", replacement: src }]
      : [...tauriAliases, { find: "@", replacement: src }],
  },

  // Multiple entry points for main app and recording overlay
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        overlay: resolve(__dirname, "src/overlay/index.html"),
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
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
      ignored: ["**/src-tauri/**"],
    },
  },
}));
