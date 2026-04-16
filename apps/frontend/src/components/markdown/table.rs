use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_table(node: MarkdownNode) -> impl IntoView {
    view! {
        <div class="mb-4 overflow-x-auto border border-border rounded">
            <table class="w-full text-sm">{render_children(node.children)}</table>
        </div>
    }
}
