use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_emphasis(node: MarkdownNode) -> impl IntoView {
    view! { <em class="italic">{render_children(node.children)}</em> }
}
