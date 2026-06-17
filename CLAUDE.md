## Project

Tauri 2 desktop app ("aivision") — React 18 + TypeScript + Tailwind CSS v4 + react-i18next + Zustand. A settings UI with a recording overlay, running against a **mock backend layer** instead of a real Tauri command surface. UI translation is **English (en) and Russian (ru)**.

## Commands

Package manager is **bun** (Tauri's `beforeDevCommand`/`beforeBuildCommand` call `bun run`).

```bash
bun install
bun run dev          # Vite dev server at http://localhost:1420 (main UI)
bun run build        # tsc (noEmit type-check) + vite build -> dist/
bun tauri dev        # full Tauri app; frontend still uses mocks
bun tauri build      # production bundle
```

## Architecture

**Two Vite entry points** (`vite.config.ts`): `index.html` (main app) + `src/overlay/index.html` (recording overlay).

**Mock layer (central concept).** The UI's backend calls go through `@/bindings` (a tauri-specta-generated `commands` object + TS types) and `@tauri-apps/*` plugin imports. `vite.config.ts → resolve.alias` redirects every `@tauri-apps/*` module (`api/core|event|app|webviewWindow`, `plugin-os|opener|dialog|fs|updater|process`) to a stub under `src/mock/` — a **runtime** swap only; TypeScript still resolves the real types from `node_modules`. In `src/mock/core.ts`, `invoke(cmd, args)` is a switch: getters return canned data from `src/mock/state.ts`; `change_*`/`set_*`/`update_*` mutators are no-ops (the Zustand stores already apply changes optimistically); `download_model` drives the event bus. `src/mock/bus.ts` + `event.ts` back `listen`/`emit` (download progress, overlay mic levels). The Zustand stores (`src/stores/{settingsStore,modelStore}.ts`) and the `useSettings` hook run unchanged against the mock `commands`.

**Layout:** `src/components/**` (UI primitives `ui/`, plus 7 settings sections in `settings/{general,advanced,history,post-processing,debug,about,models}`), `src/stores/`, `src/hooks/`, `src/lib/`, `src/i18n/` (`locales/{en,ru}/translation.json`), `src/mock/`, `src/App.css` (Tailwind v4 `@theme` tokens, light/dark via `prefers-color-scheme`). **`src-tauri/`** is the minimal starter (`greet` + `tauri-plugin-opener`) — not used by the frontend, so nothing needs registering.

## Conventions / gotchas

- **Adding a component** works as long as the mock layer satisfies its `commands.*`/plugin calls. A new getter returning data needs a case in `src/mock/core.ts`'s `invoke` switch; unknown commands fall through to the no-op default.
- **Do not wire up real Tauri Rust plugins.** The frontend is intentionally mocked; real backend is out of scope unless explicitly requested.
- **Aliases:** `@` → `src`; `@tauri-apps/*` → `src/mock/*` (runtime only).
- **i18n = en/ru only** for the UI. The separate transcription-language dropdown (`settings/LanguageSelector.tsx` + `lib/constants/languages.ts`) keeps a full ~99-language list as mock feature data — it is not UI translation.
- `tsc` is strict but `noUnusedLocals`/`noUnusedParameters` are off (`bindings.ts` needs that); keep `bun run build` clean. Vite dev server is pinned to **port 1420** (`strictPort`); `src-tauri/` is ignored from HMR.
