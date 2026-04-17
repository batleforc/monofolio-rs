use leptos::prelude::*;
use serde::Deserialize;
use tw_merge::{tw_merge, IntoTailwindClass};

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::{toggle_language, use_language, use_translations, Language, Translations};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
struct SearchEntryData {
    title: String,
    description: String,
    handle: String,
    href: String,
    kind: String,
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_search_index() -> Vec<SearchEntryData> {
    #[cfg(not(feature = "ssr"))]
    {
        let resp = match gloo_net::http::Request::get("/api/v1/search").send().await {
            Ok(resp) => resp,
            Err(_) => return vec![],
        };
        let text = match resp.text().await {
            Ok(text) => text,
            Err(_) => return vec![],
        };
        serde_json::from_str::<Vec<SearchEntryData>>(&text).unwrap_or_default()
    }
    #[cfg(feature = "ssr")]
    {
        vec![]
    }
}

fn filter_search_results(items: &[SearchEntryData], query: &str) -> Vec<SearchEntryData> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return vec![];
    }

    let mut results: Vec<(u8, SearchEntryData)> = items
        .iter()
        .filter_map(|item| {
            let title = item.title.to_lowercase();
            let handle = item.handle.to_lowercase();
            let description = item.description.to_lowercase();
            let score = if title.starts_with(&needle) {
                Some(0)
            } else if title.contains(&needle) {
                Some(1)
            } else if handle.contains(&needle) {
                Some(2)
            } else if description.contains(&needle) {
                Some(3)
            } else {
                None
            }?;

            Some((score, item.clone()))
        })
        .collect();

    results.sort_by(|(score_a, item_a), (score_b, item_b)| {
        score_a
            .cmp(score_b)
            .then_with(|| item_a.title.to_lowercase().cmp(&item_b.title.to_lowercase()))
    });

    results.into_iter().map(|(_, item)| item).take(8).collect()
}

fn static_search_entries(lang: Language, t: &'static Translations) -> Vec<SearchEntryData> {
    let home_description = match lang {
        Language::Fr => "Page d'accueil du portfolio.",
        Language::En => "Portfolio home page.",
    };
    let about_description = match lang {
        Language::Fr => "Presentation et parcours.",
        Language::En => "Background and profile.",
    };
    let contact_description = match lang {
        Language::Fr => "Moyens de contact.",
        Language::En => "Ways to get in touch.",
    };

    vec![
        SearchEntryData {
            title: t.nav_home.to_string(),
            description: home_description.to_string(),
            handle: String::new(),
            href: "/".to_string(),
            kind: "page".to_string(),
        },
        SearchEntryData {
            title: t.nav_about.to_string(),
            description: about_description.to_string(),
            handle: "about".to_string(),
            href: "/about".to_string(),
            kind: "page".to_string(),
        },
        SearchEntryData {
            title: t.nav_contact.to_string(),
            description: contact_description.to_string(),
            handle: "contact".to_string(),
            href: "/contact".to_string(),
            kind: "page".to_string(),
        },
    ]
}

/// Top navigation bar with language toggle.
#[component]
pub fn NavBar() -> impl IntoView {
    let lang = use_language();
    let t = use_translations();
    let search_query = RwSignal::new(String::new());
    let search_index: RwSignal<Vec<SearchEntryData>> = RwSignal::new(vec![]);
    let search_loading = RwSignal::new(true);
    let mobile_menu_open = RwSignal::new(false);
    #[cfg(not(feature = "ssr"))]
    let did_init = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            if did_init.get() {
                return;
            }
            did_init.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                search_index.set(load_search_index().await);
                search_loading.set(false);
            });
        });
    }

    let lang_btn_class = tw_merge!(
        ButtonClass {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
        }
        .to_class(),
        "border border-border font-mono text-xs tracking-widest"
    );

    let search_placeholder = move || match lang.get() {
        Language::Fr => "Rechercher une page...",
        Language::En => "Search a page...",
    };

    let search_empty = move || match lang.get() {
        Language::Fr => "Aucun resultat.",
        Language::En => "No result.",
    };

    let search_loading_label = move || match lang.get() {
        Language::Fr => "Chargement de l'index...",
        Language::En => "Loading index...",
    };

    view! {
        <header class="sticky top-0 z-50 border-b border-border backdrop-blur-md bg-background/90 cyber-grid-bg">
            <div class="max-w-5xl mx-auto px-5 h-14 flex items-center gap-4">
                <a
                    href="/"
                    class="font-mono font-bold text-xl text-primary tracking-tight cyber-text-glow"
                    aria-label="Home"
                >
                    "Max."
                </a>

                <nav class="hidden md:flex gap-6 flex-1 min-w-0" aria-label="Main navigation">
                    <a
                        href="/"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_home}
                    </a>
                    <a
                        href="/contact"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_contact}
                    </a>
                    <a
                        href="/projects"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_projects}
                    </a>
                    <a
                        href="/blog"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_blog}
                    </a>
                    <a
                        href="/docs"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_docs}
                    </a>
                    <a
                        href="/about"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors whitespace-nowrap"
                    >
                        {move || t.get().nav_about}
                    </a>
                </nav>

                <div class="relative w-72 shrink-0 hidden md:block">
                    <input
                        type="search"
                        prop:value=move || search_query.get()
                        on:input=move |ev| search_query.set(event_target_value(&ev))
                        placeholder=move || search_placeholder()
                        class="w-full rounded border border-border bg-background/70 px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground/70 focus:border-primary focus:outline-none"
                    />

                    {move || {
                        let query = search_query.get();
                        if query.trim().is_empty() {
                            return view! { <></> }.into_any();
                        }
                        let mut all_items = search_index.get();
                        all_items.extend(static_search_entries(lang.get(), t.get()));
                        let results = filter_search_results(&all_items, &query);
                        let is_loading = search_loading.get() && search_index.get().is_empty();

                        view! {
                            <div class="absolute right-0 top-full mt-2 w-[28rem] max-w-[calc(100vw-2rem)] rounded border border-border bg-card text-card-foreground shadow-lg overflow-hidden">
                                {if is_loading {
                                    view! {
                                        <p class="px-4 py-3 text-sm text-muted-foreground">
                                            {move || search_loading_label()}
                                        </p>
                                    }
                                        .into_any()
                                } else if results.is_empty() {
                                    view! {
                                        <p class="px-4 py-3 text-sm text-muted-foreground">
                                            {move || search_empty()}
                                        </p>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <ul class="max-h-[26rem] overflow-auto">
                                            {results
                                                .into_iter()
                                                .map(|item| {
                                                    let title = item.title.clone();
                                                    let description = item.description.clone();
                                                    let href = item.href.clone();
                                                    let kind = item.kind.clone();
                                                    view! {
                                                        <li class="border-t border-border first:border-t-0">
                                                            <a
                                                                href=href
                                                                class="block px-4 py-3 hover:bg-primary/5 transition-colors"
                                                                on:click=move |_| search_query.set(String::new())
                                                            >
                                                                <div class="flex min-w-0 items-center justify-between gap-3">
                                                                    <p class="min-w-0 text-sm font-semibold text-foreground truncate">
                                                                        {title}
                                                                    </p>
                                                                    <span class="text-[0.65rem] uppercase tracking-widest font-mono text-muted-foreground">
                                                                        {kind}
                                                                    </span>
                                                                </div>
                                                                <p class="mt-1 text-xs text-muted-foreground truncate">
                                                                    {description}
                                                                </p>
                                                            </a>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    }
                                        .into_any()
                                }}
                            </div>
                        }
                            .into_any()
                    }}
                </div>

                <a
                    href="/rss.xml"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="RSS feed"
                    class="hidden md:inline-flex items-center justify-center rounded border border-border px-2 py-1 text-muted-foreground hover:text-foreground hover:border-primary transition-colors"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                    >
                        <path d="M6.18 15.64a2.18 2.18 0 0 1 2.18 2.18C8.36 19.01 7.38 20 6.18 20C4.98 20 4 19.01 4 17.82a2.18 2.18 0 0 1 2.18-2.18M4 4.44A15.56 15.56 0 0 1 19.56 20h-2.83A12.73 12.73 0 0 0 4 7.27V4.44m0 5.66a9.9 9.9 0 0 1 9.9 9.9h-2.83A7.07 7.07 0 0 0 4 12.93V10.1z" />
                    </svg>
                </a>

                <button
                    class=tw_merge!(lang_btn_class.clone(), "hidden md:inline-flex")
                    aria-label="Toggle language"
                    on:click=move |_| toggle_language(lang)
                >
                    {move || lang.get().toggle_label()}
                </button>

                <button
                    class="md:hidden inline-flex items-center justify-center rounded border border-border px-2 py-1 text-sm font-mono tracking-wide text-muted-foreground hover:text-foreground hover:border-primary transition-colors"
                    aria-label="Toggle menu"
                    on:click=move |_| mobile_menu_open.update(|open| *open = !*open)
                >
                    {move || if mobile_menu_open.get() { "✕" } else { "☰" }}
                </button>
            </div>

            <Show when=move || mobile_menu_open.get()>
                <div class="md:hidden border-t border-border bg-background/95 backdrop-blur-md">
                    <div class="max-w-5xl mx-auto px-5 py-4 flex flex-col gap-4">
                        <nav class="flex min-w-0 flex-col gap-2" aria-label="Mobile navigation">
                            <a
                                href="/"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_home}
                            </a>
                            <a
                                href="/contact"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_contact}
                            </a>
                            <a
                                href="/projects"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_projects}
                            </a>
                            <a
                                href="/blog"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_blog}
                            </a>
                            <a
                                href="/docs"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_docs}
                            </a>
                            <a
                                href="/about"
                                class="py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors break-words"
                                on:click=move |_| mobile_menu_open.set(false)
                            >
                                {move || t.get().nav_about}
                            </a>
                        </nav>

                        <div class="relative w-full">
                            <input
                                type="search"
                                prop:value=move || search_query.get()
                                on:input=move |ev| search_query.set(event_target_value(&ev))
                                placeholder=move || search_placeholder()
                                class="w-full rounded border border-border bg-background/70 px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground/70 focus:border-primary focus:outline-none"
                            />

                            {move || {
                                let query = search_query.get();
                                if query.trim().is_empty() {
                                    return view! { <></> }.into_any();
                                }
                                let mut all_items = search_index.get();
                                all_items.extend(static_search_entries(lang.get(), t.get()));
                                let results = filter_search_results(&all_items, &query);
                                let is_loading = search_loading.get()
                                    && search_index.get().is_empty();

                                view! {
                                    <div class="absolute left-0 right-0 top-full mt-2 w-full rounded border border-border bg-card text-card-foreground shadow-lg overflow-hidden">
                                        {if is_loading {
                                            view! {
                                                <p class="px-4 py-3 text-sm text-muted-foreground">
                                                    {move || search_loading_label()}
                                                </p>
                                            }
                                                .into_any()
                                        } else if results.is_empty() {
                                            view! {
                                                <p class="px-4 py-3 text-sm text-muted-foreground">
                                                    {move || search_empty()}
                                                </p>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <ul class="max-h-[26rem] overflow-auto">
                                                    {results
                                                        .into_iter()
                                                        .map(|item| {
                                                            let title = item.title.clone();
                                                            let description = item.description.clone();
                                                            let href = item.href.clone();
                                                            let kind = item.kind.clone();
                                                            view! {
                                                                <li class="border-t border-border first:border-t-0">
                                                                    <a
                                                                        href=href
                                                                        class="block px-4 py-3 hover:bg-primary/5 transition-colors"
                                                                        on:click=move |_| {
                                                                            search_query.set(String::new());
                                                                            mobile_menu_open.set(false);
                                                                        }
                                                                    >
                                                                        <div class="flex min-w-0 items-center justify-between gap-3">
                                                                            <p class="min-w-0 text-sm font-semibold text-foreground truncate">
                                                                                {title}
                                                                            </p>
                                                                            <span class="text-[0.65rem] uppercase tracking-widest font-mono text-muted-foreground">
                                                                                {kind}
                                                                            </span>
                                                                        </div>
                                                                        <p class="mt-1 text-xs text-muted-foreground truncate">
                                                                            {description}
                                                                        </p>
                                                                    </a>
                                                                </li>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </ul>
                                            }
                                                .into_any()
                                        }}
                                    </div>
                                }
                                    .into_any()
                            }}
                        </div>

                        <a
                            href="/rss.xml"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="RSS feed"
                            class="inline-flex items-center justify-center rounded border border-border px-2 py-1 text-muted-foreground hover:text-foreground hover:border-primary transition-colors"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="14"
                                height="14"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                            >
                                <path d="M6.18 15.64a2.18 2.18 0 0 1 2.18 2.18C8.36 19.01 7.38 20 6.18 20C4.98 20 4 19.01 4 17.82a2.18 2.18 0 0 1 2.18-2.18M4 4.44A15.56 15.56 0 0 1 19.56 20h-2.83A12.73 12.73 0 0 0 4 7.27V4.44m0 5.66a9.9 9.9 0 0 1 9.9 9.9h-2.83A7.07 7.07 0 0 0 4 12.93V10.1z" />
                            </svg>
                        </a>

                        <button
                            class=lang_btn_class.clone()
                            aria-label="Toggle language"
                            on:click=move |_| toggle_language(lang)
                        >
                            {move || lang.get().toggle_label()}
                        </button>
                    </div>
                </div>
            </Show>
        </header>
    }
}
