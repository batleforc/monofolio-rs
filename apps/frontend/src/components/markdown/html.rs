use leptos::prelude::*;

use super::MarkdownNode;

/// Renders raw HTML nodes.
/// Currently shown as a warning to prevent XSS.
/// TODO: Integrate `ammonia` crate for safe HTML sanitization.
pub fn render_html(node: MarkdownNode) -> impl IntoView {
    view! {
        <div class="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded text-xs text-yellow-800">
            {format!("[HTML content - requires sanitization]: {}", node.text)}
        </div>
    }
}
