use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::ui::{Card, LinkCard, PageTitle, SectionInner, TagBadge};
use crate::date_utils::format_display_date;
use crate::i18n::{use_language, Language};
use crate::seo::StaticPageSeo;
use crate::services::api::json_resource;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct BlogEntryData {
    title: String,
    description: String,
    handle: String,
    date: String,
    minia: Option<String>,
    tags: Vec<String>,
    image: String,
    reading_time_minutes: usize,
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

    #[cfg(feature = "ssr")]
    let initial = crate::services::api::from_content_db(api::nav::build_blog_nav);
    #[cfg(not(feature = "ssr"))]
    let initial = None;
    // Already filtered (visible only) and sorted newest first by the content build.
    let blog_entries_res = json_resource::<Vec<BlogEntryData>>("/api/v1/nav/blog", initial);
    let blog_entries = Signal::derive(move || blog_entries_res.get().flatten().unwrap_or_default());

    let title = move || match lang.get() {
        Language::Fr => "Blog",
        Language::En => "Blog",
    };

    let subtitle = move || match lang.get() {
        Language::Fr => "Articles, retours d'expérience et notes techniques.",
        Language::En => "Articles, field notes and technical write-ups.",
    };

    let empty_label = move || match lang.get() {
        Language::Fr => "Aucun article publié pour le moment.",
        Language::En => "No article published yet.",
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
                <PageTitle>{move || title()}</PageTitle>
                <p class="text-muted-foreground mb-6">{move || subtitle()}</p>

                <Suspense fallback=move || {
                    view! {
                        <Card class="p-5">
                            <p class="text-sm text-muted-foreground">
                                {move || match lang.get() {
                                    Language::Fr => "Chargement des articles...",
                                    Language::En => "Loading blog entries...",
                                }}
                            </p>
                        </Card>
                    }
                }>
                {move || {
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
                                    let image = if !&entry.image.is_empty() {
                                        resolve_blog_image(&entry.image)
                                    } else if let Some(minia) = &entry.minia {
                                        resolve_blog_image(minia)
                                    } else {
                                        None
                                    };
                                    let tags = entry.tags;
                                    let reading_time = entry.reading_time_minutes;
                                    let target_url = format!("/{}", handle.trim_start_matches('/'));
                                    view! {
                                        <LinkCard class="p-4">
                                            {image
                                                .map(|src| {
                                                    view! {
                                                        <a href=target_url.clone()>
                                                            <img
                                                                src=src
                                                                alt=""
                                                                width="640"
                                                                height="320"
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
                                                            <TagBadge class="border-primary/30 text-primary">
                                                                {format!("#{}", tag)}
                                                            </TagBadge>
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
                                                    class="text-sm text-primary hover:text-foreground underline-offset-2 hover:underline"
                                                >
                                                    {move || read_label()}
                                                </a>
                                            </div>
                                        </LinkCard>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }}
                </Suspense>
            </SectionInner>
        </section>
    }
}
