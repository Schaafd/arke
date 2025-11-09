# Arke Architecture Documentation

This document provides visual architecture diagrams for the Arke PKM application.

## Table of Contents

- [System Overview](#system-overview)
- [Core Engine Architecture](#core-engine-architecture)
- [Platform Architecture](#platform-architecture)
- [Data Flow](#data-flow)
- [Plugin System Architecture](#plugin-system-architecture)
- [Sync Architecture](#sync-architecture)

---

## System Overview

High-level view of Arke's architecture showing the core engine, platform shells, and external integrations.

```mermaid
graph TB
    subgraph "User Interfaces"
        WebUI[Web PWA<br/>SvelteKit + CodeMirror]
        DesktopUI[macOS Desktop<br/>Tauri Wrapper]
        MobileUI[iOS App<br/>Capacitor + Swift]
    end

    subgraph "Core Engine (Rust)"
        VaultMgr[Vault Manager<br/>File I/O]
        Parser[Markdown Parser<br/>comrak]
        LinkEngine[Link Engine<br/>Wikilinks & Backlinks]
        Indexer[Search Indexer<br/>tantivy/FlexSearch]
        PluginAPI[Plugin API Surface]
        Exporter[Export Engine<br/>HTML/PDF/MD]
    end

    subgraph "Storage Layer"
        LocalFS[Local Filesystem<br/>Plain .md files]
        Assets[Assets Folder<br/>Images, PDFs]
    end

    subgraph "External Services"
        iCloud[iCloud Drive]
        Dropbox[Dropbox]
        GDrive[Google Drive]
        OptionalSync[Arke Sync<br/>Post-v1]
    end

    WebUI --> |WASM| VaultMgr
    DesktopUI --> |Native| VaultMgr
    MobileUI --> |Native| VaultMgr

    VaultMgr --> Parser
    VaultMgr --> LinkEngine
    Parser --> Indexer
    LinkEngine --> Indexer
    VaultMgr --> Exporter
    VaultMgr --> PluginAPI

    VaultMgr --> LocalFS
    VaultMgr --> Assets

    LocalFS -.Sync via OS.-> iCloud
    LocalFS -.Sync via OS.-> Dropbox
    LocalFS -.Sync via OS.-> GDrive
    LocalFS -.Optional E2E.-> OptionalSync

    style WebUI fill:#4A90E2
    style DesktopUI fill:#4A90E2
    style MobileUI fill:#4A90E2
    style VaultMgr fill:#E27D60
    style Parser fill:#E27D60
    style LinkEngine fill:#E27D60
    style Indexer fill:#E27D60
    style LocalFS fill:#85CDCA
```

---

## Core Engine Architecture

Detailed view of the Rust core engine and its compilation targets.

```mermaid
graph LR
    subgraph "Rust Core Library"
        direction TB
        Vault[vault::<br/>Vault I/O & Watching]
        Parse[parser::<br/>Markdown Parsing]
        Links[links::<br/>Wikilink Extraction]
        Index[index::<br/>Full-text Search]
        Export[export::<br/>Format Conversion]
        Plugin[plugin::<br/>Plugin Interface]
        Config[config::<br/>Settings Management]
    end

    subgraph "Compilation Targets"
        WASM[wasm32-unknown-unknown<br/>Web Browser]
        MacOS[aarch64/x86_64-apple-darwin<br/>macOS Native]
        iOS[aarch64-apple-ios<br/>iOS Native]
    end

    Vault --> Parse
    Parse --> Links
    Parse --> Index
    Vault --> Export
    Vault --> Plugin
    Config --> Vault

    Vault -.wasm-bindgen.-> WASM
    Vault -.cargo build.-> MacOS
    Vault -.cargo build.-> iOS

    style Vault fill:#E27D60
    style Parse fill:#E27D60
    style Links fill:#E27D60
    style Index fill:#E27D60
    style WASM fill:#F39C12
    style MacOS fill:#F39C12
    style iOS fill:#F39C12
```

---

## Platform Architecture

How each platform integrates the core engine with its UI layer.

```mermaid
graph TB
    subgraph "Web Platform"
        WebApp[SvelteKit App]
        CM6[CodeMirror 6 Editor]
        WebWorker[Web Workers<br/>Parse & Index]
        WASM_Core[Core Engine WASM]
        FSAccess[File System Access API]

        WebApp --> CM6
        WebApp --> WebWorker
        WebWorker --> WASM_Core
        WASM_Core --> FSAccess
    end

    subgraph "Desktop Platform (macOS)"
        TauriApp[Tauri App Shell]
        WebView[WebView<br/>SvelteKit UI]
        NativeCore[Core Engine Native]
        NativeFS[Native FS APIs]

        TauriApp --> WebView
        TauriApp --> NativeCore
        NativeCore --> NativeFS
    end

    subgraph "Mobile Platform (iOS)"
        CapApp[Capacitor Shell]
        iOSWebView[iOS WebView<br/>SvelteKit UI]
        SwiftBridge[Swift Bridge]
        iOSCore[Core Engine Native]
        FilesProvider[iOS Files Provider]

        CapApp --> iOSWebView
        CapApp --> SwiftBridge
        SwiftBridge --> iOSCore
        iOSCore --> FilesProvider
    end

    style WebApp fill:#4A90E2
    style TauriApp fill:#4A90E2
    style CapApp fill:#4A90E2
    style WASM_Core fill:#E27D60
    style NativeCore fill:#E27D60
    style iOSCore fill:#E27D60
```

---

## Data Flow

Key data flows for common operations.

### File Opening Flow

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant Core
    participant FS as Filesystem
    participant Index

    User->>UI: Click file in tree
    UI->>Core: open_file(path)
    Core->>FS: read_file(path)
    FS-->>Core: file_content
    Core->>Core: parse_markdown()
    Core->>Core: extract_wikilinks()
    Core-->>UI: {content, links, metadata}
    UI->>UI: render_in_editor()

    par Background Indexing
        Core->>Index: update_index(file)
        Index->>Index: tokenize & store
    end
```

### Search Flow

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant Worker
    participant Index
    participant Ranker

    User->>UI: Type in search box
    UI->>Worker: search(query)
    Worker->>Index: query_index(tokens)
    Index-->>Worker: matching_docs[]
    Worker->>Ranker: rank_by_relevance()
    Ranker-->>Worker: sorted_results[]
    Worker-->>UI: results with previews
    UI->>UI: render_results()

    Note over UI,Index: Target: <75ms p95
```

### Link Creation & Backlinks Flow

```mermaid
sequenceDiagram
    participant User
    participant Editor
    participant Core
    participant LinkIndex

    User->>Editor: Type [[note
    Editor->>Core: autocomplete_request()
    Core->>LinkIndex: find_matching_files()
    LinkIndex-->>Editor: suggestions[]
    Editor->>User: Show suggestions

    User->>Editor: Select/complete link
    Editor->>Core: notify_link_created(from, to)
    Core->>LinkIndex: update_backlink(to, from)
    LinkIndex->>Core: get_backlinks(to)
    Core-->>Editor: updated_backlinks[]
    Editor->>Editor: refresh_backlinks_panel()
```

### File Sync & Conflict Detection

```mermaid
sequenceDiagram
    participant Watcher
    participant Core
    participant FS
    participant UI
    participant User

    Watcher->>Core: file_changed(path)
    Core->>FS: read_file(path)
    Core->>Core: compare_timestamps()

    alt No Conflict
        Core->>UI: reload_file()
        UI->>UI: update_editor()
    else Conflict Detected
        Core->>UI: show_conflict_dialog()
        UI->>User: Choose: Keep Local/Remote/Merge
        User->>UI: Selection
        UI->>Core: resolve_conflict(choice)
        Core->>FS: write_file()
    end
```

---

## Plugin System Architecture

How plugins integrate with the core system.

```mermaid
graph TB
    subgraph "Plugin Runtime Environment"
        direction TB
        PluginLoader[Plugin Loader]
        Sandbox[Sandbox<br/>Web Worker / Sidecar]
        PermMgr[Permission Manager]
    end

    subgraph "Plugin Instance"
        Manifest[manifest.json<br/>name, version, capabilities]
        PluginCode[Plugin Code<br/>JS/TS]
        PluginUI[UI Components<br/>Panels, Commands]
    end

    subgraph "Core APIs"
        FileAPI[File Access API<br/>Scoped to Vault]
        UIAPI[UI Extension API<br/>Commands, Panels]
        EventAPI[Event API<br/>Hooks & Listeners]
        NetworkAPI[Network API<br/>Optional Permission]
    end

    subgraph "Sample Plugins"
        Tasks[Task Dashboard]
        Kanban[Kanban Board]
        Chat[Note Chat]
    end

    PluginLoader --> Manifest
    Manifest --> PermMgr
    PermMgr --> Sandbox
    Sandbox --> PluginCode
    PluginCode --> PluginUI

    PluginCode -.request.-> FileAPI
    PluginCode -.request.-> UIAPI
    PluginCode -.request.-> EventAPI
    PluginCode -.request.-> NetworkAPI

    Tasks --> PluginCode
    Kanban --> PluginCode
    Chat --> PluginCode

    style PluginLoader fill:#9B59B6
    style Sandbox fill:#9B59B6
    style PermMgr fill:#E74C3C
    style FileAPI fill:#85CDCA
    style UIAPI fill:#85CDCA
```

### Plugin Permission Flow

```mermaid
sequenceDiagram
    participant User
    participant PluginMgr
    participant PermMgr
    participant Plugin
    participant CoreAPI

    User->>PluginMgr: Install Plugin
    PluginMgr->>PermMgr: Read manifest.json
    PermMgr->>User: Show permission dialog
    User->>PermMgr: Grant permissions
    PermMgr->>PluginMgr: Load plugin
    PluginMgr->>Plugin: Initialize

    Plugin->>CoreAPI: API call (e.g., read_file)
    CoreAPI->>PermMgr: Check permission

    alt Permission Granted
        PermMgr-->>CoreAPI: Allowed
        CoreAPI-->>Plugin: Return data
    else Permission Denied
        PermMgr-->>CoreAPI: Denied
        CoreAPI-->>Plugin: Error: Permission denied
    end
```

---

## Sync Architecture

How synchronization works across devices.

```mermaid
graph TB
    subgraph "Device A"
        VaultA[Vault Folder]
        AppA[Arke App A]
        WatcherA[File Watcher]
    end

    subgraph "Sync Provider (OS-Level)"
        iCloudSync[iCloud Drive Sync]
        DropboxSync[Dropbox Sync]
        GDriveSync[Google Drive Sync]
    end

    subgraph "Device B"
        VaultB[Vault Folder]
        AppB[Arke App B]
        WatcherB[File Watcher]
    end

    subgraph "Future: Arke Sync Service"
        E2E[E2E Encryption]
        SyncServer[Sync Server]
        ConflictRes[Conflict Resolution]
    end

    AppA --> VaultA
    VaultA <-.OS Sync.-> iCloudSync
    VaultA <-.OS Sync.-> DropboxSync
    VaultA <-.OS Sync.-> GDriveSync

    iCloudSync <-.OS Sync.-> VaultB
    DropboxSync <-.OS Sync.-> VaultB
    GDriveSync <-.OS Sync.-> VaultB

    VaultB --> AppB
    WatcherA -.Monitor.-> VaultA
    WatcherB -.Monitor.-> VaultB

    VaultA -.Optional.-> E2E
    E2E --> SyncServer
    SyncServer --> ConflictRes
    ConflictRes --> E2E
    E2E -.Optional.-> VaultB

    style AppA fill:#4A90E2
    style AppB fill:#4A90E2
    style iCloudSync fill:#95E1D3
    style SyncServer fill:#F38181
    style E2E fill:#E74C3C
```

### Conflict Resolution Flow

```mermaid
stateDiagram-v2
    [*] --> Monitoring: File Watcher Active

    Monitoring --> ChangeDetected: File modified externally
    ChangeDetected --> CheckTimestamp: Read file metadata

    CheckTimestamp --> NoConflict: Local == Remote timestamp
    CheckTimestamp --> Conflict: Local > Remote timestamp

    NoConflict --> Reload: Update editor
    Reload --> Monitoring

    Conflict --> ShowDialog: Present options to user
    ShowDialog --> KeepLocal: User chooses local
    ShowDialog --> KeepRemote: User chooses remote
    ShowDialog --> ManualMerge: User chooses merge

    KeepLocal --> WriteLocal: Overwrite remote
    KeepRemote --> ReloadRemote: Discard local
    ManualMerge --> DiffView: Show side-by-side

    DiffView --> UserEdit: User edits merged version
    UserEdit --> SaveMerged: Write resolved file

    WriteLocal --> Monitoring
    ReloadRemote --> Monitoring
    SaveMerged --> Monitoring
```

---

## Component Dependencies

Module dependency graph for the core engine.

```mermaid
graph TD
    CLI[CLI Entry]
    Config[config]
    Vault[vault]
    Parser[parser]
    Links[links]
    Index[index]
    Export[export]
    Plugin[plugin]
    FS[filesystem]

    CLI --> Config
    CLI --> Vault
    Vault --> Config
    Vault --> Parser
    Vault --> FS
    Parser --> Links
    Parser --> Index
    Links --> Index
    Vault --> Export
    Export --> Parser
    Vault --> Plugin
    Plugin --> Vault

    style CLI fill:#95A5A6
    style Vault fill:#E27D60
    style Parser fill:#E27D60
    style Index fill:#E27D60
    style Config fill:#F39C12
```

---

## Technology Stack Summary

```mermaid
graph LR
    subgraph "Frontend Stack"
        SvelteKit[SvelteKit<br/>UI Framework]
        CM6[CodeMirror 6<br/>Editor]
        Tailwind[Tailwind CSS<br/>Styling]
        Unified[unified/remark/rehype<br/>Markdown Preview]
    end

    subgraph "Core Stack"
        Rust[Rust<br/>Core Engine]
        Comrak[comrak<br/>Markdown Parser]
        Tantivy[tantivy<br/>Search Index]
        Serde[serde<br/>Serialization]
        Notify[notify<br/>File Watching]
    end

    subgraph "Platform Stack"
        Tauri[Tauri 2.x<br/>Desktop]
        Capacitor[Capacitor<br/>Mobile]
        WASM[wasm-bindgen<br/>Web]
    end

    subgraph "Build Stack"
        PNPM[pnpm<br/>Package Manager]
        Turbo[Turborepo<br/>Monorepo]
        Vite[Vite<br/>Build Tool]
    end

    SvelteKit --> CM6
    SvelteKit --> Tailwind
    SvelteKit --> Unified

    Rust --> Comrak
    Rust --> Tantivy
    Rust --> Serde
    Rust --> Notify

    Rust --> WASM
    Rust --> Tauri
    Rust --> Capacitor

    style Rust fill:#E27D60
    style SvelteKit fill:#4A90E2
    style Tauri fill:#9B59B6
```

---

## Performance Architecture

Key performance optimizations built into the architecture.

```mermaid
graph TB
    subgraph "UI Thread"
        Editor[Editor<br/>Main Thread]
        UI[UI Rendering]
    end

    subgraph "Background Workers"
        ParseWorker[Parse Worker<br/>Markdown Processing]
        IndexWorker[Index Worker<br/>Search Indexing]
        ExportWorker[Export Worker<br/>Format Conversion]
    end

    subgraph "Optimizations"
        LazyLoad[Lazy Loading<br/>Large Files]
        Virtual[Virtual Scrolling<br/>Long Lists]
        Cache[Cache Layer<br/>Previews & Search]
        Incremental[Incremental Updates<br/>Index & Backlinks]
    end

    Editor --> ParseWorker
    Editor --> LazyLoad
    UI --> Virtual
    ParseWorker --> Cache
    IndexWorker --> Incremental
    IndexWorker --> Cache

    style Editor fill:#4A90E2
    style ParseWorker fill:#F39C12
    style IndexWorker fill:#F39C12
    style Cache fill:#85CDCA
```

---

## Notes

- **WASM Compilation:** Core Rust engine compiles to WebAssembly for web platform using `wasm-bindgen`
- **Native Compilation:** Same Rust code compiles natively for macOS and iOS with platform-specific file I/O
- **UI Sharing:** SvelteKit app is shared across web, desktop (via Tauri webview), and mobile (via Capacitor webview)
- **Plugin Isolation:** Plugins run in Web Workers (web) or Tauri sidecars (desktop) for security and stability
- **Local-First:** All operations work offline; sync is handled by OS-level file providers or optional Arke Sync service
- **Performance:** Workers prevent blocking UI thread; incremental indexing and caching ensure responsiveness
