# Arke Core

The core Rust engine for Arke PKM system.

## Features

- Vault management and file I/O
- Markdown parsing with comrak
- Wikilink extraction and backlinks
- Full-text search and indexing
- Cross-platform (WASM + native)

## Development

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build for native
cargo build --release

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```
