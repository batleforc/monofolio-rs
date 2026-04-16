use leptos::prelude::*;

use super::{render_children, MarkdownNode};

/// Inline footnote reference: renders a superscript link [^name] → <sup><a href="#fn-name">[n]</a></sup>
pub fn render_footnote_reference(node: MarkdownNode) -> impl IntoView {
    let name = node.attrs.get("name").cloned().unwrap_or_default();
    let href = format!("#fn-{name}");
    let back_id = format!("fnref-{name}");
    // Display the label as a number if it parses, otherwise as the name.
    let label = name
        .parse::<u64>()
        .map(|n| n.to_string())
        .unwrap_or_else(|_| name.clone());

    let aria = format!("Footnote {label}");
    view! {
        <sup id=back_id class="text-[0.7em] leading-none">
            <a
                href=href
                class="text-primary hover:text-primary/80 no-underline hover:underline"
                aria-label=aria
            >
                "["
                {label}
                "]"
            </a>
        </sup>
    }
}

/// Block footnote definition: renders <li id="fn-name"> with a back-link ↩ on the same line.
/// The ↩ is injected inside the last paragraph so it stays inline with the text.
pub fn render_footnote_definition(node: MarkdownNode) -> impl IntoView {
    let name = node.attrs.get("name").cloned().unwrap_or_default();
    let id = format!("fn-{name}");
    let back_href = format!("#fnref-{name}");

    // Split children: all but the last paragraph are rendered normally; the last
    // paragraph's children are rendered inline with the ↩ appended.
    let mut children = node.children;
    let last = children.pop();

    let intro = render_children(children);

    let back_link = view! {
        <a
            href=back_href
            class="ml-1 text-primary hover:text-primary/80 no-underline hover:underline text-xs"
            aria-label="Return to content"
        >
            "↩"
        </a>
    };

    let last_view = match last {
        Some(last_node) if last_node.kind == "paragraph" => {
            let para_children = render_children(last_node.children);
            view! { <p class="inline">{para_children} {back_link}</p> }.into_any()
        }
        Some(other) => {
            let rendered = render_children(vec![other]);
            view! { <span class="inline">{rendered} {back_link}</span> }.into_any()
        }
        None => view! { <span class="inline">{back_link}</span> }.into_any(),
    };

    view! {
        <li id=id class="text-sm text-muted-foreground mt-1">
            {intro}
            {last_view}
        </li>
    }
}

/// Renders the footnote section (a <footer> with an <ol>) for all definitions found at top level.
#[component]
pub fn FootnoteSection(nodes: Vec<MarkdownNode>) -> impl IntoView {
    let defs: Vec<MarkdownNode> = nodes
        .into_iter()
        .filter(|n| n.kind == "footnote_definition")
        .collect();

    if defs.is_empty() {
        return view! { <></> }.into_any();
    }

    view! {
        <footer class="mt-8 pt-4 border-t border-border">
            <p class="text-xs uppercase tracking-widest font-mono text-muted-foreground mb-2">
                "Notes"
            </p>
            <ol class="list-decimal list-inside space-y-1">
                {defs.into_iter().map(|node| render_footnote_definition(node)).collect_view()}
            </ol>
        </footer>
    }
    .into_any()
}
