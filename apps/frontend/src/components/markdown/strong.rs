use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_strong(node: MarkdownNode) -> impl IntoView {
    view! { <strong class="font-bold">{render_children(node.children)}</strong> }
}
