use std::collections::BTreeMap;

use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag};
use serde::{Deserialize, Serialize};

/// Routing flags declared in markdown front-matter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MarkdownSpec {
    #[serde(default)]
    pub blog: bool,
    #[serde(default)]
    pub project: bool,
    #[serde(default)]
    pub doc: bool,
}

/// Fixed maturity levels used by technology pages shown on the mindmap page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TechnologyMaturity {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Front-matter settings for the technology mindmap page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MindmapMeta {
    #[serde(default)]
    pub include: bool,
    #[serde(default)]
    pub maturity: Option<TechnologyMaturity>,
}

fn default_release_true() -> bool {
    true
}

/// Generic named link declared in front-matter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MarkdownLink {
    pub name: String,
    pub url: String,
}

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
    #[serde(default)]
    pub spec: MarkdownSpec,
    #[serde(default)]
    pub links: Vec<MarkdownLink>,
    #[serde(default)]
    pub techno: Vec<String>,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub mindmap: MindmapMeta,
    #[serde(default = "default_release_true")]
    pub release: bool,
}

/// A heading found while parsing markdown.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkdownHeading {
    pub level: u8,
    pub id: String,
    pub title: String,
}

/// Full parsed markdown output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MarkdownDocument {
    pub meta: MarkdownMeta,
    pub content: MarkdownContent,
    pub headings: Vec<MarkdownHeading>,
    pub reading_time_minutes: usize,
}

/// JSON payload persisted in the content database and rendered by the front.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MarkdownContent {
    pub format: String,
    pub nodes: Vec<MarkdownNode>,
}

/// A renderable JSON AST node for markdown content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MarkdownNode {
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MarkdownNode>,
}

/// Resolve known in-content handles to public routes.
pub fn resolve_handle(raw: &str) -> String {
    if let Some(rest) = raw.strip_prefix("media#") {
        return format!("/media/{rest}");
    }
    if let Some(rest) = raw.strip_prefix("blog#") {
        return format!("/blog/{rest}");
    }
    if let Some(rest) = raw.strip_prefix("doc#") {
        return format!("/docs/{rest}");
    }
    if let Some(rest) = raw.strip_prefix("project#") {
        return format!("/projects/{rest}");
    }

    raw.to_string()
}

fn normalize_handles(input: &str) -> String {
    input
        .replace("media#", "/media/")
        .replace("blog#", "/blog/")
        .replace("doc#", "/docs/")
        .replace("project#", "/projects/")
}

fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn slugify(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut previous_dash = false;

    for ch in value.chars().flat_map(|c| c.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn reading_time_minutes(markdown: &str) -> usize {
    let words = markdown.split_whitespace().count();
    if words == 0 {
        return 0;
    }

    words.div_ceil(200)
}

fn parse_frontmatter_and_content(input: &str) -> (MarkdownMeta, String) {
    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let (raw_meta, content) = match matter.parse::<MarkdownMeta>(input) {
        Ok(parsed) => (parsed.data, parsed.content),
        Err(_) => (None, input.to_string()),
    };

    let mut meta: MarkdownMeta = raw_meta.unwrap_or_default();

    meta.image = resolve_handle(&meta.image);
    meta.links = meta
        .links
        .into_iter()
        .map(|mut link| {
            link.url = resolve_handle(&link.url);
            link
        })
        .collect();

    (meta, normalize_handles(&content))
}

fn parser_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

fn render_markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, parser_options());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn text_node(kind: &str, text: impl Into<String>) -> MarkdownNode {
    MarkdownNode {
        kind: kind.to_string(),
        text: text.into(),
        attrs: BTreeMap::new(),
        children: Vec::new(),
    }
}

fn container_node(kind: &str) -> MarkdownNode {
    MarkdownNode {
        kind: kind.to_string(),
        text: String::new(),
        attrs: BTreeMap::new(),
        children: Vec::new(),
    }
}

enum AstFrame {
    Node(MarkdownNode),
}

fn append_ast_node(stack: &mut [AstFrame], root: &mut Vec<MarkdownNode>, node: MarkdownNode) {
    match stack.last_mut() {
        Some(AstFrame::Node(parent)) => parent.children.push(node),
        _ => root.push(node),
    }
}

fn node_text(node: &MarkdownNode) -> String {
    let mut text = node.text.clone();
    for child in &node.children {
        text.push_str(&node_text(child));
    }
    text
}

fn ast_node_from_tag(tag: Tag<'_>) -> MarkdownNode {
    let mut node = match tag {
        Tag::Paragraph => container_node("paragraph"),
        Tag::Heading { level, .. } => {
            let mut node = container_node("heading");
            node.attrs
                .insert("level".to_string(), heading_level_to_u8(level).to_string());
            node
        }
        Tag::BlockQuote(kind) => {
            let mut node = container_node("blockquote");
            if let Some(kind) = kind {
                node.attrs.insert("kind".to_string(), format!("{kind:?}"));
            }
            node
        }
        Tag::CodeBlock(kind) => {
            let mut node = container_node("code_block");
            match kind {
                pulldown_cmark::CodeBlockKind::Indented => {
                    node.attrs
                        .insert("style".to_string(), "indented".to_string());
                }
                pulldown_cmark::CodeBlockKind::Fenced(language) => {
                    node.attrs.insert("style".to_string(), "fenced".to_string());
                    if !language.is_empty() {
                        node.attrs
                            .insert("language".to_string(), language.to_string());
                    }
                }
            }
            node
        }
        Tag::HtmlBlock => container_node("html_block"),
        Tag::List(start) => {
            let mut node = container_node("list");
            if let Some(start) = start {
                node.attrs.insert("ordered".to_string(), "true".to_string());
                node.attrs.insert("start".to_string(), start.to_string());
            } else {
                node.attrs
                    .insert("ordered".to_string(), "false".to_string());
            }
            node
        }
        Tag::Item => container_node("item"),
        Tag::FootnoteDefinition(name) => {
            let mut node = container_node("footnote_definition");
            node.attrs.insert("name".to_string(), name.to_string());
            node
        }
        Tag::Table(alignments) => {
            let mut node = container_node("table");
            let alignments = alignments
                .into_iter()
                .map(|alignment| format!("{alignment:?}"))
                .collect::<Vec<_>>()
                .join(",");
            if !alignments.is_empty() {
                node.attrs.insert("alignments".to_string(), alignments);
            }
            node
        }
        Tag::TableHead => container_node("table_head"),
        Tag::TableRow => container_node("table_row"),
        Tag::TableCell => container_node("table_cell"),
        Tag::Emphasis => container_node("emphasis"),
        Tag::Strong => container_node("strong"),
        Tag::Strikethrough => container_node("strikethrough"),
        Tag::Link {
            dest_url, title, ..
        } => {
            let mut node = container_node("link");
            node.attrs.insert("url".to_string(), dest_url.to_string());
            if !title.is_empty() {
                node.attrs.insert("title".to_string(), title.to_string());
            }
            node
        }
        Tag::Image {
            dest_url, title, ..
        } => {
            let mut node = container_node("image");
            node.attrs.insert("url".to_string(), dest_url.to_string());
            if !title.is_empty() {
                node.attrs.insert("title".to_string(), title.to_string());
            }
            node
        }
        Tag::MetadataBlock(kind) => {
            let mut node = container_node("metadata_block");
            node.attrs.insert("kind".to_string(), format!("{kind:?}"));
            node
        }
        Tag::DefinitionList => container_node("definition_list"),
        Tag::DefinitionListTitle => container_node("definition_title"),
        Tag::DefinitionListDefinition => container_node("definition_body"),
        Tag::Superscript => container_node("superscript"),
        Tag::Subscript => container_node("subscript"),
    };

    if node.kind == "image" {
        node.attrs
            .entry("alt".to_string())
            .or_insert_with(String::new);
    }

    node
}

fn parse_markdown_ast(markdown: &str) -> Vec<MarkdownNode> {
    let mut root = Vec::new();
    let mut stack = Vec::new();

    for event in Parser::new_ext(markdown, parser_options()) {
        match event {
            Event::Start(tag) => stack.push(AstFrame::Node(ast_node_from_tag(tag))),
            Event::End(_) => {
                if let Some(frame) = stack.pop() {
                    let AstFrame::Node(mut node) = frame;
                    if node.kind == "heading" {
                        let title = node_text(&node);
                        node.attrs.insert("id".to_string(), slugify(&title));
                    }
                    if node.kind == "image" {
                        let alt = node_text(&node);
                        if !alt.is_empty() {
                            node.attrs.insert("alt".to_string(), alt);
                        }
                    }
                    append_ast_node(&mut stack, &mut root, node);
                }
            }
            Event::Text(text) => {
                append_ast_node(&mut stack, &mut root, text_node("text", text.to_string()));
            }
            Event::Code(text) => {
                append_ast_node(&mut stack, &mut root, text_node("code", text.to_string()));
            }
            Event::Html(text) => {
                append_ast_node(&mut stack, &mut root, text_node("html", text.to_string()));
            }
            Event::SoftBreak => {
                append_ast_node(&mut stack, &mut root, container_node("soft_break"));
            }
            Event::HardBreak => {
                append_ast_node(&mut stack, &mut root, container_node("hard_break"));
            }
            Event::Rule => {
                append_ast_node(&mut stack, &mut root, container_node("rule"));
            }
            Event::FootnoteReference(name) => {
                let mut node = container_node("footnote_reference");
                node.attrs.insert("name".to_string(), name.to_string());
                append_ast_node(&mut stack, &mut root, node);
            }
            Event::TaskListMarker(checked) => {
                let mut node = container_node("task_marker");
                node.attrs
                    .insert("checked".to_string(), checked.to_string());
                append_ast_node(&mut stack, &mut root, node);
            }
            _ => {}
        }
    }

    root
}

fn collect_headings(nodes: &[MarkdownNode], headings: &mut Vec<MarkdownHeading>) {
    for node in nodes {
        if node.kind == "heading" {
            let level = node
                .attrs
                .get("level")
                .and_then(|value| value.parse::<u8>().ok())
                .unwrap_or(1);
            let id = node.attrs.get("id").cloned().unwrap_or_default();
            let title = node_text(node);
            headings.push(MarkdownHeading { level, id, title });
        }
        collect_headings(&node.children, headings);
    }
}

/// Parse a markdown string (optionally with YAML front-matter) into a full document.
pub fn parse_markdown_document(input: &str) -> MarkdownDocument {
    let (meta, normalized_content) = parse_frontmatter_and_content(input);
    let nodes = parse_markdown_ast(&normalized_content);
    let mut headings = Vec::new();
    collect_headings(&nodes, &mut headings);

    MarkdownDocument {
        meta,
        content: MarkdownContent {
            format: "markdown_ast".to_string(),
            nodes,
        },
        headings,
        reading_time_minutes: reading_time_minutes(&normalized_content),
    }
}

/// Parse a markdown string (optionally with YAML front-matter) into HTML.
///
/// Returns both the parsed [`MarkdownMeta`] and the rendered HTML body.
pub fn parse_markdown_to_html(input: &str) -> (MarkdownMeta, String) {
    let (meta, normalized_content) = parse_frontmatter_and_content(input);
    (meta, render_markdown_to_html(&normalized_content))
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
        let input = "---\ntitle: My Post\ntags:\n  - rust\nmindmap:\n  include: true\n  maturity: advanced\n---\n\nContent here.";
        let (meta, html) = parse_markdown_to_html(input);
        assert_eq!(meta.title, "My Post");
        assert_eq!(meta.tags, vec!["rust"]);
        assert!(meta.mindmap.include);
        assert_eq!(meta.mindmap.maturity, Some(TechnologyMaturity::Advanced));
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

    #[test]
    fn converts_media_handle_in_markdown_body() {
        let input = "![Alt](media#sample.png)";
        let (_, html) = parse_markdown_to_html(input);
        assert!(html.contains("/media/sample.png"));
    }

    #[test]
    fn extracts_markdown_headings_for_sidebar() {
        let input = "# Intro\n\n## Details";
        let doc = parse_markdown_document(input);
        assert_eq!(doc.headings.len(), 2);
        assert_eq!(doc.headings[0].id, "intro");
        assert_eq!(doc.headings[1].level, 2);
    }

    #[test]
    fn stores_markdown_as_json_ast_instead_of_source() {
        let input = "# Hello\n\nBody with ![Alt](media#sample.png)";
        let doc = parse_markdown_document(input);
        assert_eq!(doc.content.format, "markdown_ast");
        assert!(!doc.content.nodes.is_empty());
        assert_eq!(doc.content.nodes[0].kind, "heading");
        assert_eq!(
            doc.content.nodes[0].attrs.get("id").map(String::as_str),
            Some("hello")
        );
    }
}
