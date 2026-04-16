use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_link(node: MarkdownNode) -> impl IntoView {
    let href = node
        .attrs
        .get("url")
        .or_else(|| node.attrs.get("href"))
        .cloned()
        .unwrap_or_default();
    let title = node.attrs.get("title").cloned();

    view! {
        <a
            href=href
            title=title
            class="text-primary hover:text-primary/80 underline underline-offset-2"
        >
            {render_children(node.children)}
        </a>
    }
}
