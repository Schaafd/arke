use crate::error::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents a wikilink [[link]]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WikiLink {
    /// The raw link text (e.g., "note" from [[note]])
    pub target: String,
    /// Optional display text (e.g., "display" from [[note|display]])
    pub display: Option<String>,
    /// Position in the source text
    pub position: usize,
}

/// Maps files to their outbound links
pub type LinksMap = HashMap<PathBuf, Vec<WikiLink>>;

/// Maps files to their inbound links (backlinks)
pub type BacklinksMap = HashMap<PathBuf, Vec<PathBuf>>;

/// Extracts and manages wikilinks
pub struct LinkExtractor {
    wikilink_regex: Regex,
}

impl LinkExtractor {
    /// Create a new link extractor
    pub fn new() -> Self {
        // Matches [[target]] or [[target|display]]
        let wikilink_regex =
            Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").expect("Invalid wikilink regex");

        Self { wikilink_regex }
    }

    /// Extract all wikilinks from markdown content
    pub fn extract(&self, content: &str) -> Vec<WikiLink> {
        let mut links = Vec::new();

        for cap in self.wikilink_regex.captures_iter(content) {
            let target = cap.get(1).unwrap().as_str().trim().to_string();
            let display = cap.get(2).map(|m| m.as_str().trim().to_string());
            let position = cap.get(0).unwrap().start();

            links.push(WikiLink {
                target,
                display,
                position,
            });
        }

        links
    }

    /// Resolve a wikilink target to a file path
    /// Tries .md extension if not present
    pub fn resolve_link(&self, target: &str, vault_files: &[PathBuf]) -> Option<PathBuf> {
        let target_lower = target.to_lowercase();

        // Try exact match first
        for file in vault_files {
            if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
                if stem.to_lowercase() == target_lower {
                    return Some(file.clone());
                }
            }
        }

        // Try with .md extension
        let target_with_ext = if !target.ends_with(".md") {
            format!("{}.md", target)
        } else {
            target.to_string()
        };

        for file in vault_files {
            if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase() == target_with_ext.to_lowercase() {
                    return Some(file.clone());
                }
            }
        }

        None
    }

    /// Build a map of outbound links for all files
    pub fn build_links_map(&self, files: &[(PathBuf, String)]) -> LinksMap {
        let mut map = HashMap::new();

        for (path, content) in files {
            let links = self.extract(content);
            if !links.is_empty() {
                map.insert(path.clone(), links);
            }
        }

        map
    }

    /// Build a backlinks map from a links map
    pub fn build_backlinks_map(
        &self,
        links_map: &LinksMap,
        vault_files: &[PathBuf],
    ) -> BacklinksMap {
        let mut backlinks: BacklinksMap = HashMap::new();

        for (source_path, links) in links_map {
            for link in links {
                if let Some(target_path) = self.resolve_link(&link.target, vault_files) {
                    backlinks
                        .entry(target_path)
                        .or_insert_with(Vec::new)
                        .push(source_path.clone());
                }
            }
        }

        // Remove duplicates
        for (_, sources) in backlinks.iter_mut() {
            sources.sort();
            sources.dedup();
        }

        backlinks
    }

    /// Get all backlinks for a specific file
    pub fn get_backlinks(&self, file: &Path, backlinks_map: &BacklinksMap) -> Vec<PathBuf> {
        backlinks_map.get(file).cloned().unwrap_or_default()
    }

    /// Find broken links (links that don't resolve to any file)
    pub fn find_broken_links(
        &self,
        links_map: &LinksMap,
        vault_files: &[PathBuf],
    ) -> HashMap<PathBuf, Vec<String>> {
        let mut broken = HashMap::new();

        for (source_path, links) in links_map {
            let mut broken_targets = Vec::new();

            for link in links {
                if self.resolve_link(&link.target, vault_files).is_none() {
                    broken_targets.push(link.target.clone());
                }
            }

            if !broken_targets.is_empty() {
                broken.insert(source_path.clone(), broken_targets);
            }
        }

        broken
    }
}

impl Default for LinkExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_wikilink() {
        let extractor = LinkExtractor::new();
        let content = "This is a [[test]] link.";
        let links = extractor.extract(content);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "test");
        assert_eq!(links[0].display, None);
    }

    #[test]
    fn test_extract_wikilink_with_display() {
        let extractor = LinkExtractor::new();
        let content = "See [[target|display text]].";
        let links = extractor.extract(content);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "target");
        assert_eq!(links[0].display, Some("display text".to_string()));
    }

    #[test]
    fn test_extract_multiple_wikilinks() {
        let extractor = LinkExtractor::new();
        let content = "Links: [[one]], [[two]], and [[three|3]].";
        let links = extractor.extract(content);

        assert_eq!(links.len(), 3);
        assert_eq!(links[0].target, "one");
        assert_eq!(links[1].target, "two");
        assert_eq!(links[2].target, "three");
        assert_eq!(links[2].display, Some("3".to_string()));
    }

    #[test]
    fn test_resolve_link() {
        let extractor = LinkExtractor::new();
        let vault_files = vec![
            PathBuf::from("notes/test.md"),
            PathBuf::from("docs/readme.md"),
        ];

        // Resolve without extension
        let resolved = extractor.resolve_link("test", &vault_files);
        assert_eq!(resolved, Some(PathBuf::from("notes/test.md")));

        // Resolve with extension
        let resolved = extractor.resolve_link("readme.md", &vault_files);
        assert_eq!(resolved, Some(PathBuf::from("docs/readme.md")));

        // Non-existent file
        let resolved = extractor.resolve_link("nonexistent", &vault_files);
        assert_eq!(resolved, None);
    }

    #[test]
    fn test_build_backlinks_map() {
        let extractor = LinkExtractor::new();

        let files = vec![
            (PathBuf::from("a.md"), "Link to [[b]]".to_string()),
            (
                PathBuf::from("c.md"),
                "Links to [[b]] and [[a]]".to_string(),
            ),
        ];

        let vault_files = vec![
            PathBuf::from("a.md"),
            PathBuf::from("b.md"),
            PathBuf::from("c.md"),
        ];

        let links_map = extractor.build_links_map(&files);
        let backlinks_map = extractor.build_backlinks_map(&links_map, &vault_files);

        // b.md should have backlinks from a.md and c.md
        let b_backlinks = backlinks_map.get(&PathBuf::from("b.md")).unwrap();
        assert_eq!(b_backlinks.len(), 2);
        assert!(b_backlinks.contains(&PathBuf::from("a.md")));
        assert!(b_backlinks.contains(&PathBuf::from("c.md")));

        // a.md should have backlink from c.md
        let a_backlinks = backlinks_map.get(&PathBuf::from("a.md")).unwrap();
        assert_eq!(a_backlinks.len(), 1);
        assert_eq!(a_backlinks[0], PathBuf::from("c.md"));
    }

    #[test]
    fn test_find_broken_links() {
        let extractor = LinkExtractor::new();

        let files = vec![(
            PathBuf::from("a.md"),
            "Link to [[exists]] and [[missing]]".to_string(),
        )];

        let vault_files = vec![PathBuf::from("a.md"), PathBuf::from("exists.md")];

        let links_map = extractor.build_links_map(&files);
        let broken = extractor.find_broken_links(&links_map, &vault_files);

        assert_eq!(broken.len(), 1);
        let broken_in_a = broken.get(&PathBuf::from("a.md")).unwrap();
        assert_eq!(broken_in_a.len(), 1);
        assert_eq!(broken_in_a[0], "missing");
    }
}
