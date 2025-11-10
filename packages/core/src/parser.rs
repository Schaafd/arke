use crate::error::Result;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};

/// Markdown parser using comrak
pub struct MarkdownParser {
    options: ComrakOptions<'static>,
}

/// Parsed markdown document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedMarkdown {
    /// Original markdown content
    pub raw: String,
    /// Rendered HTML
    pub html: String,
    /// Extracted headings
    pub headings: Vec<Heading>,
    /// Extracted code blocks
    pub code_blocks: Vec<CodeBlock>,
}

/// A heading in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// A code block in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}

impl MarkdownParser {
    /// Create a new parser with default options
    pub fn new() -> Self {
        let mut options = ComrakOptions::default();

        // Enable GitHub-flavored markdown extensions
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;
        options.extension.superscript = false;
        options.extension.footnotes = true;
        options.extension.description_lists = true;

        // Render options
        options.render.hardbreaks = false;
        options.render.github_pre_lang = true;
        options.render.full_info_string = true;
        options.render.unsafe_ = false; // Don't allow raw HTML for security

        Self { options }
    }

    /// Parse markdown to HTML
    pub fn parse_to_html(&self, markdown: &str) -> String {
        markdown_to_html(markdown, &self.options)
    }

    /// Parse markdown and extract structure
    pub fn parse(&self, markdown: &str) -> Result<ParsedMarkdown> {
        let html = self.parse_to_html(markdown);

        Ok(ParsedMarkdown {
            raw: markdown.to_string(),
            html,
            headings: self.extract_headings(markdown),
            code_blocks: self.extract_code_blocks(markdown),
        })
    }

    /// Extract headings from markdown
    fn extract_headings(&self, markdown: &str) -> Vec<Heading> {
        let mut headings = Vec::new();
        let mut id_counter = 0;

        for line in markdown.lines() {
            let trimmed = line.trim();
            if let Some(stripped) = trimmed.strip_prefix('#') {
                let level = stripped.chars().take_while(|c| *c == '#').count() as u8 + 1;

                if level <= 6 {
                    let text = stripped.trim_start_matches('#').trim().to_string();

                    let id = format!("heading-{}", id_counter);
                    id_counter += 1;

                    headings.push(Heading { level, text, id });
                }
            }
        }

        headings
    }

    /// Extract code blocks from markdown
    fn extract_code_blocks(&self, markdown: &str) -> Vec<CodeBlock> {
        let mut code_blocks = Vec::new();
        let mut in_code_block = false;
        let mut current_lang = None;
        let mut current_code = String::new();

        for line in markdown.lines() {
            if line.trim_start().starts_with("```") {
                if in_code_block {
                    // End of code block
                    code_blocks.push(CodeBlock {
                        language: current_lang.take(),
                        code: current_code.clone(),
                    });
                    current_code.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    let lang = line.trim_start().trim_start_matches("```").trim();
                    current_lang = if lang.is_empty() {
                        None
                    } else {
                        Some(lang.to_string())
                    };
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(line);
                current_code.push('\n');
            }
        }

        code_blocks
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown() {
        let parser = MarkdownParser::new();
        let html = parser.parse_to_html("# Hello\n\nWorld");
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn test_extract_headings() {
        let parser = MarkdownParser::new();
        let md = "# Title\n## Subtitle\n### Section";
        let parsed = parser.parse(md).unwrap();

        assert_eq!(parsed.headings.len(), 3);
        assert_eq!(parsed.headings[0].level, 1);
        assert_eq!(parsed.headings[0].text, "Title");
        assert_eq!(parsed.headings[1].level, 2);
        assert_eq!(parsed.headings[2].level, 3);
    }

    #[test]
    fn test_extract_code_blocks() {
        let parser = MarkdownParser::new();
        let md = "# Code\n\n```rust\nfn main() {}\n```\n\nText\n\n```python\nprint('hi')\n```";
        let parsed = parser.parse(md).unwrap();

        assert_eq!(parsed.code_blocks.len(), 2);
        assert_eq!(parsed.code_blocks[0].language, Some("rust".to_string()));
        assert_eq!(parsed.code_blocks[1].language, Some("python".to_string()));
    }

    #[test]
    fn test_gfm_extensions() {
        let parser = MarkdownParser::new();

        // Test tables
        let table_md = "| Col 1 | Col 2 |\n|-------|-------|\n| A | B |";
        let html = parser.parse_to_html(table_md);
        assert!(html.contains("<table"));

        // Test strikethrough
        let strike_md = "~~deleted~~";
        let html = parser.parse_to_html(strike_md);
        assert!(html.contains("<del>") || html.contains("strike"));

        // Test task lists
        let task_md = "- [ ] Task 1\n- [x] Task 2";
        let html = parser.parse_to_html(task_md);
        assert!(html.contains("checkbox") || html.contains("task"));
    }
}
