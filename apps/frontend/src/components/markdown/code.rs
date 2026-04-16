use leptos::prelude::*;

use super::MarkdownNode;

/// Render inline code (`backtick` code).
pub fn render_inline_code(node: MarkdownNode) -> impl IntoView {
    view! {
        <code class="bg-muted px-1.5 py-0.5 rounded text-sm font-mono text-primary">
            {node.text}
        </code>
    }
}
