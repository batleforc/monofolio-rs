use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

/// Front-matter extracted from a markdown file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MarkdownMeta {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub date: String,
}

/// Parse a markdown string (optionally with YAML front-matter) into HTML.
///
/// Returns both the parsed [`MarkdownMeta`] and the rendered HTML body.
pub fn parse_markdown_to_html(input: &str) -> (MarkdownMeta, String) {
    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let result = matter.parse(input);

    let meta: MarkdownMeta = result
        .data
        .and_then(|d| d.deserialize().ok())
        .unwrap_or_default();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&result.content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    (meta, html_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_markdown_without_frontmatter() {
        let input = "# Hello\n\nThis is **bold**.";
        let (meta, html) = parse_markdown_to_html(input);
        assert_eq!(meta.title, "");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn parses_frontmatter_and_body() {
        let input = "---\ntitle: My Post\ntags:\n  - rust\n---\n\nContent here.";
        let (meta, html) = parse_markdown_to_html(input);
        assert_eq!(meta.title, "My Post");
        assert_eq!(meta.tags, vec!["rust"]);
        assert!(html.contains("Content here."));
    }

    #[test]
    fn renders_tables() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |\n";
        let (_, html) = parse_markdown_to_html(input);
        assert!(html.contains("<table>"));
    }

    #[test]
    fn draft_defaults_to_false() {
        let input = "---\ntitle: Draft Test\n---\nBody.";
        let (meta, _) = parse_markdown_to_html(input);
        assert!(!meta.draft);
    }
}
