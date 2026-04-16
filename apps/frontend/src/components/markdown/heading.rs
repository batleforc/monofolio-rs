use leptos::prelude::*;

use super::{render_children, MarkdownNode};

pub fn render_heading(node: MarkdownNode) -> impl IntoView {
    let level = node
        .attrs
        .get("level")
        .and_then(|l| l.parse::<u8>().ok())
        .unwrap_or(1);

    let id = node.attrs.get("id").cloned();
    let children = render_children(node.children);

    match level {
        1 => view! {
            <h1 id=id class="text-3xl font-bold mt-6 mb-3">
                {children}
            </h1>
        }
        .into_any(),
        2 => view! {
            <h2 id=id class="text-2xl font-bold mt-5 mb-3">
                {children}
            </h2>
        }
        .into_any(),
        3 => view! {
            <h3 id=id class="text-xl font-bold mt-4 mb-2">
                {children}
            </h3>
        }
        .into_any(),
        4 => view! {
            <h4 id=id class="text-lg font-bold mt-4 mb-2">
                {children}
            </h4>
        }
        .into_any(),
        5 => view! {
            <h5 id=id class="text-base font-bold mt-3 mb-2">
                {children}
            </h5>
        }
        .into_any(),
        _ => view! {
            <h6 id=id class="text-sm font-bold mt-3 mb-2">
                {children}
            </h6>
        }
        .into_any(),
    }
}
