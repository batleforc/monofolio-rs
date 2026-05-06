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
    let is_mermaid = src.contains("/public/mermaid/") || src.contains("/mermaid/");

    let figure_class = if is_mermaid {
        "mb-6 rounded-lg border border-border/70 bg-card/60 p-3 overflow-x-auto"
    } else {
        "mb-4"
    };

    let image_class = if is_mermaid {
        "block h-auto max-w-none rounded"
    } else {
        "rounded-lg max-w-full h-auto"
    };

    view! {
        <figure class=figure_class>
            <img src=src alt=alt.clone() title=title class=image_class />
            {if !alt.is_empty() {
                view! { <figcaption class="text-xs text-muted-foreground mt-1">{alt}</figcaption> }
                    .into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </figure>
    }
}
