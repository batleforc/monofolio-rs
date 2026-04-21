use leptos::prelude::*;
use serde::Deserialize;

use crate::components::ui::{Card, SectionInner, SectionTitle};
use crate::date_utils::format_display_date;
use crate::i18n::{use_language, Language};
use crate::services::api::fetch_json;
use crate::seo::StaticPageSeo;

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct BlogEntryData {
    title: String,
    description: String,
    handle: String,
    date: String,
    tags: Vec<String>,
    image: String,
    reading_time_minutes: usize,
    draft: bool,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_blog_entries() -> Vec<BlogEntryData> {
    fetch_json("/api/v1/nav/blog").await.unwrap_or_default()
}

fn resolve_blog_image(raw: &str) -> Option<String> {
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

#[component]
pub fn BlogReferencePage() -> impl IntoView {
    let lang = use_language();

    let blog_entries: RwSignal<Vec<BlogEntryData>> = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);

    let do_fetch = move || {
        loading.set(true);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let mut entries: Vec<BlogEntryData> = load_blog_entries()
                    .await
                    .into_iter()
                    .filter(|entry| !entry.draft)
                    .collect();
                entries.sort_by(|a, b| b.date.cmp(&a.date));
                blog_entries.set(entries);
                loading.set(false);
            });
        }
    };

    #[cfg(feature = "ssr")]
    let _ = &do_fetch;

    #[cfg(not(feature = "ssr"))]
    do_fetch();

    let title = move || match lang.get() {
        Language::Fr => "Reference Blog",
        Language::En => "Blog Reference",
    };

    let subtitle = move || match lang.get() {
        Language::Fr => "Liste des articles de blog et acces rapide.",
        Language::En => "Blog entries with quick access.",
    };

    let empty_label = move || match lang.get() {
        Language::Fr => "Aucun article de blog a afficher.",
        Language::En => "No blog entry to display.",
    };

    let read_label = move || match lang.get() {
        Language::Fr => "Lire",
        Language::En => "Read",
    };

    let reading_time_label = move || match lang.get() {
        Language::Fr => "min de lecture",
        Language::En => "min read",
    };

    view! {
        <StaticPageSeo
            title="Blog | Maxime Leriche"
            description="Articles techniques et retours d'expérience autour du développement logiciel."
            path="/blog"
        />
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
                                        Language::Fr => "Chargement des articles...",
                                        Language::En => "Loading blog entries...",
                                    }}
                                </p>
                            </Card>
                        }
                            .into_any();
                    }
                    if blog_entries.get().is_empty() {
                        return view! {
                            <Card class="p-5">
                                <p class="text-sm text-muted-foreground">{move || empty_label()}</p>
                            </Card>
                        }
                            .into_any();
                    }

                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            {blog_entries
                                .get()
                                .into_iter()
                                .map(|entry| {
                                    let title = entry.title;
                                    let description = entry.description;
                                    let date_raw = entry.date;
                                    let handle = entry.handle;
                                    let image = resolve_blog_image(&entry.image);
                                    let tags = entry.tags;
                                    let reading_time = entry.reading_time_minutes;
                                    let target_url = format!("/{}", handle.trim_start_matches('/'));
                                    view! {
                                        <Card class="p-4">
                                            {image
                                                .map(|src| {
                                                    view! {
                                                        <a href=target_url.clone()>
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
                                                {move || format_display_date(&date_raw, lang.get())}
                                            </span> <h3 class="text-lg font-bold mt-1 mb-1">
                                                <a
                                                    href=target_url.clone()
                                                    class="hover:text-primary transition-colors underline-offset-2 hover:underline"
                                                >
                                                    {title}
                                                </a>
                                            </h3>
                                            <p class="text-sm text-muted-foreground line-clamp-3 mb-3">
                                                {description}
                                            </p>
                                            <div class="flex flex-wrap gap-1.5 mb-3">
                                                {tags
                                                    .into_iter()
                                                    .take(4)
                                                    .map(|tag| {
                                                        view! {
                                                            <span class="inline-flex items-center px-2 py-0.5 rounded border border-primary/30 text-[0.7rem] text-primary">
                                                                {format!("#{}", tag)}
                                                            </span>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div> <div class="flex items-center justify-between gap-2">
                                                <p class="text-xs text-muted-foreground">
                                                    {move || {
                                                        format!("{} {}", reading_time, reading_time_label())
                                                    }}
                                                </p>
                                                <a
                                                    href=target_url
                                                    class="text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                >
                                                    {move || read_label()}
                                                </a>
                                            </div>
                                        </Card>
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
