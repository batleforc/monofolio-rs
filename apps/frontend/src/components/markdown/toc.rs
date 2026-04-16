use leptos::prelude::*;

use super::{extract_text, heading_anchor_id, slugify, MarkdownNode};

/// A single entry in the table of contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// Walk the top-level nodes and collect all heading entries.
pub fn extract_headings(nodes: &[MarkdownNode]) -> Vec<TocEntry> {
    nodes
        .iter()
        .filter(|n| n.kind == "heading")
        .map(|n| {
            let level = n
                .attrs
                .get("level")
                .and_then(|l| l.parse::<u8>().ok())
                .unwrap_or(1);
            let text = extract_text(&n.children);
            let base_id = n.attrs.get("id").cloned().unwrap_or_else(|| slugify(&text));
            let id = heading_anchor_id(&base_id);
            TocEntry { level, text, id }
        })
        .collect()
}

/// Renders a sticky table of contents from a list of `TocEntry`.
#[component]
pub fn TableOfContents(entries: Vec<TocEntry>) -> impl IntoView {
    if entries.is_empty() {
        return view! { <></> }.into_any();
    }

    let min_level = entries.iter().map(|e| e.level).min().unwrap_or(1);

    view! {
        <nav class="text-sm">
            <p class="font-semibold mb-2 text-foreground">"Table des matières"</p>
            <ul class="space-y-1">
                {entries
                    .into_iter()
                    .map(|entry| {
                        let indent = (entry.level - min_level) as usize;
                        let padding = match indent {
                            0 => "",
                            1 => "pl-3",
                            2 => "pl-6",
                            3 => "pl-9",
                            _ => "pl-12",
                        };
                        let href = format!("#{}", entry.id);
                        view! {
                            <li class=padding>
                                <a
                                    href=href
                                    class="text-muted-foreground hover:text-foreground transition-colors line-clamp-2"
                                >
                                    {entry.text}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
    .into_any()
}
