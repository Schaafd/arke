# Arke Technology Stack Alignment with PRD

## 1. Architecture (Local-first, One Core, Thin Shells)
- **Core Engine (Rust)**
  - Handles vault I/O, Markdown parse/link extraction, backlinks, indexing/search, conflict detection, exports, plugin API surface.
  - Compiles to **WASM** for Web PWA; linked natively in desktop/mobile.
- **UI Shell (Web-first)**
  - **SvelteKit** + **CodeMirror 6** + **Tailwind CSS** + **@tanstack/virtual**.
- **Wrappers**
  - **macOS:** Tauri 2.x.
  - **iOS (post-v1 beta):** Capacitor + Swift bridge.

---

## 2. Editor, Markdown, and Callouts
- **Rendering pipeline:**
  - Canonical parse in Rust (**comrak**).
  - Web preview via **unified/remark/rehype** + **KaTeX**.
- **Callouts & Chat Blocks:**
  - Markdown directives (remark-directive).
  - Chat blocks as fenced blocks with sibling JSON for messages.
- **Features:**
  - Split view, outline, backlinks pane, wikilinks, safe rename.
  - Export: HTML (rehype), PDF via Tauri sidecar.

---

## 3. File Model, Storage, and Sync
- **Vaults:** Plain `.md` + `/assets/`.
- **Web:** File System Access API or OPFS fallback.
- **Desktop:** Tauri native FS.
- **iOS:** Files provider via Capacitor.
- **Sync (v1):** OS-level sync (iCloud, Dropbox).
- **Arke Sync (future):** Optional encrypted sync service.

---

## 4. Search, Indexing, and Graph
- **Indexing:**
  - Desktop/iOS: Rust **tantivy**.
  - Web: **FlexSearch** (Worker-based).
- **Backlinks/Graph:**
  - Incremental backlinks map.
  - Graph view via **Cytoscape.js** post-v1.

---

## 5. Plugin System
- **Format:** `manifest.json` (name, version, capabilities).
- **Runtime:** Web Workers / Tauri sidecar.
- **Permissions:** Scoped FS/network access.
- **Starter Plugins:** Tasks, Kanban, Note Chat.
- **Safe Mode:** Auto-disable crashes.

---

## 6. Theming, Accessibility, and Performance
- **Themes:** Tailwind tokens; light/dark/high-contrast.
- **Accessibility:** Keyboard nav, ARIA, WCAG.
- **Performance:**
  - Workers for parse/index/search.
  - Virtualized lists; p95 search <75ms.

---

## 7. Analytics / Telemetry
- **Opt-in:** PostHog or Segment.
- **Events:** Vault open, edit, search, plugin actions, export, crash.
- **No content captured.**

---

## 8. Security and Privacy
- No server dependency for core.
- Plugin permission prompts.
- Optional E2E encrypted sync.

---

## 9. Release Plan
| Phase | Duration | Goals |
|-------|-----------|-------|
| 0 | 1 week | Spike (editor, vault I/O, index, tokens). |
| 1 | 2 weeks | Web MVP (PWA, P0 features). |
| 2 | 1 week | macOS wrapper + plugin loader. |
| 3 | 2 weeks | Sync polish, export improvements. |
| 4 | 2-3 weeks | iOS beta (Capacitor + Swift bridge). |

---

## 10. Risks and Mitigations
- **Web search scale:** Partitioned indexes, WASM later.
- **File watching:** Focus refresh + manual command.
- **Plugin safety:** Strict perms + Safe Mode.
- **Mobile performance:** Lazy parsing, pause background indexing.

---

## 11. Technology Checklist
- **Core:** Rust, comrak, serde, notify, tantivy, wasm-bindgen.
- **Web UI:** SvelteKit, CodeMirror 6, Tailwind, unified, FlexSearch.
- **Desktop:** Tauri 2.x.
- **iOS:** Capacitor + Swift bridge.
- **Plugins:** Worker runtime, manifest + permissions.
- **Build:** pnpm + Turborepo, Vite, GitHub Actions.
- **Docs:** Astro/Starlight.

---

## 12. Optional Starter Monorepo
- **Packages:**
  - `core/` – Rust engine (WASM + native).
  - `web/` – SvelteKit app.
  - `desktop/` – Tauri wrapper.
  - `mobile/` – Capacitor shell.
  - `plugins/` – sample extensions.
