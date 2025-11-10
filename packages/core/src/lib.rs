//! Arke Core Engine
//!
//! This is the core library for the Arke PKM system, providing:
//! - Vault management and file I/O
//! - Markdown parsing with comrak
//! - Wikilink extraction and backlinks
//! - Full-text search and indexing
//! - Export functionality
//!
//! The library compiles to both native (via Rust) and WASM (for web).

pub mod error;
pub mod links;
pub mod parser;
pub mod vault;

#[cfg(feature = "native")]
pub mod index;

// Re-export commonly used types
pub use error::{ArkeError, Result};
pub use links::{BacklinksMap, WikiLink};
pub use parser::MarkdownParser;
pub use vault::{Vault, VaultConfig};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
