use crate::error::{ArkeError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configuration for a vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    /// Root path of the vault
    pub path: PathBuf,
    /// Name of the vault
    pub name: String,
    /// Whether to watch for file changes
    pub watch: bool,
}

/// Represents a markdown note file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// File path relative to vault root
    pub path: PathBuf,
    /// Note content (markdown)
    pub content: String,
    /// Frontmatter metadata
    pub metadata: HashMap<String, String>,
    /// Last modified timestamp
    pub modified: Option<u64>,
}

/// The Vault manages a collection of markdown files
pub struct Vault {
    config: VaultConfig,
    notes: HashMap<PathBuf, Note>,
}

impl Vault {
    /// Create a new vault with the given configuration
    pub fn new(config: VaultConfig) -> Result<Self> {
        if !config.path.exists() {
            return Err(ArkeError::Vault(format!(
                "Vault path does not exist: {}",
                config.path.display()
            )));
        }

        Ok(Self {
            config,
            notes: HashMap::new(),
        })
    }

    /// Open an existing vault at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let config = VaultConfig {
            path,
            name,
            watch: false,
        };

        Self::new(config)
    }

    /// Get the vault configuration
    pub fn config(&self) -> &VaultConfig {
        &self.config
    }

    /// List all markdown files in the vault
    pub fn list_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.walk_dir(&self.config.path, &mut files)?;
        Ok(files)
    }

    /// Recursively walk directory and collect .md files
    fn walk_dir(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and node_modules
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "node_modules" {
                        continue;
                    }
                }
                self.walk_dir(&path, files)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(relative) = path.strip_prefix(&self.config.path) {
                    files.push(relative.to_path_buf());
                }
            }
        }
        Ok(())
    }

    /// Read a note file
    pub fn read_note<P: AsRef<Path>>(&mut self, path: P) -> Result<&Note> {
        let path = path.as_ref().to_path_buf();
        let full_path = self.config.path.join(&path);

        if !full_path.exists() {
            return Err(ArkeError::FileNotFound(path.display().to_string()));
        }

        let content = std::fs::read_to_string(&full_path)?;
        let metadata = HashMap::new(); // TODO: Parse frontmatter

        let modified = full_path
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        let note = Note {
            path: path.clone(),
            content,
            metadata,
            modified,
        };

        self.notes.insert(path.clone(), note);
        Ok(self.notes.get(&path).unwrap())
    }

    /// Write a note to disk
    pub fn write_note(&mut self, path: &Path, content: &str) -> Result<()> {
        let full_path = self.config.path.join(path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&full_path, content)?;

        // Update cache
        let note = Note {
            path: path.to_path_buf(),
            content: content.to_string(),
            metadata: HashMap::new(),
            modified: None,
        };
        self.notes.insert(path.to_path_buf(), note);

        Ok(())
    }

    /// Delete a note
    pub fn delete_note(&mut self, path: &Path) -> Result<()> {
        let full_path = self.config.path.join(path);
        std::fs::remove_file(&full_path)?;
        self.notes.remove(path);
        Ok(())
    }

    /// Rename/move a note
    pub fn rename_note(&mut self, old_path: &Path, new_path: &Path) -> Result<()> {
        let old_full = self.config.path.join(old_path);
        let new_full = self.config.path.join(new_path);

        // Create parent directories for new path
        if let Some(parent) = new_full.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::rename(&old_full, &new_full)?;

        // Update cache
        if let Some(note) = self.notes.remove(old_path) {
            let mut note = note;
            note.path = new_path.to_path_buf();
            self.notes.insert(new_path.to_path_buf(), note);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_vault() -> (TempDir, Vault) {
        let temp = TempDir::new().unwrap();
        let vault = Vault::open(temp.path()).unwrap();
        (temp, vault)
    }

    #[test]
    fn test_create_vault() {
        let temp = TempDir::new().unwrap();
        let vault = Vault::open(temp.path());
        assert!(vault.is_ok());
    }

    #[test]
    fn test_write_and_read_note() {
        let (_temp, mut vault) = create_test_vault();
        let path = Path::new("test.md");
        let content = "# Test Note\n\nThis is a test.";

        vault.write_note(path, content).unwrap();
        let note = vault.read_note(path).unwrap();

        assert_eq!(note.content, content);
    }

    #[test]
    fn test_list_files() {
        let (_temp, mut vault) = create_test_vault();

        vault.write_note(Path::new("note1.md"), "content").unwrap();
        vault.write_note(Path::new("note2.md"), "content").unwrap();

        let files = vault.list_files().unwrap();
        assert_eq!(files.len(), 2);
    }
}
