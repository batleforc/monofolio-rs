use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[cfg(not(feature = "ssr"))]
use js_sys;

pub mod blockquote;
pub mod breaks;
pub mod code;
pub mod code_block;
pub mod container;
pub mod emphasis;
pub mod footnote;
pub mod heading;
pub mod html;
pub mod image;
pub mod link;
pub mod list;
pub mod paragraph;
pub mod rule;
pub mod strong;
pub mod table;
pub mod text;
pub mod toc;

use blockquote::render_blockquote;
use breaks::{render_hard_break, render_soft_break};
use code::render_inline_code;
use code_block::render_code_block;
use container::render_container;
use emphasis::render_emphasis;
use footnote::{render_footnote_reference, FootnoteSection};
use heading::render_heading;
use html::render_html;
use image::render_image;
use link::render_link;
use list::{render_list, render_list_item};
use paragraph::render_paragraph;
use rule::render_rule;
use strong::render_strong;
use table::render_table;
use text::render_text;
use toc::{extract_headings, TableOfContents};

use crate::components::ui::ProseContent;

/// Represents a markdown AST node.
/// Corresponds to the backend's `MarkdownNode` structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkdownNode {
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MarkdownNode>,
}

/// Markdown content container with format and nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkdownContent {
    pub format: String,
    pub nodes: Vec<MarkdownNode>,
}

/// Recursively extract plain text from a node tree.
pub fn extract_text(nodes: &[MarkdownNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        if !node.text.is_empty() {
            out.push_str(&node.text);
        }
        out.push_str(&extract_text(&node.children));
    }
    out
}

/// Turn a heading's plain text into a URL-friendly slug.
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Ensure heading anchors target heading elements with a dedicated id namespace.
pub fn heading_anchor_id(base_id: &str) -> String {
    if base_id.starts_with("heading-") {
        base_id.to_string()
    } else {
        format!("heading-{base_id}")
    }
}

/// Render a collection of child nodes.
pub fn render_children(children: Vec<MarkdownNode>) -> impl IntoView {
    children
        .into_iter()
        .map(|child| {
            view! { <RenderMarkdownNode node=child /> }
        })
        .collect_view()
}

/// Dispatch a single markdown node to its specialized renderer.
#[component]
pub fn RenderMarkdownNode(node: MarkdownNode) -> impl IntoView {
    match node.kind.as_str() {
        "heading" => render_heading(node).into_any(),
        "paragraph" => render_paragraph(node).into_any(),
        "code_block" => render_code_block(node).into_any(),
        "blockquote" => render_blockquote(node).into_any(),
        "list" => render_list(node).into_any(),
        "list_item" => render_list_item(node).into_any(),
        "table" => render_table(node).into_any(),
        "rule" => render_rule().into_any(),
        "html" => render_html(node).into_any(),
        "text" => render_text(node).into_any(),
        "code" => render_inline_code(node).into_any(),
        "emphasis" => render_emphasis(node).into_any(),
        "strong" => render_strong(node).into_any(),
        "link" => render_link(node).into_any(),
        "image" => render_image(node).into_any(),
        "hard_break" => render_hard_break().into_any(),
        "soft_break" => render_soft_break().into_any(),
        "footnote_reference" => render_footnote_reference(node).into_any(),
        "footnote_definition" => view! { <></> }.into_any(),
        _ => render_container(node).into_any(),
    }
}

#[component]
fn MarkdownHashScroll() -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(|_| {
            let _ = js_sys::eval(
                r#"(function() {
                    var hash = window.location.hash;
                    if (!hash) return;
                    var rawId = decodeURIComponent(hash.slice(1));
                    var el = document.getElementById(rawId);
                    if (!el) {
                        el = document.getElementById('heading-' + rawId);
                    }
                    if (!el) return;
                    el.scrollIntoView({ block: 'nearest', behavior: "smooth" });
                })()"#,
            );
        });
    }

    view! { <></> }
}

/// Renders a full `MarkdownContent` inside a shared prose wrapper.
#[component]
pub fn MarkdownRenderer(content: MarkdownContent) -> impl IntoView {
    let nodes = content.nodes;
    let footnote_nodes = nodes.clone();
    let body_nodes: Vec<MarkdownNode> = nodes
        .into_iter()
        .filter(|n| n.kind != "footnote_definition")
        .collect();

    view! {
        <ProseContent>
            <MarkdownHashScroll />
            {body_nodes
                .into_iter()
                .map(|node| view! { <RenderMarkdownNode node=node /> })
                .collect_view()}
            <FootnoteSection nodes=footnote_nodes />
        </ProseContent>
    }
}

/// Deserializes a `serde_json::Value` and renders it as markdown.
#[component]
pub fn MarkdownFromValue(value: serde_json::Value) -> impl IntoView {
    match serde_json::from_value::<MarkdownContent>(value) {
        Ok(content) => view! { <MarkdownWithToc content=content /> }.into_any(),
        Err(_) => view! { <div class="text-sm text-muted-foreground">"Failed to parse markdown content"</div> }
        .into_any(),
    }
}

/// Renders markdown with an optional sticky table of contents on the right.
#[component]
pub fn MarkdownWithToc(content: MarkdownContent) -> impl IntoView {
    let entries = extract_headings(&content.nodes);
    let has_toc = !entries.is_empty();

    if has_toc {
        view! {
            <div class="flex gap-4">
                <div class="flex-1 min-w-0">
                    <MarkdownRenderer content=content />
                </div>
                <aside class="hidden lg:block w-48 shrink-0">
                    <div class="sticky top-6 border border-border rounded-lg p-4 bg-background/70 backdrop-blur-sm">
                        <TableOfContents entries=entries />
                    </div>
                </aside>
            </div>
        }
        .into_any()
    } else {
        view! { <MarkdownRenderer content=content /> }.into_any()
    }
}
