use leptos::prelude::*;
use serde::Deserialize;

#[cfg(not(feature = "ssr"))]
use web_sys::js_sys;

use crate::components::ui::{Card, SectionInner, SectionTitle, TagBadge};
use crate::i18n::{use_language, Language};
use crate::seo::StaticPageSeo;
use crate::services::api::fetch_json;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct TechnologyNodeData {
    title: String,
    handle: String,
    href: Option<String>,
    description: String,
    tags: Vec<String>,
    techno: Vec<String>,
    image: String,
    maturity: Option<String>,
    children: Vec<TechnologyNodeData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TechnologyVisual {
    Image(String),
    Icomoon(String),
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_technology_tree() -> Vec<TechnologyNodeData> {
    fetch_json("/api/v1/nav/technologies")
        .await
        .unwrap_or_default()
}

#[cfg(not(feature = "ssr"))]
async fn load_technology_tree_send_safe() -> Vec<TechnologyNodeData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Vec<TechnologyNodeData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_technology_tree().await);
    });
    match rx.await {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("oneshot receiver canceled while loading technology tree: {err}");
            Vec::new()
        }
    }
}

#[cfg(not(feature = "ssr"))]
fn decode_query_component(value: &str) -> String {
    let normalized = value.replace('+', " ");
    js_sys::decode_uri_component(&normalized)
        .map(|s| s.into())
        .unwrap_or(normalized)
}

#[cfg(not(feature = "ssr"))]
fn encode_query_component(value: &str) -> String {
    js_sys::encode_uri_component(value).into()
}

#[cfg(not(feature = "ssr"))]
fn read_search_query_from_url() -> String {
    let Some(window) = web_sys::window() else {
        return String::new();
    };
    let query = window.location().search().ok().unwrap_or_default();

    for pair in query.trim_start_matches('?').split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.splitn(2, '=');
        let key = decode_query_component(parts.next().unwrap_or_default());
        let value = decode_query_component(parts.next().unwrap_or_default());
        if key == "q" {
            return value;
        }
    }

    String::new()
}

#[cfg(not(feature = "ssr"))]
fn sync_search_query_to_url(search: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(pathname) = window.location().pathname() else {
        return;
    };

    let query = if search.trim().is_empty() {
        String::new()
    } else {
        format!("?q={}", encode_query_component(search.trim()))
    };

    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            "",
            Some(&format!("{}{}", pathname, query)),
        );
    }
}

fn maturity_badge_class(value: &str) -> &'static str {
    match value {
        "expert" => {
            "inline-flex items-center rounded border border-primary/60 bg-primary/10 px-2 py-0.5 text-[0.7rem] font-mono uppercase tracking-widest text-primary"
        }
        "advanced" => {
            "inline-flex items-center rounded border border-emerald-400/60 bg-emerald-400/10 px-2 py-0.5 text-[0.7rem] font-mono uppercase tracking-widest text-emerald-300"
        }
        "intermediate" => {
            "inline-flex items-center rounded border border-amber-400/60 bg-amber-400/10 px-2 py-0.5 text-[0.7rem] font-mono uppercase tracking-widest text-amber-300"
        }
        _ => {
            "inline-flex items-center rounded border border-border bg-muted/30 px-2 py-0.5 text-[0.7rem] font-mono uppercase tracking-widest text-muted-foreground"
        }
    }
}

fn maturity_label(value: &str, lang: Language) -> &'static str {
    match (value, lang) {
        ("expert", Language::Fr) => "Expert",
        ("expert", Language::En) => "Expert",
        ("advanced", Language::Fr) => "Avance",
        ("advanced", Language::En) => "Advanced",
        ("intermediate", Language::Fr) => "Intermediaire",
        ("intermediate", Language::En) => "Intermediate",
        ("beginner", Language::Fr) => "Debutant",
        ("beginner", Language::En) => "Beginner",
        (_, Language::Fr) => "A definir",
        (_, Language::En) => "Unset",
    }
}

fn resolve_technology_visual(raw: &str) -> Option<TechnologyVisual> {
    if raw.trim().is_empty() {
        return None;
    }
    if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with('/') {
        return Some(TechnologyVisual::Image(raw.to_string()));
    }
    if let Some(file_name) = raw.strip_prefix("media#") {
        return Some(TechnologyVisual::Image(format!(
            "/public/media/{file_name}"
        )));
    }
    if let Some(icon_name) = raw.strip_prefix("icomoon#") {
        return Some(TechnologyVisual::Icomoon(format!(
            "/assets/icon/symbol-defs.svg#ico-{icon_name}"
        )));
    }
    Some(TechnologyVisual::Image(format!(
        "/public/media/{}",
        raw.trim_start_matches('/')
    )))
}

fn node_matches_query(node: &TechnologyNodeData, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let haystack = format!(
        "{} {} {} {}",
        node.title,
        node.description,
        node.tags.join(" "),
        node.techno.join(" ")
    )
    .to_ascii_lowercase();

    haystack.contains(query)
}

fn filter_technology_tree(nodes: &[TechnologyNodeData], query: &str) -> Vec<TechnologyNodeData> {
    nodes
        .iter()
        .filter_map(|node| {
            let children = filter_technology_tree(&node.children, query);
            if query.is_empty() || node_matches_query(node, query) || !children.is_empty() {
                let mut next = node.clone();
                next.children = children;
                Some(next)
            } else {
                None
            }
        })
        .collect()
}

fn count_visible_technologies(nodes: &[TechnologyNodeData]) -> usize {
    nodes
        .iter()
        .map(|node| {
            usize::from(node.maturity.is_some()) + count_visible_technologies(&node.children)
        })
        .sum()
}

fn render_visual(visual: TechnologyVisual) -> impl IntoView {
    match visual {
        TechnologyVisual::Image(src) => view! {
            <img
                src=src
                alt=""
                class="h-10 w-10 shrink-0 rounded border border-border object-cover"
                loading="lazy"
            />
        }
        .into_any(),
        TechnologyVisual::Icomoon(href) => view! {
            <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded border border-border bg-muted/20">
                <svg class="h-6 w-6" aria-hidden="true" focusable="false" fill="#fff">
                    <use href=href></use>
                </svg>
            </div>
        }
        .into_any(),
    }
}

fn branch_technology_count(node: &TechnologyNodeData) -> usize {
    usize::from(node.maturity.is_some())
        + node
            .children
            .iter()
            .map(branch_technology_count)
            .sum::<usize>()
}

fn branch_has_only_technology_children(node: &TechnologyNodeData) -> bool {
    !node.children.is_empty() && node.children.iter().all(|child| child.maturity.is_some())
}

#[component]
fn TechnologyBranch(
    node: TechnologyNodeData,
    depth: usize,
    on_tag_click: Callback<String>,
) -> impl IntoView {
    let lang = use_language();
    let has_children = !node.children.is_empty();
    let is_technology = node.maturity.is_some();
    let direct_leaf_grid = branch_has_only_technology_children(&node);
    let technology_count = branch_technology_count(&node);
    let title = node.title.clone();
    let href = node.href.clone();
    let description = node.description.clone();
    let maturity = node.maturity.clone();
    let tags = node.tags.clone();
    let children = node.children.clone();
    let visual = resolve_technology_visual(&node.image);
    let section_label = move || match lang.get() {
        Language::Fr => "Categorie",
        Language::En => "Category",
    };
    let technologies_label = move || match lang.get() {
        Language::Fr => "technos",
        Language::En => "techs",
    };
    let category_title_class = if depth == 0 {
        "text-xl md:text-2xl font-mono font-bold text-foreground"
    } else {
        "text-lg font-semibold text-foreground"
    };

    view! {
        <div class=if is_technology {
            "space-y-3"
        } else if depth == 0 {
            "space-y-5"
        } else {
            "space-y-4"
        }>
            {if is_technology {
                view! {
                    <Card class="p-4 bg-background/60">
                        <div class="flex items-start gap-3">
                            {visual.map(render_visual)} <div class="min-w-0 flex-1">
                                <div class="flex flex-wrap items-center gap-2">
                                    {if let Some(link) = href {
                                        view! {
                                            <a
                                                href=link
                                                class="text-base font-semibold text-foreground hover:text-primary transition-colors underline-offset-2 hover:underline"
                                            >
                                                {title.clone()}
                                            </a>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <h3 class="text-base font-semibold text-foreground">
                                                {title.clone()}
                                            </h3>
                                        }
                                            .into_any()
                                    }}
                                    {maturity
                                        .clone()
                                        .map(|value| {
                                            let label_value = value.clone();
                                            view! {
                                                <span class=maturity_badge_class(
                                                    &value,
                                                )>{move || maturity_label(&label_value, lang.get())}</span>
                                            }
                                        })}
                                </div>

                                {if !description.trim().is_empty() {
                                    view! {
                                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                                            {description}
                                        </p>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}

                                {if !tags.is_empty() {
                                    view! {
                                        <div class="mt-3 flex flex-wrap gap-1.5">
                                            {tags
                                                .into_iter()
                                                .map(|tag| {
                                                    let tag_for_click = tag.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            on:click=move |_| on_tag_click.run(tag_for_click.clone())
                                                        >
                                                            <TagBadge class="border-primary/30 text-primary hover:border-primary/60 hover:text-primary transition-colors">
                                                                {format!("#{}", tag)}
                                                            </TagBadge>
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                        </div>
                    </Card>
                }
                    .into_any()
            } else {
                view! {
                    <div class=if depth == 0 {
                        "rounded-xl border border-border/80 bg-card/70 p-5 md:p-6"
                    } else {
                        "rounded-lg border border-border/60 bg-background/40 p-4"
                    }>
                        <div class="flex flex-wrap items-center gap-3">
                            <span class="text-[0.65rem] uppercase tracking-widest font-mono text-muted-foreground">
                                {section_label}
                            </span>
                            <span class="text-xs text-muted-foreground">
                                {move || format!("{} {}", technology_count, technologies_label())}
                            </span>
                        </div>
                        <div class="mt-2 flex flex-wrap items-start gap-3">
                            <div class="min-w-0 flex-1">
                                <h3 class=category_title_class>{title.clone()}</h3>
                                {if !description.trim().is_empty() {
                                    view! {
                                        <p class="mt-2 max-w-3xl text-sm leading-6 text-muted-foreground">
                                            {description}
                                        </p>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                        </div>
                    </div>
                }
                    .into_any()
            }}
            {if has_children {
                view! {
                    <div class=if is_technology {
                        "ml-4 border-l border-border/50 pl-4 space-y-3"
                    } else if direct_leaf_grid {
                        "grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-3"
                    } else if depth == 0 {
                        "space-y-5"
                    } else {
                        "ml-4 border-l border-border/40 pl-4 space-y-4"
                    }>
                        {children
                            .into_iter()
                            .map(|child| {
                                view! {
                                    <TechnologyBranch
                                        node=child
                                        depth=depth + 1
                                        on_tag_click=on_tag_click
                                    />
                                }
                            })
                            .collect_view()}
                    </div>
                }
                    .into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn TechnologiesPage() -> impl IntoView {
    let lang = use_language();
    let technology_tree: RwSignal<Vec<TechnologyNodeData>> = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);
    let search_query = RwSignal::new(String::new());
    #[cfg(not(feature = "ssr"))]
    let query_hydrated = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            if query_hydrated.get() {
                return;
            }
            search_query.set(read_search_query_from_url());
            query_hydrated.set(true);
        });
    }

    let do_fetch = move || {
        loading.set(true);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                technology_tree.set(load_technology_tree_send_safe().await);
                loading.set(false);
            });
        }
    };

    #[cfg(feature = "ssr")]
    let _ = &do_fetch;

    #[cfg(not(feature = "ssr"))]
    do_fetch();

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        if !query_hydrated.get() {
            return;
        }
        sync_search_query_to_url(&search_query.get());
    });

    let filtered_tree = Memo::new(move |_| {
        let query = search_query.get().trim().to_ascii_lowercase();
        filter_technology_tree(&technology_tree.get(), &query)
    });

    let visible_count = Memo::new(move |_| count_visible_technologies(&filtered_tree.get()));

    let title = move || match lang.get() {
        Language::Fr => "Carte techno",
        Language::En => "Tech map",
    };

    let subtitle = move || {
        match lang.get() {
        Language::Fr => {
            "Vue d'ensemble des technologies documentees, avec niveau de maturite et acces direct vers leur doc."
        }
        Language::En => {
            "Overview of documented technologies, with maturity level and direct links to their docs."
        }
    }
    };

    let search_label = move || match lang.get() {
        Language::Fr => "Recherche par nom ou tag",
        Language::En => "Search by name or tag",
    };

    let search_placeholder = move || match lang.get() {
        Language::Fr => "Ex: kubernetes, rust, gitops...",
        Language::En => "E.g. kubernetes, rust, gitops...",
    };

    let empty_label = move || match lang.get() {
        Language::Fr => "Aucune technologie ne correspond a la recherche.",
        Language::En => "No technology matches the current search.",
    };

    let loading_label = move || match lang.get() {
        Language::Fr => "Chargement de la carte techno...",
        Language::En => "Loading tech map...",
    };

    let count_label = move || match lang.get() {
        Language::Fr => "technologies visibles",
        Language::En => "visible technologies",
    };

    let on_tag_click = Callback::new(move |tag: String| {
        search_query.set(tag);
    });

    view! {
        <StaticPageSeo
            title="Carte techno | Maxime Leriche"
            description="Carte des technologies utilisees, avec liens directs vers la documentation et niveau de maturite."
            path="/technologies"
        />
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <SectionTitle>{title}</SectionTitle>
                <p class="mb-6 text-muted-foreground">{subtitle}</p>

                <Card class="mb-6 p-4 sm:p-5">
                    <div class="grid gap-3 md:grid-cols-[minmax(0,1fr)_auto] md:items-end">
                        <label class="flex flex-col gap-1 text-sm">
                            <span class="text-muted-foreground">{search_label}</span>
                            <input
                                class="h-10 rounded border border-border bg-background px-3 text-sm"
                                type="search"
                                prop:value=move || search_query.get()
                                placeholder=search_placeholder
                                on:input=move |ev| search_query.set(event_target_value(&ev))
                            />
                        </label>
                        <div class="text-sm font-mono text-muted-foreground">
                            {move || format!("{} {}", visible_count.get(), count_label())}
                        </div>
                    </div>
                </Card>

                {move || {
                    if loading.get() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">{loading_label}</p>
                            </Card>
                        }
                            .into_any();
                    }
                    if filtered_tree.get().is_empty() {
                        return
                        view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">{empty_label}</p>
                            </Card>
                        }
                            .into_any();
                    }

                    view! {
                        <div class="space-y-6">
                            {filtered_tree
                                .get()
                                .into_iter()
                                .map(|node| {
                                    view! {
                                        <TechnologyBranch
                                            node=node
                                            depth=0
                                            on_tag_click=on_tag_click
                                        />
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }}
            </SectionInner>
        </section>
    }
}
