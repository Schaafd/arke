use thiserror::Error;

/// Result type for Arke operations
pub type Result<T> = std::result::Result<T, ArkeError>;

/// Error types for the Arke core engine
#[derive(Error, Debug)]
pub enum ArkeError {
    #[error("Vault error: {0}")]
    Vault(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid wikilink: {0}")]
    InvalidWikilink(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[cfg(feature = "native")]
    #[error("Index error: {0}")]
    Index(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
