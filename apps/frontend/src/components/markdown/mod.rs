use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod blockquote;
pub mod breaks;
pub mod code;
pub mod code_block;
pub mod container;
pub mod emphasis;
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

use blockquote::render_blockquote;
use breaks::{render_hard_break, render_soft_break};
use code::render_inline_code;
use code_block::render_code_block;
use container::render_container;
use emphasis::render_emphasis;
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
        _ => render_container(node).into_any(),
    }
}

/// Renders a full `MarkdownContent` inside an `<article>` wrapper.
#[component]
pub fn MarkdownRenderer(content: MarkdownContent) -> impl IntoView {
    let nodes = content.nodes;
    view! {
        <article class="prose prose-sm max-w-none">
            {nodes
                .into_iter()
                .map(|node| view! { <RenderMarkdownNode node=node /> })
                .collect_view()}
        </article>
    }
}

/// Deserializes a `serde_json::Value` and renders it as markdown.
#[component]
pub fn MarkdownFromValue(value: serde_json::Value) -> impl IntoView {
    match serde_json::from_value::<MarkdownContent>(value) {
        Ok(content) => view! { <MarkdownRenderer content=content /> }.into_any(),
        Err(_) => view! { <div class="text-sm text-muted-foreground">"Failed to parse markdown content"</div> }
        .into_any(),
    }
}
