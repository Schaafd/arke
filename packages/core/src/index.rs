use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Path to the file
    pub path: PathBuf,
    /// Relevance score
    pub score: f32,
    /// Snippet of matching content
    pub snippet: String,
}

/// Search index for native platforms (using tantivy)
/// This is a placeholder for Phase 0 - full implementation in Phase 1
pub struct SearchIndex {
    indexed_files: Vec<PathBuf>,
}

impl SearchIndex {
    /// Create a new search index
    pub fn new() -> Result<Self> {
        Ok(Self {
            indexed_files: Vec::new(),
        })
    }

    /// Add a file to the index
    pub fn index_file(&mut self, path: PathBuf, _content: &str) -> Result<()> {
        // Placeholder: just track indexed files
        if !self.indexed_files.contains(&path) {
            self.indexed_files.push(path);
        }
        Ok(())
    }

    /// Remove a file from the index
    pub fn remove_file(&mut self, path: &Path) -> Result<()> {
        self.indexed_files.retain(|p| p != path);
        Ok(())
    }

    /// Search the index
    pub fn search(&self, _query: &str) -> Result<Vec<SearchResult>> {
        // Placeholder: return empty results
        // Full implementation with tantivy will come in Phase 1
        Ok(Vec::new())
    }

    /// Get stats about the index
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            num_files: self.indexed_files.len(),
            num_terms: 0,
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Statistics about the search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub num_files: usize,
    pub num_terms: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_index() {
        let index = SearchIndex::new();
        assert!(index.is_ok());
    }

    #[test]
    fn test_index_file() {
        let mut index = SearchIndex::new().unwrap();
        let path = PathBuf::from("test.md");

        index.index_file(path.clone(), "content").unwrap();

        let stats = index.stats();
        assert_eq!(stats.num_files, 1);
    }

    #[test]
    fn test_remove_file() {
        let mut index = SearchIndex::new().unwrap();
        let path = PathBuf::from("test.md");

        index.index_file(path.clone(), "content").unwrap();
        index.remove_file(&path).unwrap();

        let stats = index.stats();
        assert_eq!(stats.num_files, 0);
    }
}
