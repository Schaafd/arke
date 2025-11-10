# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Arke is a cross-platform, markdown-first personal knowledge management (PKM) system with a local-first architecture. The project is currently in the planning phase with no implementation yet.

**Core Concept:** Fast, extensible note-taking with wikilinks, backlinks, and a powerful plugin system across Web, macOS, and iOS.

## Architecture

### Planned Monorepo Structure

```
arke/
├── packages/
│   ├── core/          # Rust engine (WASM for web, native for desktop/mobile)
│   ├── web/           # SvelteKit PWA
│   ├── desktop/       # Tauri 2.x macOS wrapper
│   ├── mobile/        # Capacitor iOS app
│   └── plugins/       # Sample plugin implementations
└── plans/             # All project documentation
```

### Technology Stack

- **Core Engine:** Rust (compiled to WASM for web, native for desktop/mobile)
  - Markdown parsing: `comrak`
  - Search/indexing: `tantivy` (native), FlexSearch (web)
  - File watching: `notify`

- **Web UI:** SvelteKit + TypeScript
  - Editor: CodeMirror 6
  - Styling: Tailwind CSS
  - Markdown processing: unified, remark, rehype
  - Math: KaTeX

- **Desktop:** Tauri 2.x (Rust-based wrapper)
- **Mobile:** Capacitor + Swift bridge for iOS
- **Build Tools:** pnpm, Turborepo, Vite

## Key Design Principles

### Local-First Architecture

- All notes stored as plain Markdown files in local vaults
- No server dependency for core features
- Full offline functionality
- Optional sync via OS-level file providers (iCloud, Dropbox)

### Performance Targets

- Cold start: <1s
- Keystroke latency: <10ms
- File switching: <50ms
- Search (p95): <75ms
- Crash-free sessions: ≥99.9%

### Core Features (P0)

- **Editor:** Low-latency Markdown editing with live preview
- **Linking:** `[[wikilinks]]` with automatic backlinks
- **Organization:** Tags, daily notes, outline navigation
- **Search:** Instant, typo-tolerant full-text search
- **Command Palette:** Keyboard-first navigation

### Plugin System

- Permissioned runtime (Web Workers / Tauri sidecar)
- Manifest-based (`manifest.json` with capabilities)
- Safe mode for crash recovery
- Starter plugins: Task Dashboard, Kanban, Note Chat

## Documentation

All project documentation lives in `/plans/`:

- **`Arke_PRD.md`** - Product Requirements Document with business goals, user stories, functional requirements, UX flows, and success metrics
- **`Arke_Tech_Stack_and_Plan.md`** - Technology stack decisions, architecture details, and release phases
- **`Initial_Development_Plan.md`** - Detailed 8-9 week development roadmap with phase-by-phase tasks

## Development Phases

The project follows a 5-phase plan:

1. **Phase 0 (Week 1):** Spike - Core technology validation
2. **Phase 1 (Weeks 2-3):** Web MVP with all P0 features
3. **Phase 2 (Week 4):** macOS wrapper + plugin system
4. **Phase 3 (Weeks 5-6):** File-based sync + polish
5. **Phase 4 (Weeks 7-9):** iOS beta

## Implementation Notes

### When Starting Implementation

1. Initialize monorepo with pnpm workspace + Turborepo
2. Set up Rust workspace in `packages/core/`
3. Start with vault I/O abstraction and Markdown parsing
4. Build editor prototype with CodeMirror 6 in `packages/web/`
5. Implement wikilink parsing and backlinks map

### Critical Technical Decisions

- **File System Access:** Web uses File System Access API (or OPFS fallback)
- **WASM Compilation:** Core Rust library compiles to WASM for web using `wasm-bindgen`
- **Markdown Spec:** CommonMark + extended syntax (tables, footnotes, task checkboxes)
- **Callouts:** Implemented via `remark-directive`
- **Sync Strategy:** OS-level file providers first; optional Arke Sync service post-v1

### Plugin Architecture

Plugins are isolated in Workers/sidecars with explicit permissions:

- File access (scoped to vault)
- Network access
- UI panel creation
- Command registration

Each plugin has a `manifest.json` declaring capabilities. Permission prompts shown on install.

## Important Constraints

### Security

- Plugin permission model must be strict
- No arbitrary code execution without user consent
- E2E encryption for optional sync service

### Privacy

- Opt-in telemetry only (anonymous, aggregated)
- No note content captured in analytics
- Transparent data collection policies

### Accessibility

- WCAG AA compliance minimum
- Full keyboard navigation
- Screen reader support
- High-contrast theme

## Success Metrics

Track these from day 1:

- Activation rate: ≥70% create note in first 10 minutes
- Day-30 retention: ≥35%
- MAU target: 10,000 within 6 months
- Pro conversion: 5-7%

## When Working on This Codebase

### Before Implementation Exists

- Reference `/plans/Initial_Development_Plan.md` for task breakdowns
- Follow the phase sequence (don't skip Phase 0 validation)
- Set up performance monitoring from the start

### During Implementation

- Maintain keystroke latency <10ms (critical for UX)
- Use Web Workers for parse/index/search operations
- Test with large vaults (10k+ notes) regularly
- Implement crash recovery and autosave early

### For New Features

- Check if it's P0, P1, or post-v1 in the PRD
- Consider plugin system extension vs. core feature
- Validate against performance targets
- Ensure accessibility compliance

## Project Status

**Current Phase:** Planning complete, awaiting implementation start

**Next Steps:**

1. Initialize repository structure with pnpm workspace
2. Set up Rust core library with WASM compilation
3. Create SvelteKit web app shell
4. Build editor prototype with file I/O
