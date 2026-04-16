use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_paragraph(node: MarkdownNode) -> impl IntoView {
    view! { <p class="mb-4 leading-relaxed">{render_children(node.children)}</p> }
}
