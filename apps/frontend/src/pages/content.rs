use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::Deserialize;
use serde_json::Value;

use crate::components::ui::{Card, SectionInner, SectionTitle};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct DocSidebarItemData {
    title: String,
    handle: String,
    order: usize,
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
                    let is_active = item.handle == active_handle;
                    let item_title = item.title.clone();
                    let kind = item.kind.clone();
                    let active_handle_next = active_handle.clone();

                    view! {
                        <li class=indent>
                            <div class="flex items-center gap-2">
                                {if is_folder {
                                    view! {
                                        <span class="text-sm font-medium text-foreground/80">{item_title.clone()}</span>
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

                                <span class="text-[0.65rem] uppercase tracking-widest font-mono text-muted-foreground">
                                    {kind}
                                </span>
                            </div>

                            {if has_children {
                                view! {
                                    <div class="mt-1.5">
                                        {render_doc_sidebar_tree(children, active_handle_next, depth + 1)}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
pub fn ContentHandlePage() -> impl IntoView {
    #[allow(unused_variables)]
    let location = use_location();
    let page_data: RwSignal<Option<PageData>> = RwSignal::new(None);
    let loading = RwSignal::new(true);
    let fetch_error = RwSignal::new(false);
    let docs_nav: RwSignal<Vec<DocSidebarItemData>> = RwSignal::new(vec![]);

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
    }

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
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

                <Show when=move || !loading.get() && !fetch_error.get()>
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

                                let content_pretty = serde_json::to_string_pretty(&content)
                                    .unwrap_or_else(|_| String::from("{}"));
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
                                        view! {
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
                                                            )
                                                        }}
                                                    </Card>
                                                </aside>

                                                <Card class="p-5">
                                                    <p class="text-sm text-muted-foreground mb-4">{description.clone()}</p>
                                                    <p class="text-xs text-muted-foreground font-mono mb-3">{handle.clone()}</p>
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
                                                <p class="text-sm text-muted-foreground mb-4">{description}</p>
                                                <p class="text-xs text-muted-foreground font-mono mb-3">{handle}</p>
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
                                                            <p class="text-sm font-semibold mb-2">"Blog post content"</p>
                                                            <pre class="text-xs text-muted-foreground overflow-x-auto whitespace-pre-wrap bg-background/70 border border-border rounded p-3">
                                                                {content_pretty}
                                                            </pre>
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
            </SectionInner>
        </section>
    }
}
