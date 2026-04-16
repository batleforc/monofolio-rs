use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_list(node: MarkdownNode) -> impl IntoView {
    let ordered = node
        .attrs
        .get("ordered")
        .map(|v| v == "true")
        .unwrap_or(false);

    let children = render_children(node.children);

    if ordered {
        view! { <ol class="mb-4 ml-6 list-decimal space-y-1">{children}</ol> }.into_any()
    } else {
        view! { <ul class="mb-4 ml-6 list-disc space-y-1">{children}</ul> }.into_any()
    }
}

pub fn render_list_item(node: MarkdownNode) -> impl IntoView {
    view! { <li class="text-sm">{render_children(node.children)}</li> }
}
