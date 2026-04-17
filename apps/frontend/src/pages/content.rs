use std::collections::HashSet;

use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "ssr")]
use content::{ContentDatabase, ContentEntry, SidebarItem};

use crate::components::markdown::toc::{extract_headings, TableOfContents};
use crate::components::markdown::{MarkdownContent, MarkdownFromValue};
use crate::components::ui::{Card, SectionTitle};
use crate::seo::{canonical_url, DEFAULT_OG_IMAGE};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
struct DocSidebarItemData {
    title: String,
    handle: String,
    order: u64,
    kind: String,
    children: Vec<DocSidebarItemData>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
struct PageData {
    title: String,
    description: String,
    handle: String,
    blog: bool,
    project: bool,
    doc: bool,
    tags: Vec<String>,
    techno: Vec<String>,
    image: String,
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

#[cfg(not(feature = "ssr"))]
async fn load_page_data_send_safe(handle: String) -> Option<PageData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Option<PageData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_page_data(handle).await);
    });
    rx.await.ok().flatten()
}

#[cfg(not(feature = "ssr"))]
async fn load_docs_nav_send_safe() -> Vec<DocSidebarItemData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Vec<DocSidebarItemData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_docs_nav().await);
    });
    rx.await.unwrap_or_default()
}

#[cfg(feature = "ssr")]
impl From<&ContentEntry> for PageData {
    fn from(entry: &ContentEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            blog: entry.kind.blog,
            project: entry.kind.project,
            doc: entry.kind.doc,
            tags: entry.tags.clone(),
            techno: entry.techno.clone(),
            image: entry.image.clone(),
            reading_time_minutes: entry.reading_time_minutes,
            content: serde_json::to_value(&entry.content).unwrap_or(serde_json::Value::Null),
        }
    }
}

#[cfg(feature = "ssr")]
fn map_sidebar_item(item: &SidebarItem, known_doc_handles: &HashSet<String>) -> DocSidebarItemData {
    let kind = if item.children.is_empty() {
        "markdown".to_string()
    } else if known_doc_handles.contains(&item.handle) {
        "folder_with_index".to_string()
    } else {
        "folder".to_string()
    };
    DocSidebarItemData {
        title: item.title.clone(),
        handle: item.handle.clone(),
        order: item.order as u64,
        kind,
        children: item
            .children
            .iter()
            .map(|child| map_sidebar_item(child, known_doc_handles))
            .collect(),
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
    let location = use_location();
    #[cfg(feature = "ssr")]
    let content_db = use_context::<ContentDatabase>();

    let page_data = Resource::new(
        move || location.pathname.get(),
        {
            #[cfg(feature = "ssr")]
            let content_db = content_db.clone();
            move |pathname: String| {
                #[cfg(feature = "ssr")]
                let content_db = content_db.clone();
                async move {
                    let handle = pathname.trim_start_matches('/').to_string();
                    #[cfg(not(feature = "ssr"))]
                    {
                        load_page_data_send_safe(handle).await
                    }
                    #[cfg(feature = "ssr")]
                    {
                        content_db.and_then(|db| {
                            db.entries
                                .iter()
                                .find(|entry| entry.handle == handle)
                                .map(PageData::from)
                        })
                    }
                }
            }
        },
    );
    let docs_nav = Resource::new(
        move || location.pathname.get(),
        {
            #[cfg(feature = "ssr")]
            let content_db = content_db.clone();
            move |pathname: String| {
                #[cfg(feature = "ssr")]
                let content_db = content_db.clone();
                async move {
                    let handle = pathname.trim_start_matches('/').to_string();
                    if !handle.starts_with("docs/") && handle != "docs" {
                        return vec![];
                    }
                    #[cfg(not(feature = "ssr"))]
                    {
                        load_docs_nav_send_safe().await
                    }
                    #[cfg(feature = "ssr")]
                    {
                        content_db
                            .map(|db| {
                                let known_doc_handles: HashSet<String> = db
                                    .entries
                                    .iter()
                                    .filter(|entry| entry.kind.doc)
                                    .map(|entry| entry.handle.clone())
                                    .collect();
                                db.sidebar
                                    .iter()
                                    .map(|item| map_sidebar_item(item, &known_doc_handles))
                                    .collect()
                            })
                            .unwrap_or_default()
                    }
                }
            }
        },
    );
    let expanded_folders: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());
    Effect::new(move |_| {
        let handle = location.pathname.get().trim_start_matches('/').to_string();
        let nav_data = docs_nav.get().unwrap_or_default();
        if !handle.starts_with("docs/") && handle != "docs" {
            expanded_folders.set(HashSet::new());
            return;
        }

        let mut open = HashSet::new();
        collect_active_path_folders(&nav_data, &handle, &mut open);
        expanded_folders.set(open);
    });

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <div class="max-w-7xl mx-auto px-5 py-16">
                <Show when=move || page_data.get().is_none()>
                    <Card class="p-5">
                        <p class="text-sm text-muted-foreground">"Loading page..."</p>
                    </Card>
                </Show>

                <Show when=move || page_data.get().is_some() && page_data.get().flatten().is_none()>
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
                    page_data.get().flatten().is_some()
                }>
                    {move || {
                        page_data
                            .get()
                            .flatten()
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
                                    image,
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
                                let page_title = format!("{title} | Maxime Leriche");
                                let page_description = if description.trim().is_empty() {
                                    "Page de contenu de Maxime Leriche".to_string()
                                } else {
                                    description.clone()
                                };
                                let canonical = canonical_url(&handle);
                                let image_url = if image.trim().is_empty() {
                                    DEFAULT_OG_IMAGE.to_string()
                                } else if image.starts_with("https://") {
                                    image
                                } else if image.starts_with("http://") {
                                    format!("https://{}", image.trim_start_matches("http://"))
                                } else if let Some(file_name) = image.strip_prefix("media#") {
                                    canonical_url(&format!("/media/{file_name}"))
                                } else {
                                    canonical_url(&format!("/media/{}", image.trim_start_matches('/')))
                                };

                                view! {
                                    <Title text=page_title.clone() />
                                    <Meta name="description" content=page_description.clone() />
                                    <Link rel="canonical" href=canonical.clone() />
                                    <Meta property="og:type" content=if is_blog { "article" } else { "website" } />
                                    <Meta property="og:title" content=page_title.clone() />
                                    <Meta property="og:description" content=page_description.clone() />
                                    <Meta property="og:url" content=canonical.clone() />
                                    <Meta property="og:image" content=image_url.clone() />
                                    <Meta name="twitter:card" content="summary_large_image" />
                                    <Meta name="twitter:title" content=page_title.clone() />
                                    <Meta name="twitter:description" content=page_description.clone() />
                                    <Meta name="twitter:image" content=image_url.clone() />
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
                                                                            docs_nav
                                                                                .get()
                                                                                .unwrap_or_default(),
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
                                                                    docs_nav
                                                                        .get()
                                                                        .unwrap_or_default(),
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
