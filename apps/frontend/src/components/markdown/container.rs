use leptos::prelude::*;

use super::{render_children, MarkdownNode};

/// Fallback renderer for unknown node types.
pub fn render_container(node: MarkdownNode) -> impl IntoView {
    view! { <div>{render_children(node.children)}</div> }
}
