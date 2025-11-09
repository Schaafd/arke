# Arke: Cross-Platform Markdown PKM Application

### TL;DR

Arke is a modern, fast, markdown-first personal knowledge management (PKM) system inspired by Thymer’s focus-driven simplicity and Kortex’s extensible, link-rich workflows. It delivers a local-first, cross-platform experience with flexible organization, powerful linking, theming, and a lightweight plugin model. Designed for developers, writers, students, and power users who want speed, control, and consistency across web, desktop, and mobile.

---

## Goals

### Business Goals

* Achieve 10,000 Monthly Active Users within 6 months of launch.

* Sustain 35% Day-30 retention and 55% Week-4 activation-to-retention rate.

* Convert 5–7% of active users to Pro (themes, advanced plugins, optional sync) within 9 months.

* Launch a plugin directory with 50+ vetted plugins and reach 10,000 plugin installs by month 6.

* Maintain CSAT ≥ 4.5/5 and NPS ≥ 45.

### User Goals

* Write and organize ideas with near-zero friction using fast, reliable Markdown.

* Link knowledge effortlessly (backlinks, tags, and daily notes) to form a coherent knowledge graph.

* Access and edit notes across web, macOS, and iOS with consistent behavior and shortcuts.

* Customize the workspace (themes, layout, plugins) to fit diverse workflows.

* Retain full ownership of data (plain-text Markdown, transparent sync, easy import/export).

### Non-Goals

* Real-time multi-user collaboration in v1 (no shared cursors or complex permissions).

* A heavy WYSIWYG editor (Arke remains Markdown-first with tasteful previews).

* Proprietary file formats or data lock-in (Arke avoids closed formats).

---

## User Stories

* Developer

  * As a Developer, I want to create linked technical notes using \[\[wikilinks\]\], so that related concepts are discoverable via backlinks.

  * As a Developer, I want checklists and code-friendly formatting, so that I can plan and track tasks without leaving my notes.

  * As a Developer, I want a command palette and keyboard-first navigation, so that I can stay in flow.

* Writer

  * As a Writer, I want to draft and restructure long-form content, so that I can move from outline to publication smoothly.

  * As a Writer, I want theming and distraction-free mode, so that I can focus for extended periods.

  * As a Writer, I want to export to clean HTML/PDF, so that I can share finished work easily.

* Student

  * As a Student, I want to capture lecture notes with tags and links, so that I can review by topic later.

  * As a Student, I want daily notes and spaced-repetition-friendly callouts, so that I can retain key insights.

  * As a Student, I want mobile access, so that I can study on the go.

* Power User

  * As a Power User, I want plugin extensibility and custom commands, so that I can tailor the app to my workflow.

  * As a Power User, I want fast, global search across thousands of notes, so that I can retrieve information instantly.

  * As a Power User, I want granular sync control and local-first operation, so that I decide how and where my data lives.

---

## Functional Requirements

* Core Markdown Editing (Priority: P0) -- Editor: Low-latency typing, CommonMark + extended syntax (tables, footnotes, math inline via fenced notation), live preview toggle, split view. -- Linking: \[\[wikilinks\]\], automatic backlinks pane, link auto-complete, renaming updates links. -- Structure: Headings outline panel, collapsible sections, callouts/admonitions, task checkboxes. -- Command Palette: Global command search, keyboard-first actions.

* File Management (Priority: P0) -- Vaults: Create/open local folders as vaults; import existing Markdown directories. -- Files: New, rename, move, duplicate; safe delete with undo; attachments (images, PDFs). -- Search: Instant, typo-tolerant search with filters (tags, path, date); recent files. -- Templates: Per-vault templates; frontmatter support (tags, aliases, status).

* Organization & Navigation (Priority: P0) -- Tags and Aliases: Inline tags, file-level tags; aliases for titles. -- Daily/Periodic Notes: Auto-create daily, weekly, monthly pages with links. -- Graph: Lightweight backlinks and neighborhood view in v1; full graph visualization post-v1 (P2). -- Favorites & Pins: Quick access to key notes; recent edits history.

* Plugins & Extensibility (Priority: P1 for v1 subset) -- Plugin Loader: Safe, permissioned plugin runtime with clear capabilities. -- Plugin Manager: Install, enable/disable, update; per-vault configuration; minimal curation. -- Starter Plugins: Task dashboard, Kanban, and simple “Note Chat” plugin (opt-in, respects privacy). -- Scripting Hooks: Commands, panels, and menu augmentation (limited in v1).

* Theming & Customization (Priority: P1) -- Themes: Light/Dark/High-Contrast presets; custom accent color and typography scales. -- Layout: Toggle sidebars, status bar; distraction-free mode. -- Preferences Sync: Optional settings sync (post-v1 P2).

* Sync & Cross-Platform (Priority: P1) -- Local-First: Files stored plainly in local vaults; fully functional offline. -- Cloud Connectors: Integrate with OS-level file providers (e.g., iCloud Drive, Dropbox, Google Drive) via the file system. -- Optional Arke Sync: Opt-in end-to-end encrypted sync (post-v1 P2). -- Platforms: Web PWA (v1), macOS desktop wrapper (v1.1), iOS app (beta post-v1).

* Accessibility & Reliability (Priority: P0) -- Accessibility: Full keyboard navigation, screen reader labels, scalable fonts, high-contrast theme. -- Performance: Keystroke latency under 10ms, instant file switching, non-blocking indexing. -- Stability: Crash recovery, autosave, conflict alerts and resolutions.

V1 focus: Core Markdown Editing, File Management, Organization & Navigation, Accessibility & Reliability; basic Plugins & Theming; Web PWA + macOS wrapper.

---

## User Experience

* Entry Point & First-Time User Experience

  * Discovery: Users land on the web app or download from the website/App Store. Clear message: “Your notes, your way—Markdown, fast, extensible.”

  * Accountless Start: Create/open a local vault without sign-up. Optional sign-in only for sync and Pro features.

  * Onboarding: Choose “Create Vault” or “Open Existing.” Provide sample notes (Welcome, Shortcuts, Linking) and a 60-second guided tour.

  * Keyboard Cheat Sheet: Quick overlay shows fundamental commands (new note, search, backlink toggle, command palette).

  * Privacy Preferences: Prompt to opt-in to anonymous usage analytics; explain data collection transparently.

* Core Experience

  * Step 1: Create or open a vault

    * Minimal steps: Select a folder or create one; confirm permissions.

    * Validation: Check read/write access; detect existing Markdown structure.

    * Success: Vault opens; sidebar shows folders and tags; center shows Welcome note; next steps surfaced (Create Note, Import).

  * Step 2: Create a note

    * Press Cmd/Ctrl+N or click “New Note.”

    * Title field auto-focused; first line becomes H1; autosave starts immediately.

    * Confirm saved status in status bar; undo/redo available.

  * Step 3: Write and format

    * Markdown toolbar for common elements; live preview toggle.

    * Error handling: Show unobtrusive hints for malformed tables or links.

    * Suggest related notes as you type links; provide quick create for new \[\[pages\]\].

  * Step 4: Organize

    * Drag notes into folders; add tags inline or in frontmatter.

    * Daily note template available from command palette; pin important notes.

    * Rename notes safely: update backlinks automatically; show diff preview of changes.

  * Step 5: Link and navigate

    * Backlinks panel shows inbound links and context snippets.

    * Hover previews of linked notes; open in split panes for side-by-side editing.

    * Outline panel to jump headings; breadcrumbs highlight path.

  * Step 6: Search and retrieve

    * Global search via Cmd/Ctrl+K: notes, commands, and tags unified.

    * Filters for type, folder, date; keyboard navigation through results.

    * Search shows instant previews with highlighted matches.

  * Step 7: Customize and theme

    * Switch Light/Dark/High-Contrast; adjust font size and line length.

    * Distraction-free mode: hides sidebars and toolbars; focus on content.

  * Step 8: Plugins (optional in v1)

    * Open Plugin Manager; browse starter plugins.

    * Permissions dialog explains access (read notes, write notes, UI panels).

    * Enable Task Dashboard to surface checkboxes across vault; configure scopes.

  * Step 9: Sync and cross-platform

    * Default: Use OS file provider (e.g., iCloud Drive folder) to sync across devices.

    * Optional: Enable Arke Sync when available; verify encryption key; choose sync scope.

    * iOS: Open same vault via Files app; edit offline; changes merge when online.

  * Step 10: Export and share

    * Export note or folder to PDF/HTML/Markdown bundle.

    * Preserve internal links where possible; include media assets.

    * Show completion message with open-in-folder button.

* Advanced Features & Edge Cases

  * Large vault performance: Lazy-load index; background indexing; progress indicator in status bar.

  * Conflicts: Detect divergent edits; show diff with “Keep Local,” “Keep Remote,” or “Merge” guidance.

  * Broken links: Backlink panel flags missing targets; one-click create missing note or fix link.

  * Plugin safe mode: If a plugin crashes, restart without third-party plugins; show diagnostics.

  * Import/Export: Bulk import from other Markdown apps; map frontmatter fields to Arke tags/aliases.

  * Non-Latin scripts and RTL: Ensure correct cursor movement, line wrapping, and search normalization.

  * Mobile constraints: Auto-save on backgrounding; reduced animations; battery-friendly indexing.

* UI/UX Highlights

  * Performance-first: Sub-100ms app boot for cached sessions; sub-10ms keystroke latency.

  * Clarity: Minimal chrome, consistent iconography, and plain-language tooltips.

  * Accessibility: WCAG-compliant color contrast; full keyboard access; ARIA labeling; screen reader-friendly headings and landmarks.

  * Consistency: Same shortcuts across platforms where possible; platform-appropriate fallbacks.

  * Discoverability: Command palette as the universal entry point; contextual tips that can be muted.

  * Resilience: Autosave indicator; undoable destructive actions; robust offline behavior.

---

## Narrative

Maya, a full-stack developer who writes technical essays and mentors students, often juggles specs, code snippets, and reading notes across her laptop and phone. She’s tried heavyweight note apps that feel slow and rigid, and barebones editors that fall apart at scale. What she wants is speed, structure when she needs it, and the comfort of plain text that she controls.

With Arke, Maya creates a new vault on her Mac and starts a “Graph Database Primer” note. She drafts in Markdown at full speed, adding code fences and callouts for pitfalls. When she brackets \[\[Indexing Strategies\]\], Arke suggests an existing note and shows backlinks to related topics. She pins the overview, creates a daily note for open questions, and drops PDFs into an Attachments folder—links update automatically.

During a commute, Maya opens the same vault via iCloud Drive on her iPhone. The mobile editor is just as responsive; she checks tasks off and adds a paragraph offline. Back at her desk, everything is already synced. From the command palette, she enables the Task Dashboard plugin to surface checkboxes across all notes and switches to a high-contrast theme for a long evening session.

By the end of the week, Maya exports a clean HTML version for her blog and keeps her original Markdown safe in her vault. She has the speed of a focused editor, the power of linked knowledge, and the freedom of portable files. Arke helps her ship work faster while keeping her data—and her flow—under her control.

---

## Success Metrics

* Activation rate: ≥ 70% of new users create a note within first 10 minutes.

* Retention: ≥ 35% Day-30 retention; ≥ 20% monthly returning users with 5+ sessions.

* Performance: Median cold start ≤ 1s; keystroke latency ≤ 10ms; crash-free sessions ≥ 99.9%.

* Revenue: 5–7% conversion to paid; target $25k MRR by Month 12.

* Plugin ecosystem: 50 published plugins; 10,000 installs; ≥ 75% plugin health (no crash reports per 1k sessions).

### User-Centric Metrics

* Weekly Active Editors (WAEs): Users who edit ≥ 3 notes/week.

* Average time-to-first-link: Measure onboarding success into linking workflows.

* Task completion rate: Percentage of users who create and complete tasks/checklists weekly.

* CSAT/NPS: In-product surveys after week 2 and week 6.

### Business Metrics

* Free-to-Pro conversion rate; monthly churn; ARPU and MRR growth.

* Marketplace GMV for plugins/themes (if monetized); revenue per active vault.

* Support volume per 1,000 users; median response/resolution times.

### Technical Metrics

* App cold/warm start times; memory footprint under target per platform.

* Indexing throughput (notes/sec) and search latency (p95 < 75ms).

* Sync reliability: conflict rate < 0.5% of sync events; recovery success > 98%.

* Crash-free sessions ≥ 99.9%; plugin sandbox error containment ≥ 99%.

### Tracking Plan

* app_opened, onboarding_started/completed

* vault_created/opened/imported

* note_created/edited/renamed/moved/deleted

* link_created/backlink_viewed/broken_link_fixed

* search_opened/query_submitted/result_opened

* template_used, daily_note_created

* theme_changed, distraction_free_toggled

* plugin_installed/enabled/disabled/updated

* sync_enabled/provider_selected/conflict_resolved

* export_started/completed

* settings_viewed/privacy_opt_in/out

* crash_reported/plugin_crashed/safe_mode_entered

(Analytics are opt-in, anonymized, and minimal by default.)

---

## Technical Considerations

### Technical Needs

* Editor Engine: Markdown parser and renderer with split-view; low-latency text model; callouts, tables, and footnotes support.

* File Layer: Vault abstraction over local filesystem; attachment handling; safe rename and link updates; file watchers.

* Index & Search: Incremental indexing for content, tags, links; typo tolerance; fast query engine.

* Linking & Graph: Backlinks index; neighborhood view; deferred full-graph visualization.

* Plugin Runtime: Permissioned sandbox; API for commands, panels, file access (scoped); crash isolation and safe mode.

* Theming: Theme tokens, presets, and user overrides; high-contrast accessibility baseline.

* Cross-Platform Shell: Web PWA first; lightweight desktop wrapper for macOS; native iOS with Files provider integration.

* Sync Layer: OS file providers by default; optional encrypted sync as a service (post-v1); conflict detection and guided resolution.

### Integration Points

* OS-level file providers: iCloud Drive, Dropbox, Google Drive via filesystem integration.

* Share sheets and Shortcuts (iOS/macOS) for quick capture and automation.

* Import from other PKM systems: Markdown directories, zipped exports; field mapping for tags/aliases/frontmatter.

* Export to HTML/PDF/Markdown bundles with assets.

* Optional Git integration (post-v1 P2) for versioning.

### Data Storage & Privacy

* Storage: Plain-text Markdown files with optional frontmatter; media stored alongside notes in predictable folders.

* Data Flow: Local-first reads/writes; background indexing; no server dependency for core features.

* Privacy: Opt-in telemetry with minimal, aggregated metrics; transparent controls; user can disable entirely.

* Compliance: Respect platform privacy guidelines; clear data retention and deletion policies; encryption for any hosted sync.

### Scalability & Performance

* Designed to handle 50k+ notes per vault; background, incremental indexing.

* Lazy loading of large files and lists; virtualization for long lists; caching for previews and search results.

* Target p95 search latency under 75ms; navigation under 50ms; startup under 1s on modern devices.

### Potential Challenges

* Cross-platform file watcher differences and path conventions.

* Mobile constraints: background execution, memory, and file access limitations.

* Plugin security and stability; guarding against malicious or poorly performing plugins.

* Conflict resolution UX that balances simplicity with transparency.

* Internationalization: IME input handling, RTL layout support, and Unicode normalization.

---

## Milestones & Sequencing

* Phase 0: Concept & Spike (1 week)

  * Key Deliverables: Product/Engineering — editor prototype; file model; indexing approach; performance targets; initial design system.

  * Dependencies: Team alignment on Markdown spec extensions and plugin permission model.

* Phase 1: Web MVP (2 weeks)

  * Key Deliverables: Core editor (P0), vaults and files (P0), search (P0), linking/backlinks (P0), command palette (P0), basic themes (P1-lite), onboarding.

  * Dependencies: Stable parser/renderer, initial accessibility pass, telemetry opt-in.

* Phase 2: macOS Wrapper + Hardening (1 week)

  * Key Deliverables: macOS desktop packaging; filesystem permissions; crash recovery; autosave; performance tuning; basic plugin loader with starter plugins.

  * Dependencies: Web MVP stability; plugin sandbox basics.

* Phase 3: Sync via OS Providers + Beta (2 weeks)

  * Key Deliverables: Polished file-based sync with iCloud Drive/Dropbox/Google Drive via filesystem; conflict detection and resolution UI; export improvements; documentation.

  * Dependencies: Reliable file watchers; indexing resilience under churn.

* Phase 4: iOS Beta (2–3 weeks)

  * Key Deliverables: iOS editor with Files integration; offline mode; search; daily notes; theming; performance tweaks; TestFlight rollout.

  * Dependencies: Shared core engine; mobile UI optimizations; power management adherence.

* Phase 5: Post-v1 Enhancements (ongoing)

  * Key Deliverables: Optional encrypted Arke Sync, richer plugin APIs, graph visualization, preferences sync, Git integration.

### Project Estimate

* MVP (Phases 0–2): Medium — 2–4 weeks

* Beta with file-based sync (Phase 3): Small — 1–2 weeks

* iOS Beta (Phase 4): Large — 4–8 weeks (can run parallelized after Phase 2 hardening)

### Team Size & Composition

* Small Team: 1–2 total people

  * Product/Engineering lead (full-stack, performance-minded) — core editor, file/indexing, packaging.

  * Designer/Front-end (part-time or shared) — UX flows, visuals, accessibility, onboarding.

Optional support (as-needed): QA/Support for beta, technical writer for docs.

### Suggested Phases

**Concept to MVP (3 weeks)**

* Key Deliverables: Product/Engineering — editor core, vaults/files, search, linking, onboarding, initial themes; Designer — core UI, accessibility baseline.

* Dependencies: Parser/renderer selection; initial design tokens.

**Beta Hardening + macOS (1–2 weeks)**

* Key Deliverables: Engineering — desktop wrapper, crash recovery, plugin loader (starter set), performance/stability; Designer — refined layouts and empty states.

* Dependencies: MVP stability; plugin permissions.

**Sync + iOS Beta (3–6 weeks)**

* Key Deliverables: Engineering — OS-provider sync with conflict UI, iOS editor with Files integration, export polish; Designer — mobile UX patterns, responsive layouts.

* Dependencies: File watcher fidelity; mobile performance targets.