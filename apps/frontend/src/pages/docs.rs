use leptos::prelude::*;
use serde::Deserialize;

use crate::components::ui::{Card, SectionInner, SectionTitle};
use crate::i18n::{use_language, Language};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct DocSidebarItemData {
    title: String,
    handle: String,
    order: usize,
    kind: String,
    children: Vec<DocSidebarItemData>,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
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

#[allow(dead_code)]
fn render_doc_tree(items: Vec<DocSidebarItemData>, depth: usize) -> impl IntoView {
    let mut sorted = items;
    sorted.sort_by(|a, b| a.order.cmp(&b.order));

    view! {
        <ul class="flex flex-col gap-2">
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
                    view! {
                        <li class=indent>
                            <div class="flex items-center gap-2">
                                {if is_folder {
                                    view! {
                                        <span class="text-sm font-medium text-foreground/80">{item.title.clone()}</span>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <a
                                            href=href
                                            class="text-sm font-medium text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                        >
                                            {item.title.clone()}
                                        </a>
                                    }
                                        .into_any()
                                }}

                                <span class="text-[0.65rem] uppercase tracking-widest font-mono text-muted-foreground">
                                    {item.kind}
                                </span>
                            </div>

                            {if has_children {
                                view! {
                                    <div class="mt-2">
                                        {render_doc_tree(children, depth + 1)}
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

fn collect_index_entries(items: &[DocSidebarItemData], out: &mut Vec<DocSidebarItemData>) {
    for item in items {
        if item.kind != "folder" && item.kind != "folder_with_index" {
            out.push(item.clone());
        }
        collect_index_entries(&item.children, out);
    }
}

fn collect_folder_with_index_sections(
    items: &[DocSidebarItemData],
    out: &mut Vec<(DocSidebarItemData, Vec<DocSidebarItemData>)>,
) {
    for item in items {
        if item.kind == "folder_with_index" {
            let mut entries = Vec::new();
            collect_index_entries(&item.children, &mut entries);
            entries.sort_by(|a, b| a.order.cmp(&b.order));
            out.push((item.clone(), entries));
        }
        collect_folder_with_index_sections(&item.children, out);
    }
}

#[component]
pub fn DocsReferencePage() -> impl IntoView {
    let lang = use_language();
    let docs_nav: RwSignal<Vec<DocSidebarItemData>> = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);
    let did_init = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            if did_init.get() {
                return;
            }
            did_init.set(true);
            loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                let data = load_docs_nav().await;
                docs_nav.set(data);
                loading.set(false);
            });
        });
    }

    let title = move || match lang.get() {
        Language::Fr => "Reference Docs",
        Language::En => "Docs Reference",
    };

    let subtitle = move || match lang.get() {
        Language::Fr => "Index de la documentation et navigation rapide.",
        Language::En => "Documentation index and quick navigation.",
    };

    let empty_label = move || match lang.get() {
        Language::Fr => "Aucune documentation a afficher.",
        Language::En => "No documentation entry to display.",
    };

    let sidebar_label = move || match lang.get() {
        Language::Fr => "Sidebar",
        Language::En => "Sidebar",
    };

    let index_label = move || match lang.get() {
        Language::Fr => "Index rapide",
        Language::En => "Quick index",
    };

    let index_hint = move || match lang.get() {
        Language::Fr => "Index disponible uniquement pour les sections de type folder_with_index.",
        Language::En => "Index is shown only for sections of type folder_with_index.",
    };

    let index_empty = move || match lang.get() {
        Language::Fr => "Aucune section folder_with_index detectee.",
        Language::En => "No folder_with_index section detected.",
    };

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <SectionTitle>{move || title()}</SectionTitle>
                <p class="text-muted-foreground mb-6">{move || subtitle()}</p>

                {move || {
                    if loading.get() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">
                                    {move || match lang.get() {
                                        Language::Fr => "Chargement de la documentation...",
                                        Language::En => "Loading documentation...",
                                    }}
                                </p>
                            </Card>
                        }
                            .into_any();
                    }

                    if docs_nav.get().is_empty() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">{move || empty_label()}</p>
                            </Card>
                        }
                            .into_any();
                    }

                    let nav_data = docs_nav.get();
                    let mut index_sections: Vec<(DocSidebarItemData, Vec<DocSidebarItemData>)> =
                        Vec::new();
                    collect_folder_with_index_sections(&nav_data, &mut index_sections);

                    view! {
                        <div class="grid grid-cols-1 lg:grid-cols-[18rem_1fr] gap-5">
                            <aside class="lg:sticky lg:top-16 self-start">
                                <Card class="p-4 max-h-[calc(100svh-6rem)] overflow-auto">
                                    <p class="text-xs uppercase tracking-widest font-mono text-muted-foreground mb-3">
                                        {move || sidebar_label()}
                                    </p>
                                    {render_doc_tree(nav_data.clone(), 0)}
                                </Card>
                            </aside>

                            <Card class="p-5">
                                <p class="text-sm font-semibold">{move || index_label()}</p>
                                <p class="text-xs text-muted-foreground mt-1 mb-4">{move || index_hint()}</p>

                                {if index_sections.is_empty() {
                                    view! {
                                        <p class="text-sm text-muted-foreground">{move || index_empty()}</p>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="flex flex-col gap-5">
                                            {index_sections
                                                .into_iter()
                                                .map(|(section, entries)| {
                                                    let section_href =
                                                        format!("/{}", section.handle.trim_start_matches('/'));
                                                    let section_title = section.title.clone();
                                                    view! {
                                                        <div>
                                                            <a
                                                                href=section_href
                                                                class="text-sm font-semibold text-foreground hover:text-primary transition-colors"
                                                            >
                                                                {section_title}
                                                            </a>
                                                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2">
                                                                {entries
                                                                    .into_iter()
                                                                    .take(24)
                                                                    .map(|item| {
                                                                        let href =
                                                                            format!("/{}", item.handle.trim_start_matches('/'));
                                                                        let label = item.title.clone();
                                                                        view! {
                                                                            <a
                                                                                href=href
                                                                                class="text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline truncate"
                                                                            >
                                                                                {label}
                                                                            </a>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                            </div>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }}
                            </Card>
                        </div>
                    }
                        .into_any()
                }}
            </SectionInner>
        </section>
    }
}
