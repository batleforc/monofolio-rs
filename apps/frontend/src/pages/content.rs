use std::collections::HashSet;

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::Deserialize;
use serde_json::Value;

use crate::components::markdown::toc::{extract_headings, TableOfContents};
use crate::components::markdown::{MarkdownContent, MarkdownFromValue};
use crate::components::ui::{Card, SectionTitle};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct DocSidebarItemData {
    title: String,
    handle: String,
    order: u64,
    kind: String,
    children: Vec<DocSidebarItemData>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct PageData {
    title: String,
    description: String,
    handle: String,
    blog: bool,
    project: bool,
    doc: bool,
    tags: Vec<String>,
    techno: Vec<String>,
    reading_time_minutes: usize,
    content: Value,
}

#[allow(dead_code)]
async fn load_page_data(_handle: String) -> Option<PageData> {
    #[cfg(not(feature = "ssr"))]
    {
        let handle = _handle;
        let endpoint = format!("/api/v1/page/{}", handle.trim_start_matches('/'));
        let resp = gloo_net::http::Request::get(&endpoint).send().await.ok()?;
        let text = resp.text().await.ok()?;
        serde_json::from_str::<PageData>(&text).ok()
    }
    #[cfg(feature = "ssr")]
    {
        let _ = &_handle;
        None
    }
}

#[allow(dead_code)]
async fn load_docs_nav() -> Vec<DocSidebarItemData> {
    #[cfg(not(feature = "ssr"))]
    {
        let resp = match gloo_net::http::Request::get("/api/v1/nav/doc").send().await {
            Ok(resp) => resp,
            Err(_) => return vec![],
        };
        let text = match resp.text().await {
            Ok(text) => text,
            Err(_) => return vec![],
        };
        serde_json::from_str::<Vec<DocSidebarItemData>>(&text).unwrap_or_default()
    }
    #[cfg(feature = "ssr")]
    {
        vec![]
    }
}

fn render_doc_sidebar_tree(
    items: Vec<DocSidebarItemData>,
    active_handle: String,
    depth: usize,
    expanded_folders: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let mut sorted = items;
    sorted.sort_by(|a, b| a.order.cmp(&b.order));

    view! {
        <ul class="flex flex-col gap-1.5">
            {sorted
                .into_iter()
                .map(|item| {
                    let indent = match depth {
                        0 => "",
                        1 => "ms-2",
                        2 => "ms-4",
                        3 => "ms-6",
                        _ => "ms-8",
                    };
                    let href = format!("/{}", item.handle.trim_start_matches('/'));
                    let has_children = !item.children.is_empty();
                    let children = item.children.clone();
                    let is_folder = item.kind == "folder";
                    let is_folder_with_index = item.kind == "folder_with_index";
                    let is_folder_like = is_folder || is_folder_with_index;
                    let is_active = item.handle == active_handle;
                    let item_title = item.title.clone();
                    let handle = item.handle.clone();
                    let handle_for_toggle = handle.clone();
                    let handle_for_open = handle.clone();
                    let active_handle_next = active_handle.clone();

                    view! {
                        <li class=indent>
                            <div class="flex items-center gap-2">
                                {if is_folder_like && has_children {
                                    view! {
                                        <button
                                            type="button"
                                            class="inline-flex h-5 w-5 items-center justify-center rounded border border-border text-muted-foreground hover:text-foreground hover:border-foreground/30 transition-colors"
                                            on:click=move |_| {
                                                if depth == 0 {
                                                    return;
                                                }
                                                expanded_folders
                                                    .update(|open| {
                                                        if open.contains(&handle_for_toggle) {
                                                            open.remove(&handle_for_toggle);
                                                        } else {
                                                            open.insert(handle_for_toggle.clone());
                                                        }
                                                    });
                                            }
                                        >
                                            <span aria-hidden="true">
                                                {move || {
                                                    if depth == 0
                                                        || expanded_folders
                                                            .with(|open| open.contains(&handle_for_open))
                                                    {
                                                        "▾"
                                                    } else {
                                                        "▸"
                                                    }
                                                }}
                                            </span>
                                        </button>
                                    }
                                        .into_any()
                                } else {
                                    view! { <span class="inline-flex h-5 w-5"></span> }.into_any()
                                }}
                                {if is_folder_with_index {
                                    view! {
                                        <a
                                            href=href
                                            class=if is_active {
                                                "text-sm font-semibold text-primary underline"
                                            } else {
                                                "text-sm font-semibold text-foreground hover:text-primary transition-colors"
                                            }
                                        >
                                            {item_title}
                                        </a>
                                    }
                                        .into_any()
                                } else if is_folder {
                                    view! {
                                        <span class="text-xs uppercase tracking-wide text-muted-foreground">
                                            {item_title}
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <a
                                            href=href
                                            class=if is_active {
                                                "text-sm font-semibold text-primary underline"
                                            } else {
                                                "text-sm font-medium text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                            }
                                        >
                                            {item_title}
                                        </a>
                                    }
                                        .into_any()
                                }}
                            </div>

                            {move || {
                                let is_open = depth == 0
                                    || expanded_folders.with(|open| open.contains(&handle));
                                if has_children && is_open {
                                    view! {
                                        <div class="mt-2">
                                            {render_doc_sidebar_tree(
                                                children.clone(),
                                                active_handle_next.clone(),
                                                depth + 1,
                                                expanded_folders,
                                            )}
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }
                            }}
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[allow(dead_code)]
fn collect_active_path_folders(
    items: &[DocSidebarItemData],
    active_handle: &str,
    out: &mut HashSet<String>,
) -> bool {
    for item in items {
        let self_matches = item.handle == active_handle;
        let child_matches = collect_active_path_folders(&item.children, active_handle, out);
        if self_matches || child_matches {
            if !item.children.is_empty() {
                out.insert(item.handle.clone());
            }
            return true;
        }
    }
    false
}

#[component]
pub fn ContentHandlePage() -> impl IntoView {
    #[allow(unused_variables)]
    let location = use_location();
    let page_data: RwSignal<Option<PageData>> = RwSignal::new(None);
    let loading = RwSignal::new(true);
    let fetch_error = RwSignal::new(false);
    let docs_nav: RwSignal<Vec<DocSidebarItemData>> = RwSignal::new(vec![]);
    let expanded_folders: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            let handle = location.pathname.get().trim_start_matches('/').to_string();

            // This page handles only content routes. During SPA transitions,
            // the effect may observe intermediate pathnames (e.g. /projects).
            // Guard to avoid bogus fetches like /api/v1/page/projects.
            let is_supported_handle = handle.starts_with("docs/") || handle.starts_with("blogs/");
            if !is_supported_handle {
                return;
            }

            loading.set(true);
            fetch_error.set(false);
            page_data.set(None);

            if handle.is_empty() {
                fetch_error.set(true);
                loading.set(false);
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                match load_page_data(handle).await {
                    Some(data) => {
                        page_data.set(Some(data));
                        fetch_error.set(false);
                    }
                    None => {
                        fetch_error.set(true);
                    }
                }
                loading.set(false);
            });
        });

        Effect::new(move |_| {
            let handle = location.pathname.get().trim_start_matches('/').to_string();
            if !handle.starts_with("docs/") {
                docs_nav.set(vec![]);
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                docs_nav.set(load_docs_nav().await);
            });
        });

        Effect::new(move |_| {
            let handle = location.pathname.get().trim_start_matches('/').to_string();
            let nav_data = docs_nav.get();
            if !handle.starts_with("docs/") && handle != "docs" {
                expanded_folders.set(HashSet::new());
                return;
            }

            let mut open = HashSet::new();
            collect_active_path_folders(&nav_data, &handle, &mut open);
            expanded_folders.set(open);
        });
    }

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <div class="max-w-7xl mx-auto px-5 py-16">
                <Show when=move || loading.get()>
                    <Card class="p-5">
                        <p class="text-sm text-muted-foreground">"Loading page..."</p>
                    </Card>
                </Show>

                <Show when=move || fetch_error.get() && !loading.get()>
                    <Card class="p-5">
                        <p class="text-sm text-muted-foreground">"Unable to load this page."</p>
                        <a
                            href="/projects"
                            class="inline-flex mt-3 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                        >
                            "Back to projects"
                        </a>
                    </Card>
                </Show>

                <Show when=move || {
                    !loading.get() && !fetch_error.get()
                }>
                    {move || {
                        page_data
                            .get()
                            .map(|page| {
                                let PageData {
                                    title,
                                    description,
                                    handle,
                                    blog,
                                    project,
                                    doc,
                                    tags,
                                    techno,
                                    reading_time_minutes,
                                    content,
                                } = page;
                                let kind_label = if blog {
                                    "blog"
                                } else if project {
                                    "project"
                                } else if doc {
                                    "doc"
                                } else {
                                    "content"
                                };
                                let is_blog = blog;
                                let is_doc = doc;
                                let back_href = if is_blog { "/blog" } else { "/projects" };
                                let back_label = if is_blog {
                                    "Back to blog"
                                } else {
                                    "Back to projects"
                                };

                                view! {
                                    <SectionTitle>{title.clone()}</SectionTitle>

                                    {if is_doc {
                                        let handle_for_sidebar = handle.clone();
                                        let handle_for_mobile_sidebar = handle.clone();
                                        let toc_entries = serde_json::from_value::<
                                            MarkdownContent,
                                        >(content.clone())
                                            .map(|markdown| extract_headings(&markdown.nodes))
                                            .unwrap_or_default();
                                        let has_toc_entries = !toc_entries.is_empty();
                                        view! {
                                            <div class="lg:hidden sticky top-14 z-40 mb-4">
                                                <Card class="p-2 border-border/80 bg-background/95 backdrop-blur-md">
                                                    <div class="grid grid-cols-2 gap-2">
                                                        <details>
                                                            <summary class="list-none cursor-pointer select-none rounded border border-border px-3 py-2 text-xs font-mono uppercase tracking-widest text-muted-foreground hover:text-foreground hover:border-primary/40 transition-colors">
                                                                "Navigation"
                                                            </summary>
                                                            <div class="mt-2 rounded border border-border p-3 max-h-[55svh] overflow-auto">
                                                                {move || {
                                                                    render_doc_sidebar_tree(
                                                                            docs_nav.get(),
                                                                            handle_for_mobile_sidebar.clone(),
                                                                            0,
                                                                            expanded_folders,
                                                                        )
                                                                        .into_any()
                                                                }}
                                                            </div>
                                                        </details>

                                                        {if has_toc_entries {
                                                            view! {
                                                                <details>
                                                                    <summary class="list-none cursor-pointer select-none rounded border border-border px-3 py-2 text-xs font-mono uppercase tracking-widest text-muted-foreground hover:text-foreground hover:border-primary/40 transition-colors">
                                                                        "Sommaire"
                                                                    </summary>
                                                                    <div class="mt-2 rounded border border-border p-3 max-h-[55svh] overflow-auto">
                                                                        <TableOfContents entries=toc_entries.clone() />
                                                                    </div>
                                                                </details>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! {
                                                                <div class="rounded border border-dashed border-border/70 px-3 py-2 text-xs text-muted-foreground">
                                                                    "Pas de sommaire"
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }}
                                                    </div>
                                                </Card>
                                            </div>

                                            <div class="grid grid-cols-1 lg:grid-cols-[18rem_1fr] gap-5">
                                                <aside class="lg:sticky lg:top-16 self-start">
                                                    <Card class="p-4 max-h-[calc(100svh-6rem)] overflow-auto">
                                                        <p class="text-xs uppercase tracking-widest font-mono text-muted-foreground mb-3">
                                                            "Docs navigation"
                                                        </p>
                                                        {move || {
                                                            render_doc_sidebar_tree(
                                                                    docs_nav.get(),
                                                                    handle_for_sidebar.clone(),
                                                                    0,
                                                                    expanded_folders,
                                                                )
                                                                .into_any()
                                                        }}
                                                    </Card>
                                                </aside>

                                                <Card class="p-5">
                                                    <p class="text-sm text-muted-foreground mb-4">
                                                        {description.clone()}
                                                    </p>
                                                    <p class="text-xs text-muted-foreground font-mono mb-3">
                                                        {handle.clone()}
                                                    </p>
                                                    <p class="text-xs text-muted-foreground uppercase tracking-widest font-mono mb-3">
                                                        {kind_label}
                                                    </p>

                                                    <div class="flex flex-wrap gap-1.5 mb-3">
                                                        {techno
                                                            .clone()
                                                            .into_iter()
                                                            .map(|item| {
                                                                view! {
                                                                    <span class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground">
                                                                        {item}
                                                                    </span>
                                                                }
                                                            })
                                                            .collect_view()}
                                                        {tags
                                                            .clone()
                                                            .into_iter()
                                                            .map(|item| {
                                                                view! {
                                                                    <span class="inline-flex items-center px-2 py-0.5 rounded border border-primary/30 text-[0.7rem] text-primary">
                                                                        {format!("#{}", item)}
                                                                    </span>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>

                                                    <p class="text-xs text-muted-foreground">
                                                        {format!("Reading time: {} min", reading_time_minutes)}
                                                    </p>

                                                    <div class="mt-5 border-t border-border pt-4">
                                                        <MarkdownFromValue value=content.clone() />
                                                    </div>

                                                    <a
                                                        href=back_href
                                                        class="inline-flex mt-4 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                    >
                                                        {back_label}
                                                    </a>
                                                </Card>
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <Card class="p-5">
                                                <p class="text-sm text-muted-foreground mb-4">
                                                    {description}
                                                </p>
                                                <p class="text-xs text-muted-foreground font-mono mb-3">
                                                    {handle}
                                                </p>
                                                <p class="text-xs text-muted-foreground uppercase tracking-widest font-mono mb-3">
                                                    {kind_label}
                                                </p>

                                                <div class="flex flex-wrap gap-1.5 mb-3">
                                                    {techno
                                                        .into_iter()
                                                        .map(|item| {
                                                            view! {
                                                                <span class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground">
                                                                    {item}
                                                                </span>
                                                            }
                                                        })
                                                        .collect_view()}
                                                    {tags
                                                        .into_iter()
                                                        .map(|item| {
                                                            view! {
                                                                <span class="inline-flex items-center px-2 py-0.5 rounded border border-primary/30 text-[0.7rem] text-primary">
                                                                    {format!("#{}", item)}
                                                                </span>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>

                                                <p class="text-xs text-muted-foreground">
                                                    {format!("Reading time: {} min", reading_time_minutes)}
                                                </p>

                                                {if is_blog {
                                                    view! {
                                                        <div class="mt-5 border-t border-border pt-4">
                                                            <MarkdownFromValue value=content />
                                                        </div>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! { <></> }.into_any()
                                                }}

                                                <a
                                                    href=back_href
                                                    class="inline-flex mt-4 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                >
                                                    {back_label}
                                                </a>
                                            </Card>
                                        }
                                            .into_any()
                                    }}
                                }
                            })
                    }}
                </Show>
            </div>
        </section>
    }
}
