use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_blockquote(node: MarkdownNode) -> impl IntoView {
    view! {
        <blockquote class="mb-4 pl-4 border-l-4 border-primary/30 italic text-muted-foreground">
            {render_children(node.children)}
        </blockquote>
    }
}
