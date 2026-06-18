## Project

Tauri 2 desktop app ("aivision") — React 18 + TypeScript + Tailwind CSS v4 + react-i18next + Zustand. A settings UI with a recording overlay. The UI talks to a **real Rust command surface** under `tauri dev`/`build`; a **mock layer** is still used for web-only dev (`bun run dev`). UI translation is **English (en) and Russian (ru)**.

## Commands

Package manager is **bun** (Tauri's `beforeDevCommand`/`beforeBuildCommand` call `bun run`).

```bash
bun install
bun run dev          # Vite dev server at http://localhost:1420 (web-only, uses mocks)
bun run build        # tsc (type-check) + vite build -> dist/
bun tauri dev        # full Tauri app; frontend talks to the REAL Rust backend
bun tauri build      # production bundle
```

## Architecture

**Two Vite entry points** (`vite.config.ts`): `index.html` (main app) + `src/overlay/index.html` (recording overlay).

**Mock layer (web dev only).** The UI's backend calls go through `@/bindings` (a tauri-specta-generated `commands` object + TS types) and `@tauri-apps/*` plugin imports. `vite.config.ts → resolve.alias` redirects every `@tauri-apps/*` module to a stub under `src/mock/` — but this alias is now **conditional on `TAURI_ENV_PLATFORM`**: active only for `bun run dev` (web, no Rust), and **off** under `tauri dev`/`build` so the real `@tauri-apps/*` packages reach the Rust backend. TypeScript always resolves the real types from `node_modules`. In `src/mock/core.ts`, `invoke(cmd, args)` is a switch: getters return canned data from `src/mock/state.ts`; `change_*`/`set_*`/`update_*` mutators are no-ops (the Zustand stores apply changes optimistically); `download_model` drives the event bus. `src/mock/bus.ts` + `event.ts` back `listen`/`emit`. The Zustand stores (`src/stores/{settingsStore,modelStore}.ts`) and the `useSettings` hook run unchanged against either the mock `commands` or the real Rust commands.

**Real backend (`src-tauri/`).** `lib.rs::run()` registers plugins (`opener`, `os`, `dialog`, `fs`, `process`, `updater`, `log`) and all **100 commands** from `bindings.ts` via `generate_handler!`, split across `src/commands/{settings,shortcuts,models,audio,post_process,history,system}.rs`. Each command logs `[cmd] <name> …` via `tauri-plugin-log` (Stdout + LogDir) and returns canned data mirroring the mock (`state.rs`, `types.rs`); `download_model` emits the 6-event download sequence from a spawned thread (`events.rs`). State is a managed `Mutex<AppState>`. No tauri-specta — `bindings.ts` is kept verbatim; its per-command wrapper synthesizes the `{status,data/error}` envelope, so plain `#[tauri::command]` returning `Result<T,String>` matches.

**Layout:** `src/components/**` (UI primitives `ui/`, plus 7 settings sections in `settings/{general,advanced,history,post-processing,debug,about,models}`), `src/stores/`, `src/hooks/`, `src/lib/`, `src/i18n/` (`locales/{en,ru}/translation.json`), `src/mock/`, `src/App.css` (Tailwind v4 `@theme` tokens, light/dark via `prefers-color-scheme`).

## Conventions / gotchas

- **Adding a component** works as long as the mock layer satisfies its `commands.*`/plugin calls. A new getter returning data needs a case in `src/mock/core.ts`'s `invoke` switch; unknown commands fall through to the no-op default.
- **Do not wire up real Tauri Rust plugins.** The frontend is intentionally mocked; real backend is out of scope unless explicitly requested.
- **Aliases:** `@` → `src`; `@tauri-apps/*` → `src/mock/*` (runtime only).
- **i18n = en/ru only** for the UI. The separate transcription-language dropdown (`settings/LanguageSelector.tsx` + `lib/constants/languages.ts`) keeps a full ~99-language list as mock feature data — it is not UI translation.
- `tsc` is strict but `noUnusedLocals`/`noUnusedParameters` are off (`bindings.ts` needs that); keep `bun run build` clean. Vite dev server is pinned to **port 1420** (`strictPort`); `src-tauri/` is ignored from HMR.
