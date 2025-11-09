# Arke Initial Development Plan

## Overview

Arke is a cross-platform, markdown-first personal knowledge management (PKM) system built with a local-first architecture. This plan outlines the development path from initial setup through iOS beta launch.

**Timeline:** 8-9 weeks
**Team:** 1-2 developers (full-stack + designer)
**Target:** Web PWA → macOS Desktop → iOS App

---

## Project Setup

### Repository Structure (Monorepo)

```
arke/
├── packages/
│   ├── core/              # Rust engine (WASM + native)
│   ├── web/               # SvelteKit PWA
│   ├── desktop/           # Tauri 2.x wrapper
│   ├── mobile/            # Capacitor + iOS bridge
│   └── plugins/           # Sample plugin implementations
├── plans/                 # Documentation and planning
├── .github/               # CI/CD workflows
├── pnpm-workspace.yaml
└── turbo.json
```

### Initial Setup Tasks

- [ ] Initialize pnpm workspace with Turborepo
- [ ] Set up Rust workspace for core package
- [ ] Configure build tools (Vite, wasm-pack, cargo)
- [ ] Set up GitHub Actions for CI/CD
- [ ] Configure ESLint, Prettier, rustfmt
- [ ] Set up testing infrastructure (Vitest, cargo test)

### Technology Stack Summary

| Component | Technology |
|-----------|------------|
| Core Engine | Rust (comrak, tantivy, serde, notify) |
| Web Compilation | wasm-bindgen, wasm-pack |
| UI Framework | SvelteKit + TypeScript |
| Editor | CodeMirror 6 |
| Styling | Tailwind CSS |
| Markdown Processing | unified, remark, rehype, remark-directive |
| Search (Web) | FlexSearch (Worker-based) |
| Search (Native) | tantivy |
| Desktop Wrapper | Tauri 2.x |
| Mobile Wrapper | Capacitor + Swift |
| Package Manager | pnpm |
| Monorepo Tool | Turborepo |
| Math Rendering | KaTeX |
| Graph Visualization | Cytoscape.js (post-v1) |

---

## Phase 0: Spike & Foundation (Week 1)

**Goal:** Validate core technology choices and establish architecture foundation.

### Core Engine Tasks

- [ ] Create Rust library structure in `packages/core/`
- [ ] Implement vault abstraction (open folder, list files, read/write)
- [ ] Integrate comrak for Markdown parsing
- [ ] Build wikilink extractor (`[[link]]` pattern detection)
- [ ] Implement basic backlinks map (file → referenced files)
- [ ] Set up WASM compilation pipeline
- [ ] Write unit tests for vault operations and parsing

### Editor Prototype

- [ ] Initialize SvelteKit app in `packages/web/`
- [ ] Integrate CodeMirror 6 with basic Markdown syntax highlighting
- [ ] Create simple file tree component
- [ ] Implement basic file open/save via File System Access API
- [ ] Test keystroke latency (target: <10ms)
- [ ] Build split view (editor + preview pane)

### Indexing & Search Proof-of-Concept

- [ ] Implement incremental indexing in Rust core
- [ ] Integrate FlexSearch for web search
- [ ] Test search performance with 1k+ sample notes
- [ ] Verify p95 latency <75ms target

### Design System

- [ ] Define Tailwind theme tokens (colors, typography, spacing)
- [ ] Create light/dark mode presets
- [ ] Build basic component library (buttons, inputs, panels)
- [ ] Establish accessibility baseline (WCAG AA contrast)

### Performance Baselines

- [ ] Measure cold start time (target: <1s)
- [ ] Profile editor typing performance
- [ ] Test file switching latency (target: <50ms)

**Deliverables:**
- Working editor with file I/O
- Functional wikilink parsing and backlinks
- Search prototype with acceptable performance
- Design system foundation

---

## Phase 1: Web MVP (Weeks 2-3)

**Goal:** Build a fully functional Web PWA with core P0 features.

### Core Editing Features (P0)

- [ ] Complete CodeMirror 6 extensions:
  - [ ] Wikilink syntax highlighting
  - [ ] Auto-complete for existing file names
  - [ ] Markdown toolbar (bold, italic, headings, lists)
- [ ] Implement live preview toggle
- [ ] Add split view with synchronized scrolling
- [ ] Support extended Markdown syntax:
  - [ ] Tables
  - [ ] Footnotes
  - [ ] Task checkboxes
  - [ ] Fenced code blocks with syntax highlighting
- [ ] Callouts via remark-directive
- [ ] Math inline rendering with KaTeX (fenced notation)

### File Management (P0)

- [ ] Vault creation and opening flow
- [ ] File tree with folder hierarchy
- [ ] New file, rename, move, duplicate operations
- [ ] Safe delete with undo confirmation
- [ ] Attachment support (images, PDFs in `/assets/`)
- [ ] Autosave with status indicator
- [ ] Recent files list

### Linking & Navigation (P0)

- [ ] Wikilink auto-completion while typing
- [ ] Click to follow links (Cmd/Ctrl+Click)
- [ ] Backlinks panel showing inbound links
- [ ] Hover preview for linked notes
- [ ] Safe rename: update all backlinks automatically
- [ ] Show diff preview before rename
- [ ] Broken link detection and warnings

### Organization (P0)

- [ ] Inline tag support (`#tag`)
- [ ] Frontmatter parsing (tags, aliases, status)
- [ ] Tag browser/filter view
- [ ] Favorites/pinned notes
- [ ] Outline panel (heading navigation)
- [ ] Breadcrumb navigation

### Search (P0)

- [ ] Global search via Cmd/Ctrl+K
- [ ] Instant, typo-tolerant search
- [ ] Filter by tags, folder, date
- [ ] Search result previews with highlighting
- [ ] Keyboard navigation through results
- [ ] Recent searches

### Command Palette (P0)

- [ ] Global command search (Cmd/Ctrl+Shift+P)
- [ ] Quick actions (new note, search, settings)
- [ ] Command history
- [ ] Keyboard-first navigation

### Theming (P1-lite)

- [ ] Light/dark mode toggle
- [ ] High-contrast theme
- [ ] Font size adjustment
- [ ] Line length control
- [ ] Distraction-free mode (hide sidebars)

### Onboarding

- [ ] Welcome screen with "Create Vault" / "Open Existing"
- [ ] Sample notes (Welcome, Shortcuts, Linking Guide)
- [ ] 60-second guided tour
- [ ] Keyboard cheat sheet overlay
- [ ] Privacy preferences opt-in

### PWA Configuration

- [ ] Service worker for offline support
- [ ] App manifest (icons, theme colors)
- [ ] Install prompt
- [ ] Offline fallback page

**Deliverables:**
- Functional Web PWA with all P0 features
- Onboarding flow for new users
- Basic theming support
- PWA installable on desktop and mobile browsers

---

## Phase 2: macOS Wrapper + Plugin Foundation (Week 4)

**Goal:** Package web app as native macOS application and establish plugin system.

### Desktop Packaging (Tauri)

- [ ] Initialize Tauri 2.x in `packages/desktop/`
- [ ] Configure app icon, metadata, and signing
- [ ] Implement native file system access
- [ ] Native file picker dialogs
- [ ] System menu integration
- [ ] Native window controls
- [ ] App tray/dock integration
- [ ] Auto-update mechanism

### Performance & Stability

- [ ] Crash recovery on unexpected quit
- [ ] Autosave before app close
- [ ] Memory profiling and optimization
- [ ] File watcher integration (notify crate)
- [ ] Conflict detection for external changes
- [ ] Performance tuning for large vaults (10k+ notes)

### Plugin System Foundation (P1)

- [ ] Define plugin manifest schema (`manifest.json`)
  - name, version, description, author
  - capabilities (file access, network, UI panels)
  - permissions model
- [ ] Build plugin loader (Web Worker runtime)
- [ ] Implement permission prompts
- [ ] Plugin manager UI (install, enable/disable, update)
- [ ] Safe mode (auto-disable crashing plugins)
- [ ] Plugin API surface:
  - Command registration
  - Panel creation
  - File access (scoped)
  - Event hooks

### Starter Plugins

- [ ] **Task Dashboard Plugin**
  - Aggregate all checkboxes across vault
  - Filter by status, file, date
  - Mark complete from dashboard
- [ ] **Kanban Plugin**
  - Board view with columns
  - Drag-and-drop cards
  - Link cards to notes
- [ ] **Note Chat Plugin** (opt-in, privacy-first)
  - Simple local LLM integration or API-based
  - Context-aware note queries
  - Clearly labeled as experimental

### Export Improvements

- [ ] Export to HTML (clean, styled)
- [ ] Export to PDF via Tauri sidecar
- [ ] Export to Markdown bundle with assets
- [ ] Preserve internal links in exports
- [ ] Batch export (folder → ZIP)

**Deliverables:**
- macOS .app bundle
- Working plugin system with 3 starter plugins
- Enhanced export functionality
- Stable, crash-resistant application

---

## Phase 3: Sync & Polish (Weeks 5-6)

**Goal:** Enable cross-platform sync via OS file providers and polish UX.

### File-Based Sync

- [ ] Document iCloud Drive setup for users
- [ ] Document Dropbox/Google Drive integration
- [ ] Implement file watcher for external changes
- [ ] Conflict detection (local vs. remote edits)
- [ ] Conflict resolution UI:
  - Show diff view
  - "Keep Local" / "Keep Remote" / "Merge" options
  - Manual merge guidance
- [ ] Sync status indicators
- [ ] Handle network failures gracefully

### Templates & Daily Notes

- [ ] Template management (create, edit, delete)
- [ ] Per-vault template folders
- [ ] Daily note auto-creation
- [ ] Weekly/monthly note generation
- [ ] Date-based note linking
- [ ] Template variables (date, time, vault name)

### Advanced Organization

- [ ] Alias support for notes (alternative titles)
- [ ] Tag hierarchy (nested tags)
- [ ] Graph view (lightweight, neighborhood only)
  - Prepare for full Cytoscape.js integration post-v1
- [ ] Related notes suggestions

### Accessibility Enhancements

- [ ] Full keyboard navigation audit
- [ ] Screen reader testing and fixes
- [ ] ARIA labels for all interactive elements
- [ ] Focus management improvements
- [ ] Keyboard shortcuts documentation

### Performance Optimization

- [ ] Lazy loading for large files
- [ ] Virtual scrolling for long lists (@tanstack/virtual)
- [ ] Background indexing with progress indicator
- [ ] Caching for previews and search results
- [ ] Optimize WASM bundle size

### Documentation

- [ ] User guide (website or in-app)
- [ ] Keyboard shortcuts reference
- [ ] Plugin development guide
- [ ] API documentation
- [ ] FAQ and troubleshooting

**Deliverables:**
- Reliable file-based sync with conflict resolution
- Daily notes and templates
- Polished, accessible UX
- Comprehensive user documentation

---

## Phase 4: iOS Beta (Weeks 7-9)

**Goal:** Launch iOS app with Files integration for on-the-go editing.

### iOS App Setup

- [ ] Initialize Capacitor in `packages/mobile/`
- [ ] Configure iOS project with Xcode
- [ ] Set up Swift bridge for native features
- [ ] Integrate with Files app provider
- [ ] Request file system permissions

### Mobile-Optimized UI

- [ ] Touch-friendly controls (larger tap targets)
- [ ] Mobile navigation patterns (bottom tabs, swipe gestures)
- [ ] Responsive layouts for iPhone and iPad
- [ ] Optimized keyboard toolbar
- [ ] Context menus for long-press
- [ ] Pull-to-refresh

### Offline Mode

- [ ] Full offline editing capability
- [ ] Queue sync operations when offline
- [ ] Offline indicator in UI
- [ ] Automatic sync when online
- [ ] Handle app backgrounding gracefully

### Performance for Mobile

- [ ] Reduce bundle size (code splitting)
- [ ] Lazy parsing for large notes
- [ ] Pause background indexing on low battery
- [ ] Optimize memory usage
- [ ] Smooth scrolling and animations

### Mobile-Specific Features

- [ ] Share extension (capture to Arke)
- [ ] iOS Shortcuts integration
- [ ] Quick note widget
- [ ] Biometric lock (optional)
- [ ] Dark mode respects system setting

### TestFlight Launch

- [ ] Prepare App Store metadata
- [ ] Create screenshots and preview video
- [ ] Set up TestFlight beta group
- [ ] Internal testing round
- [ ] External beta launch
- [ ] Gather feedback and iterate

**Deliverables:**
- Functional iOS app on TestFlight
- Full offline support
- Mobile-optimized UX
- Integration with iOS ecosystem

---

## Technical Decisions Log

Document key architectural decisions as they're made:

### Decided

1. **Rust Core + WASM:** Enables code reuse across platforms; acceptable performance.
2. **SvelteKit for UI:** Fast, minimal overhead, good DX.
3. **CodeMirror 6:** Best-in-class editor performance and extensibility.
4. **File System Access API:** Enables true local-first on web without server.
5. **Tauri 2.x:** Lightweight, Rust-based, good security model.
6. **FlexSearch (Web) + tantivy (Native):** Balance web compatibility with native performance.

### To Decide

- [ ] **Plugin sandboxing model:** Web Workers vs. iframes vs. Tauri sidecar?
- [ ] **Optional sync service architecture:** Server stack for Arke Sync (post-v1)?
- [ ] **Telemetry provider:** PostHog vs. Segment vs. custom?
- [ ] **Graph visualization library:** Cytoscape.js vs. D3.js vs. vis.js?
- [ ] **Mobile offline storage:** OPFS vs. Capacitor Filesystem?
- [ ] **Monetization gateway:** Stripe vs. Paddle for Pro subscriptions?

---

## Development Workflow

### Branch Strategy

- `main`: Production-ready code
- `develop`: Integration branch for features
- `feature/*`: Individual feature branches
- `release/*`: Release preparation branches

### Testing Strategy

- **Unit Tests:** Rust core (cargo test), Web utilities (Vitest)
- **Integration Tests:** API contracts between core and UI
- **E2E Tests:** Critical user flows (Playwright)
- **Manual Testing:** Accessibility, performance, cross-platform

### Code Review Process

- All changes via pull requests
- At least one approval required
- Automated checks: lint, format, test, build
- Performance regression tests for core paths

### Release Process

1. Create release branch
2. Update version numbers and changelog
3. Full QA pass
4. Build release artifacts
5. Tag release in git
6. Deploy web PWA
7. Publish desktop apps (macOS App Store / direct download)
8. Submit iOS TestFlight build
9. Announce release with changelog

---

## Milestone Checklist

### Phase 0 Complete ✓

- [ ] Editor prototype with file I/O works
- [ ] Wikilink parsing and backlinks functional
- [ ] Search meets performance targets (<75ms p95)
- [ ] Design system established

### Phase 1 Complete ✓

- [ ] Web PWA with all P0 features
- [ ] Onboarding flow tested with users
- [ ] PWA installable and works offline
- [ ] Performance targets met (cold start <1s, typing <10ms)

### Phase 2 Complete ✓

- [ ] macOS app signed and distributable
- [ ] Plugin system with 3 working plugins
- [ ] Crash recovery and stability tested
- [ ] Export to HTML/PDF functional

### Phase 3 Complete ✓

- [ ] Sync via iCloud Drive/Dropbox documented and tested
- [ ] Conflict resolution UI works reliably
- [ ] Daily notes and templates implemented
- [ ] User documentation published

### Phase 4 Complete ✓

- [ ] iOS app on TestFlight
- [ ] Offline editing works flawlessly
- [ ] Mobile UX meets accessibility standards
- [ ] Beta feedback collected and prioritized

---

## Success Metrics (from PRD)

### Activation

- **Target:** ≥70% of new users create a note within first 10 minutes
- **Measure:** Track `note_created` event in first session

### Retention

- **Target:** ≥35% Day-30 retention
- **Measure:** Users active on Day 30 / Users who signed up on Day 0

### Performance

- **Cold start:** ≤1s median
- **Keystroke latency:** ≤10ms
- **Crash-free sessions:** ≥99.9%

### Adoption

- **MAU Target:** 10,000 within 6 months of launch
- **Plugin installs:** 10,000 across 50+ plugins by month 6
- **Pro conversion:** 5-7% of active users

### Satisfaction

- **CSAT:** ≥4.5/5
- **NPS:** ≥45

---

## Post-v1 Roadmap (Phase 5+)

Potential future enhancements:

- **Encrypted Arke Sync:** Optional E2E encrypted sync service
- **Full Graph Visualization:** Interactive knowledge graph with Cytoscape.js
- **Advanced Plugin APIs:** More extensibility hooks
- **Preferences Sync:** Sync settings across devices
- **Git Integration:** Version control for notes
- **Collaboration:** Shared vaults (real-time or async)
- **Mobile: Android:** Expand to Android platform
- **API & CLI:** Programmatic access to vaults
- **Advanced Export:** LaTeX, DOCX, EPUB formats
- **Spaced Repetition:** Built-in or plugin for learning

---

## Risk Mitigation

### Risk: Web search performance degrades at scale

- **Mitigation:** Partitioned indexes, lazy loading, consider WASM for search
- **Monitoring:** Track p95 search latency in telemetry

### Risk: File watching unreliable across platforms

- **Mitigation:** Manual refresh command, focus-based refresh, polling fallback
- **Testing:** Test with iCloud, Dropbox, Google Drive on all platforms

### Risk: Plugin security vulnerabilities

- **Mitigation:** Strict permission model, code review for marketplace plugins, safe mode
- **Response:** Quick disable mechanism for problematic plugins

### Risk: Mobile performance issues

- **Mitigation:** Lazy parsing, pause indexing on battery/background, aggressive optimization
- **Testing:** Test on older iOS devices (iPhone X, iPad 6th gen)

### Risk: User data loss

- **Mitigation:** Autosave, conflict detection, backup prompts, export tools
- **Recovery:** Clear error messages, support documentation

---

## Next Steps

1. **Set up repository structure** (see Project Setup section)
2. **Begin Phase 0 spike** with focus on editor and vault I/O
3. **Establish weekly sync** to review progress and adjust plan
4. **Set up telemetry** to measure performance and usage from day 1
5. **Create design mockups** for core screens and flows

---

**Document Version:** 1.0
**Last Updated:** 2025-11-09
**Next Review:** End of Phase 0
