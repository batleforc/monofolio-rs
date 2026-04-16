use leptos::prelude::*;

use super::MarkdownNode;

pub fn render_image(node: MarkdownNode) -> impl IntoView {
    let src = node
        .attrs
        .get("url")
        .or_else(|| node.attrs.get("src"))
        .cloned()
        .unwrap_or_default();
    let alt = node.attrs.get("alt").cloned().unwrap_or_default();
    let title = node.attrs.get("title").cloned();

    view! {
        <figure class="mb-4">
            <img src=src alt=alt.clone() title=title class="rounded-lg max-w-full h-auto" />
            {if !alt.is_empty() {
                view! { <figcaption class="text-xs text-muted-foreground mt-1">{alt}</figcaption> }
                    .into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </figure>
    }
}
