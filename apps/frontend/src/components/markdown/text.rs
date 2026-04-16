use leptos::prelude::*;

use super::MarkdownNode;

pub fn render_text(node: MarkdownNode) -> impl IntoView {
    view! { {node.text} }
}
