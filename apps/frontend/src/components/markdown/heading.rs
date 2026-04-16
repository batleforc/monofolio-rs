use leptos::prelude::*;

use super::{extract_text, heading_anchor_id, render_children, slugify, MarkdownNode};

pub fn render_heading(node: MarkdownNode) -> impl IntoView {
    let level = node
        .attrs
        .get("level")
        .and_then(|l| l.parse::<u8>().ok())
        .unwrap_or(1);

    let base_id = node
        .attrs
        .get("id")
        .cloned()
        .unwrap_or_else(|| slugify(&extract_text(&node.children)));
    let id = heading_anchor_id(&base_id);

    let href = format!("#{id}");
    let children = render_children(node.children);

    let anchor_class = "anchor-link opacity-0 group-hover:opacity-100 transition-opacity ml-2 text-muted-foreground hover:text-foreground no-underline";

    match level {
        1 => view! {
            <h1 id=id.clone() class="group text-3xl font-bold mt-6 mb-3">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h1>
        }
        .into_any(),
        2 => view! {
            <h2 id=id.clone() class="group text-2xl font-bold mt-5 mb-3">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h2>
        }
        .into_any(),
        3 => view! {
            <h3 id=id.clone() class="group text-xl font-bold mt-4 mb-2">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h3>
        }
        .into_any(),
        4 => view! {
            <h4 id=id.clone() class="group text-lg font-bold mt-4 mb-2">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h4>
        }
        .into_any(),
        5 => view! {
            <h5 id=id.clone() class="group text-base font-bold mt-3 mb-2">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h5>
        }
        .into_any(),
        _ => view! {
            <h6 id=id.clone() class="group text-sm font-bold mt-3 mb-2">
                {children}
                <a href=href class=anchor_class aria-hidden="true">
                    "#"
                </a>
            </h6>
        }
        .into_any(),
    }
}
