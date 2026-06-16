# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Tauri 2 desktop app ("aivision") — React 18 + TypeScript + Tailwind CSS v4 + react-i18next. The frontend is a settings app with a recording overlay, built to run against a **mock backend layer** rather than a real Tauri command surface. App UI translation is restricted to **English (en) and Russian (ru)** only.

## Commands

Package manager is **bun** (Tauri's `beforeDevCommand`/`beforeBuildCommand` call `bun run`).

```bash
bun install                # install deps

bun run dev                # Vite dev server at http://localhost:1420 (primary way to view the UI)
bun run build              # tsc (type-check, no emit) + vite build -> dist/
bun run preview            # preview the production build

bun tauri dev              # full Tauri app (Rust shell + webview); frontend still uses mocks
bun tauri build            # production bundle
```

There is no lint or test script configured.

> **Viewing the UI:** the simplest path is `bun run dev` then open `http://localhost:1420`. The recording overlay is a second Vite entry — open `http://localhost:1420/src/overlay/index.html` to see it.

## Architecture

### Two frontend entry points
`vite.config.ts` defines a multi-page build: `index.html` (main settings app) and `src/overlay/index.html` (recording overlay).

### The mock layer (the central concept)
The UI reaches its backend through two narrow funnels: `@/bindings` (a tauri-specta-generated `commands` object + TS types) and a handful of `@tauri-apps/*` plugin imports. Instead of a real backend, those funnels are intercepted by mocks, so the UI renders and is interactive using in-memory state:

- **`src/bindings.ts`** — the generated bindings, kept as-is. Every `commands.*` call funnels through `invoke()` imported from `@tauri-apps/api/core`.
- **`vite.config.ts` → `resolve.alias`** redirects each Tauri module (`@tauri-apps/api/core|event|app|webviewWindow`, `plugin-os|opener|dialog|fs|updater|process`) to a stub under `src/mock/`. TypeScript still resolves the **real types** from `node_modules`; only runtime is swapped.
- **`src/mock/core.ts`** implements `invoke(cmd, args)` as a switch: getter commands (`get_app_settings`, `get_available_models`, `has_any_models_available`, `get_available_microphones`, …) return canned data from **`src/mock/state.ts`**; all `change_*`/`set_*`/`update_*` mutators are no-ops (the Zustand stores already apply changes optimistically to local state); `download_model` drives the event bus.
- **`src/mock/bus.ts`** is an in-memory event bus backing `listen`/`emit` (`src/mock/event.ts`), so UI flows that react to backend events work here too — e.g. model download progress/complete (consumed by `src/stores/modelStore.ts`) and the overlay's mic levels (`src/overlay/demo.ts` seeds these).

The Zustand stores (`src/stores/settingsStore.ts`, `modelStore.ts`) and the `useSettings` hook run unchanged against the mock `commands`.

### Directory map

- `src/components/**` — UI primitives (`ui/`), the seven settings sections (`settings/general|advanced|history|post-processing|debug|about|models`), `Sidebar`, `footer`, `onboarding`, `model-selector`, `icons`, etc.
- `src/stores/`, `src/hooks/`, `src/lib/` (incl. `lib/utils/rtl.ts`, `lib/constants/languages.ts`).
- `src/i18n/` — react-i18next; `locales/{en,ru}/translation.json` only; `languages.ts` trimmed to `en` + `ru`.
- `src/mock/` — the mock layer described above.
- `src/App.css` — Tailwind v4 `@import "tailwindcss"` + `@theme` design tokens (light/dark via `prefers-color-scheme`); the visual foundation the components' classes depend on.

### Rust backend (`src-tauri/`)
The minimal starter (`greet` command + `tauri-plugin-opener`). It is **not used by the frontend** — everything is mocked — so no Rust commands/plugins need to be registered. `tauri.conf.json` keeps `productName: aivision`, an 800×600 window, and bun before-commands.

## Conventions / gotchas

- **To extend the UI with another component**, add it; it will work as long as the mock layer satisfies its `commands.*`/plugin calls. If it needs a new getter that returns data, add a case to `src/mock/core.ts`'s `invoke` switch — otherwise unknown commands fall through to the no-op default.
- **Do not wire up real Tauri Rust plugins.** The frontend is intentionally backed by the mock layer. Adding real backend behavior is out of scope unless explicitly requested.
- **`@` → `src`** path alias; **`@tauri-apps/*` → `src/mock/*`** (runtime only).
- **i18n = en/ru only** for the app UI. The separate *transcription-language* dropdown (`src/components/settings/LanguageSelector.tsx` + `src/lib/constants/languages.ts`) keeps a full ~99-language list as mock feature data — it is not UI translation.
- TypeScript is strict but `noUnusedLocals`/`noUnusedParameters` are off (the generated `bindings.ts` needs that). `bun run build` runs `tsc` — keep it clean.
- Vite dev server is pinned to **port 1420** (`strictPort`); ignore `src-tauri/` from HMR.
- **Not a git repository** (no `.git`).
