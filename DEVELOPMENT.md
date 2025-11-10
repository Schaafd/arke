# Development Workflow

## Git Workflow

Arke follows a feature-branch workflow for all development tasks.

### Branch Naming Convention

```
feature/<task-name>
```

Examples:

- `feature/editor-prototype`
- `feature/wasm-compilation`
- `feature/search-integration`
- `feature/design-system`

### Workflow Steps

1. **Create Feature Branch**

   ```bash
   git checkout -b feature/<task-name>
   ```

2. **Develop & Test**
   - Make incremental commits
   - Write tests as you go
   - Run tests frequently: `pnpm test` and `cargo test`

3. **Pre-Merge Checklist**
   - [ ] All tests passing (`cargo test` and `pnpm test`)
   - [ ] Code formatted (`cargo fmt` and `pnpm format`)
   - [ ] Linting passes (`cargo clippy` and `pnpm lint`)
   - [ ] Feature is complete and working

4. **Merge to Main**

   ```bash
   git checkout main
   git merge feature/<task-name>
   git push origin main
   ```

5. **Delete Feature Branch** (optional)
   ```bash
   git branch -d feature/<task-name>
   ```

### Commit Message Format

Follow conventional commits format:

```
<type>(<scope>): <description>

<body>

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Types:**

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Formatting, missing semicolons, etc.
- `refactor:` - Code restructuring
- `test:` - Adding tests
- `chore:` - Maintenance tasks

**Scopes:**

- `core` - Rust core engine
- `web` - SvelteKit web app
- `desktop` - Tauri desktop app
- `mobile` - Capacitor mobile app
- `ci` - CI/CD changes
- `docs` - Documentation

**Examples:**

```
feat(web): integrate CodeMirror 6 editor with syntax highlighting
feat(core): implement WASM compilation pipeline
fix(web): resolve File System Access API permissions issue
```

## Development Commands

### Root Level (Monorepo)

```bash
# Install all dependencies
pnpm install

# Run all dev servers
pnpm dev

# Run all tests
pnpm test

# Run all linters
pnpm lint

# Format all code
pnpm format

# Build all packages
pnpm build

# Clean all build artifacts
pnpm clean
```

### Rust Core (`packages/core/`)

```bash
cd packages/core

# Check compilation
cargo check

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Build for native
cargo build --release

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

### Web App (`packages/web/`)

```bash
cd packages/web

# Run dev server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview

# Run tests
pnpm test

# Run tests in watch mode
pnpm test -- --watch

# Type check
pnpm check

# Lint
pnpm lint

# Format
pnpm format
```

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with logging
RUST_LOG=debug cargo test

# Run tests with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

### Web Tests

```bash
# Run all tests
pnpm test

# Run in watch mode
pnpm test -- --watch

# Run with coverage
pnpm test -- --coverage

# Run specific test file
pnpm test src/lib/component.test.ts
```

## Performance Targets

Monitor these during development:

- **Cold start**: <1s
- **Keystroke latency**: <10ms
- **File switching**: <50ms
- **Search (p95)**: <75ms
- **Crash-free sessions**: ≥99.9%

Use browser DevTools Performance tab and Rust benchmarks to measure.

## Phase 0 Tasks (Current)

Following feature branches in order:

1. ✅ **Project Setup** - `main` (completed)
2. 🔄 **Editor Prototype** - `feature/editor-prototype` (next)
3. ⏳ **WASM Compilation** - `feature/wasm-compilation`
4. ⏳ **Search Integration** - `feature/search-integration`
5. ⏳ **Design System** - `feature/design-system`

## Troubleshooting

### WASM Build Fails

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### pnpm Install Fails

```bash
# Clear pnpm cache
pnpm store prune

# Remove lockfile and node_modules
rm -rf node_modules pnpm-lock.yaml

# Reinstall
pnpm install
```

### Tests Failing

```bash
# Clean and rebuild
pnpm clean
cargo clean
pnpm install
cargo build
```
