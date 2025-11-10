# Arke

A cross-platform, markdown-first personal knowledge management (PKM) system built with a local-first architecture.

## Features

- **Fast Markdown Editing**: Low-latency editor built with CodeMirror 6
- **Wikilinks & Backlinks**: `[[link]]` syntax with automatic backlink tracking
- **Full-Text Search**: Instant, typo-tolerant search across all notes
- **Local-First**: Your data stays on your device as plain Markdown files
- **Cross-Platform**: Web PWA, macOS desktop, and iOS app (coming soon)
- **Plugin System**: Extensible architecture for custom functionality
- **Tags & Organization**: Flexible organization with tags, folders, and daily notes

## Project Structure

```
arke/
├── packages/
│   ├── core/          # Rust core engine (WASM + native)
│   ├── web/           # SvelteKit web application
│   ├── desktop/       # Tauri macOS wrapper (coming soon)
│   ├── mobile/        # Capacitor iOS app (coming soon)
│   └── plugins/       # Sample plugins (coming soon)
├── plans/             # Project planning and roadmaps
├── docs/              # Architecture and design documentation
└── .github/           # CI/CD workflows
```

## Technology Stack

- **Core Engine**: Rust (comrak, tantivy, serde, notify)
- **Web UI**: SvelteKit + TypeScript + CodeMirror 6
- **Desktop**: Tauri 2.x
- **Mobile**: Capacitor + Swift
- **Build Tools**: pnpm + Turborepo

## Getting Started

### Prerequisites

- Node.js 18+ and pnpm 8+
- Rust 1.70+ and Cargo
- (Optional) wasm-pack for WASM builds

### Installation

```bash
# Install dependencies
pnpm install

# Run development servers
pnpm dev
```

### Development

```bash
# Run all tests
pnpm test

# Run linters
pnpm lint

# Format code
pnpm format

# Build all packages
pnpm build
```

### Rust Core

```bash
cd packages/core

# Run Rust tests
cargo test

# Check compilation
cargo check

# Format code
cargo fmt
```

### Web Application

```bash
cd packages/web

# Run dev server
pnpm dev

# Build for production
pnpm build

# Run tests
pnpm test
```

## Development Status

**Current Phase**: Phase 0 - Spike & Foundation (Week 1)

- [x] Project setup and monorepo structure
- [x] Rust core engine with vault, parser, and links modules
- [x] SvelteKit web application foundation
- [x] CI/CD with GitHub Actions
- [ ] WASM compilation pipeline
- [ ] CodeMirror 6 integration
- [ ] Search indexing implementation

See [plans/Initial_Development_Plan.md](plans/Initial_Development_Plan.md) for the full development roadmap.

## Documentation

- **[Product Requirements](plans/Arke_PRD.md)** - Business goals, user stories, and requirements
- **[Technology Stack](plans/Arke_Tech_Stack_and_Plan.md)** - Architecture and technology decisions
- **[Development Plan](plans/Initial_Development_Plan.md)** - Detailed implementation roadmap
- **[Architecture](docs/Architecture.md)** - System architecture diagrams

## Contributing

Arke is currently in early development. Contributions will be welcome once we reach Phase 1 (Web MVP).

## License

MIT

## Contact

- GitHub: [Schaafd/arke](https://github.com/Schaafd/arke)
