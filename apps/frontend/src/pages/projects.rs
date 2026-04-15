use leptos::prelude::*;
use serde::Deserialize;
use tw_merge::IntoTailwindClass;

#[cfg(not(feature = "ssr"))]
use web_sys::js_sys;

use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::i18n::{use_language, Language};

const PROJECTS_PER_PAGE: usize = 6;

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct ProjectSummaryData {
    title: String,
    description: String,
    handle: String,
    tags: Vec<String>,
    techno: Vec<String>,
    image: String,
    released_at: String,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_projects_data() -> Vec<ProjectSummaryData> {
    #[cfg(not(feature = "ssr"))]
    {
        let resp = match gloo_net::http::Request::get("/api/v1/nav/projects")
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(_) => return vec![],
        };
        let text = match resp.text().await {
            Ok(text) => text,
            Err(_) => return vec![],
        };
        serde_json::from_str::<Vec<ProjectSummaryData>>(&text).unwrap_or_default()
    }
    #[cfg(feature = "ssr")]
    {
        vec![]
    }
}

fn resolve_project_image(raw: &str) -> Option<String> {
    if raw.trim().is_empty() {
        return None;
    }
    if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with('/') {
        return Some(raw.to_string());
    }
    if let Some(file_name) = raw.strip_prefix("media#") {
        return Some(format!("/public/media/{}", file_name));
    }
    Some(format!("/public/media/{}", raw))
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
fn read_projects_query_from_url() -> (String, String, String, usize) {
    let Some(window) = web_sys::window() else {
        return (String::new(), String::from("all"), String::from("all"), 1);
    };

    let query = window.location().search().ok().unwrap_or_default();
    let mut search = String::new();
    let mut tag = String::from("all");
    let mut techno = String::from("all");
    let mut page = 1usize;

    for pair in query.trim_start_matches('?').split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        let val = parts.next().unwrap_or_default();
        let decoded_key = decode_query_component(key);
        let decoded_val = decode_query_component(val);

        match decoded_key.as_str() {
            "q" => search = decoded_val,
            "tag" if !decoded_val.is_empty() => tag = decoded_val,
            "tech" if !decoded_val.is_empty() => techno = decoded_val,
            "page" => {
                if let Ok(parsed) = decoded_val.parse::<usize>() {
                    page = parsed.max(1);
                }
            }
            _ => {}
        }
    }

    (search, tag, techno, page)
}

#[cfg(not(feature = "ssr"))]
fn sync_projects_query_to_url(search: &str, tag: &str, techno: &str, page: usize) {
    let mut pairs: Vec<String> = Vec::new();

    if !search.trim().is_empty() {
        pairs.push(format!("q={}", encode_query_component(search.trim())));
    }
    if tag != "all" {
        pairs.push(format!("tag={}", encode_query_component(tag)));
    }
    if techno != "all" {
        pairs.push(format!("tech={}", encode_query_component(techno)));
    }
    if page > 1 {
        pairs.push(format!("page={}", page));
    }

    let query = if pairs.is_empty() {
        String::new()
    } else {
        format!("?{}", pairs.join("&"))
    };

    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(pathname) = window.location().pathname() else {
        return;
    };

    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            "",
            Some(&format!("{}{}", pathname, query)),
        );
    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let lang = use_language();

    let projects_data: RwSignal<Vec<ProjectSummaryData>> = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let search_query = RwSignal::new(String::new());
    let selected_tag = RwSignal::new(String::from("all"));
    let selected_techno = RwSignal::new(String::from("all"));
    let current_page = RwSignal::new(1usize);
    let query_hydrated = RwSignal::new(false);

    let search_input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            if query_hydrated.get() {
                return;
            }
            let (q, tag, techno, page) = read_projects_query_from_url();
            search_query.set(q);
            selected_tag.set(tag);
            selected_techno.set(techno);
            current_page.set(page.max(1));
            query_hydrated.set(true);
        });
    }

    #[cfg(feature = "ssr")]
    {
        let _ = &query_hydrated;
    }

    let do_fetch = move || {
        loading.set(true);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let mut projects = load_projects_data().await;
                projects.sort_by(|a, b| b.released_at.cmp(&a.released_at));
                projects_data.set(projects);
                loading.set(false);
            });
        }
    };

    #[cfg(feature = "ssr")]
    let _ = &do_fetch;

    #[cfg(not(feature = "ssr"))]
    do_fetch();

    let page_title = move || match lang.get() {
        Language::Fr => "Projets",
        Language::En => "Projects",
    };

    let page_intro = move || match lang.get() {
        Language::Fr => "Tous les projets, avec filtres et pagination.",
        Language::En => "All projects, with filtering and pagination.",
    };

    let search_label = move || match lang.get() {
        Language::Fr => "Recherche",
        Language::En => "Search",
    };

    let search_placeholder = move || match lang.get() {
        Language::Fr => "Titre, description, tag, techno...",
        Language::En => "Title, description, tag, tech...",
    };

    let tag_label = move || match lang.get() {
        Language::Fr => "Tag",
        Language::En => "Tag",
    };

    let techno_label = move || match lang.get() {
        Language::Fr => "Technologie",
        Language::En => "Technology",
    };

    let all_label = move || match lang.get() {
        Language::Fr => "Tous",
        Language::En => "All",
    };

    let no_result_label = move || match lang.get() {
        Language::Fr => "Aucun projet ne correspond aux filtres.",
        Language::En => "No project matches your filters.",
    };

    let prev_label = move || match lang.get() {
        Language::Fr => "Precedent",
        Language::En => "Previous",
    };

    let next_label = move || match lang.get() {
        Language::Fr => "Suivant",
        Language::En => "Next",
    };

    let project_link_label = move || match lang.get() {
        Language::Fr => "Ouvrir",
        Language::En => "Open",
    };

    let unique_tags = Memo::new(move |_| {
        let mut tags = projects_data
            .get()
            .into_iter()
            .flat_map(|p| p.tags)
            .collect::<Vec<_>>();
        tags.sort();
        tags.dedup();
        tags
    });

    let unique_techno = Memo::new(move |_| {
        let mut techno = projects_data
            .get()
            .into_iter()
            .flat_map(|p| p.techno)
            .collect::<Vec<_>>();
        techno.sort();
        techno.dedup();
        techno
    });

    let filtered_projects = Memo::new(move |_| {
        let query = search_query.get().trim().to_ascii_lowercase();
        let tag_filter = selected_tag.get();
        let techno_filter = selected_techno.get();

        projects_data
            .get()
            .into_iter()
            .filter(|project| {
                let match_query = if query.is_empty() {
                    true
                } else {
                    let haystack = format!(
                        "{} {} {} {}",
                        project.title,
                        project.description,
                        project.tags.join(" "),
                        project.techno.join(" "),
                    )
                    .to_ascii_lowercase();
                    haystack.contains(&query)
                };

                let match_tag = if tag_filter == "all" {
                    true
                } else {
                    project.tags.iter().any(|tag| tag == &tag_filter)
                };

                let match_techno = if techno_filter == "all" {
                    true
                } else {
                    project.techno.iter().any(|tech| tech == &techno_filter)
                };

                match_query && match_tag && match_techno
            })
            .collect::<Vec<_>>()
    });

    let total_pages = Memo::new(move |_| {
        let len = filtered_projects.get().len();
        let pages = len.div_ceil(PROJECTS_PER_PAGE);
        pages.max(1)
    });

    let paged_projects = Memo::new(move |_| {
        let projects = filtered_projects.get();
        let page = current_page.get().max(1);
        let start = (page - 1) * PROJECTS_PER_PAGE;
        projects
            .into_iter()
            .skip(start)
            .take(PROJECTS_PER_PAGE)
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        if loading.get() {
            return;
        }
        let total = total_pages.get();
        let page = current_page.get();
        if page > total {
            current_page.set(total);
        }
        if page == 0 {
            current_page.set(1);
        }
    });

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        if !query_hydrated.get() {
            return;
        }

        sync_projects_query_to_url(
            &search_query.get(),
            &selected_tag.get(),
            &selected_techno.get(),
            current_page.get().max(1),
        );
    });

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        if !query_hydrated.get() {
            return;
        }

        let search = search_query.get();
        if let Some(input) = search_input_ref.get() {
            input.set_value(&search);
        }
    });

    let ghost_btn_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    }
    .to_class();

    let outline_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();

    let prev_disabled = move || current_page.get() <= 1;
    let next_disabled = move || current_page.get() >= total_pages.get();

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <SectionTitle>{page_title}</SectionTitle>
                <p class="text-muted-foreground mb-6">{page_intro}</p>

                <Card class="p-4 sm:p-5 mb-6">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                        <label class="flex flex-col gap-1 text-sm">
                            <span class="text-muted-foreground">{search_label}</span>
                            <input
                                node_ref=search_input_ref
                                class="h-10 px-3 rounded border border-border bg-background text-sm"
                                type="text"
                                placeholder=search_placeholder
                                value=move || search_query.get()
                                on:input=move |ev| {
                                    search_query.set(event_target_value(&ev));
                                    current_page.set(1);
                                }
                            />
                        </label>

                        <label class="flex flex-col gap-1 text-sm">
                            <span class="text-muted-foreground">{tag_label}</span>
                            <select
                                class="h-10 px-3 rounded border border-border bg-background text-sm"
                                on:change=move |ev| {
                                    selected_tag.set(event_target_value(&ev));
                                    current_page.set(1);
                                }
                            >
                                <option value="all" selected=move || selected_tag.get() == "all">{all_label}</option>
                                {move || {
                                    unique_tags
                                        .get()
                                        .into_iter()
                                        .map(|tag| {
                                            let option_value = tag.clone();
                                            let option_label = tag.clone();
                                            let selected_value = option_value.clone();
                                            view! {
                                                <option
                                                    value=option_value
                                                    selected=move || selected_tag.get() == selected_value
                                                >
                                                    {option_label}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </label>

                        <label class="flex flex-col gap-1 text-sm">
                            <span class="text-muted-foreground">{techno_label}</span>
                            <select
                                class="h-10 px-3 rounded border border-border bg-background text-sm"
                                on:change=move |ev| {
                                    selected_techno.set(event_target_value(&ev));
                                    current_page.set(1);
                                }
                            >
                                <option
                                    value="all"
                                    selected=move || selected_techno.get() == "all"
                                >
                                    {all_label}
                                </option>
                                {move || {
                                    unique_techno
                                        .get()
                                        .into_iter()
                                        .map(|tech| {
                                            let option_value = tech.clone();
                                            let option_label = tech.clone();
                                            let selected_value = option_value.clone();
                                            view! {
                                                <option
                                                    value=option_value
                                                    selected=move || selected_techno.get() == selected_value
                                                >
                                                    {option_label}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </label>
                    </div>
                </Card>

                {move || {
                    if loading.get() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">
                                    {move || match lang.get() {
                                        Language::Fr => "Chargement des projets...",
                                        Language::En => "Loading projects...",
                                    }}
                                </p>
                            </Card>
                        }
                            .into_any();
                    }

                    if filtered_projects.get().is_empty() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">{no_result_label}</p>
                            </Card>
                        }
                            .into_any();
                    }

                    view! {
                        <>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                {paged_projects
                                    .get()
                                    .into_iter()
                                    .map(|project| {
                                        let title = project.title;
                                        let description = project.description;
                                        let released_at = project.released_at;
                                        let handle = project.handle;
                                        let tags = project.tags;
                                        let techno = project.techno;
                                        let image = resolve_project_image(&project.image);
                                        let project_url = format!("/{}", handle.trim_start_matches('/'));
                                        view! {
                                            <Card class="p-4">
                                                {image
                                                    .map(|src| {
                                                        view! {
                                                            <a href=project_url.clone()>
                                                                <img
                                                                    src=src
                                                                    alt=""
                                                                    class="w-full h-40 object-cover rounded border border-border mb-3"
                                                                    loading="lazy"
                                                                />
                                                            </a>
                                                        }
                                                    })}
                                                <span class="text-[0.7rem] uppercase tracking-widest font-mono font-semibold text-accent">
                                                    {released_at}
                                                </span>
                                                <h3 class="text-lg font-bold mt-1 mb-1">
                                                    <a
                                                        href=project_url.clone()
                                                        class="hover:text-primary transition-colors underline-offset-2 hover:underline"
                                                    >
                                                        {title}
                                                    </a>
                                                </h3>
                                                <p class="text-sm text-muted-foreground line-clamp-3 mb-3">{description}</p>
                                                <div class="flex flex-wrap gap-1.5 mb-3">
                                                    {techno
                                                        .into_iter()
                                                        .take(4)
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
                                                        .take(3)
                                                        .map(|item| {
                                                            view! {
                                                                <span class="inline-flex items-center px-2 py-0.5 rounded border border-primary/30 text-[0.7rem] text-primary">
                                                                    {format!("#{}", item)}
                                                                </span>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                                <div class="flex items-center justify-between gap-2">
                                                    <p class="text-xs text-muted-foreground font-mono truncate">{handle.clone()}</p>
                                                    <a
                                                        href=project_url
                                                        class="text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                    >
                                                        {project_link_label}
                                                    </a>
                                                </div>
                                            </Card>
                                        }
                                    })
                                    .collect_view()}
                            </div>

                            <div class="flex items-center justify-between gap-3 mt-6">
                                <button
                                    class=ghost_btn_class.clone()
                                    disabled=prev_disabled
                                    on:click=move |_| {
                                        let p = current_page.get();
                                        if p > 1 {
                                            current_page.set(p - 1);
                                        }
                                    }
                                >
                                    {prev_label}
                                </button>

                                <p class="text-sm text-muted-foreground font-mono">
                                    {format!(
                                        "{}/{}",
                                        current_page.get().max(1),
                                        total_pages.get().max(1),
                                    )}
                                </p>

                                <button
                                    class=outline_btn_class.clone()
                                    disabled=next_disabled
                                    on:click=move |_| {
                                        let p = current_page.get();
                                        let total = total_pages.get();
                                        if p < total {
                                            current_page.set(p + 1);
                                        }
                                    }
                                >
                                    {next_label}
                                </button>
                            </div>
                        </>
                    }
                        .into_any()
                }}
            </SectionInner>
        </section>
    }
}
