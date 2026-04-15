use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tw_merge::IntoTailwindClass;

use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::components::{about::About, hero::Hero};
use crate::i18n::{use_language, use_translations, Language};

const LAST_PROJECTS_COUNT: usize = 6;

// ── Data types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SocialLinkData {
    pub name: String,
    pub url: String,
    pub primaire: bool,
    pub img_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoryEntryData {
    pub title: String,
    pub title_en: Option<String>,
    pub lieux: String,
    pub date: String,
    pub weight: u32,
    pub img_url: String,
    pub ico_url: String,
    pub description: String,
    pub description_en: Option<String>,
    pub url: Vec<SocialLinkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HomeData {
    pub name: String,
    pub presentation: String,
    pub presentation_en: Option<String>,
    pub short_description: String,
    pub short_description_en: Option<String>,
    pub cover_title: Vec<String>,
    pub cover_title_en: Option<Vec<String>>,
    pub cv_url: String,
    pub contact_email: Option<String>,
    pub contact_location: Option<String>,
    pub current_work: Option<String>,
    pub contact_availability: Option<String>,
    pub contact_availability_en: Option<String>,
    pub url: Vec<SocialLinkData>,
    pub history: Vec<HistoryEntryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
struct ProjectSummaryData {
    title: String,
    description: String,
    handle: String,
    tags: Vec<String>,
    techno: Vec<String>,
    image: String,
    released_at: String,
}

// ── Fetch (client-only) ───────────────────────────────────────────────────

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_home_data() -> Option<HomeData> {
    #[cfg(not(feature = "ssr"))]
    {
        fetch_home_from_api().await
    }
    #[cfg(feature = "ssr")]
    {
        None
    }
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

#[cfg(not(feature = "ssr"))]
async fn fetch_home_from_api() -> Option<HomeData> {
    let resp = gloo_net::http::Request::get("/api/v1/home")
        .send()
        .await
        .ok()?;
    let text = resp.text().await.ok()?;
    serde_json::from_str::<HomeData>(&text).ok()
}

// ── Sub-components ────────────────────────────────────────────────────────

/// Loading skeleton shown while data is being fetched.
#[component]
fn LoadingScreen() -> impl IntoView {
    let t = use_translations();
    view! {
        <div
            class="min-h-[calc(100svh-3.5rem)] flex flex-col items-center justify-center gap-5 text-muted-foreground"
            role="status"
            aria-live="polite"
        >
            <div
                class="w-10 h-10 border-[3px] border-border border-t-primary rounded-full animate-spin"
                aria-hidden="true"
            ></div>
            <p class="text-sm">{move || t.get().loading}</p>
        </div>
    }
}

/// Error state shown when the API fetch fails.
#[component]
fn ErrorScreen(#[prop(into)] on_retry: Callback<()>) -> impl IntoView {
    let t = use_translations();
    let btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    }
    .to_class();
    view! {
        <div
            class="min-h-[calc(100svh-3.5rem)] flex flex-col items-center justify-center gap-5 text-muted-foreground"
            role="alert"
        >
            <p>{move || t.get().error_loading}</p>
            <button class=btn_class on:click=move |_| on_retry.run(())>
                {move || t.get().error_retry}
            </button>
        </div>
    }
}

#[component]
fn LatestProjectsSection(projects: Vec<ProjectSummaryData>) -> impl IntoView {
    let lang = use_language();
    let mut latest_projects = projects;
    latest_projects.sort_by(|a, b| b.released_at.cmp(&a.released_at));

    let title = move || match lang.get() {
        Language::Fr => "Derniers projets",
        Language::En => "Last projects",
    };

    let empty = move || match lang.get() {
        Language::Fr => "Aucun projet à afficher pour le moment.",
        Language::En => "No project to display yet.",
    };

    view! {
        <section>
            <SectionInner>
                <SectionTitle>{title}</SectionTitle>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {if latest_projects.is_empty() {
                        view! {
                            <Card class="p-4 md:col-span-2">
                                <p class="text-sm text-muted-foreground">{empty}</p>
                            </Card>
                        }
                            .into_any()
                    } else {
                        latest_projects
                            .iter()
                            .take(LAST_PROJECTS_COUNT)
                            .map(|project| {
                                let title = project.title.clone();
                                let description = project.description.clone();
                                let released_at = project.released_at.clone();
                                let tags = project.tags.clone();
                                let techno = project.techno.clone();
                                let project_handle = project.handle.clone();
                                view! {
                                    <Card class="p-4">
                                        <span class="text-[0.7rem] uppercase tracking-widest font-mono font-semibold text-accent">
                                            {released_at}
                                        </span>
                                        <h3 class="text-base font-bold mt-1 mb-1">{title}</h3>
                                        <p class="text-sm text-muted-foreground line-clamp-3">{description}</p>
                                        <div class="flex flex-wrap gap-1.5 mt-3">
                                            {techno
                                                .into_iter()
                                                .take(3)
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
                                                .take(2)
                                                .map(|item| {
                                                    view! {
                                                        <span class="inline-flex items-center px-2 py-0.5 rounded border border-primary/30 text-[0.7rem] text-primary">
                                                            {format!("#{}", item)}
                                                        </span>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                        <p class="mt-3 text-xs text-muted-foreground font-mono">{project_handle}</p>
                                    </Card>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }}
                </div>
            </SectionInner>
        </section>
    }
}

/// Renders the full home page content once data is available.
#[component]
fn HomeContent(data: HomeData, projects: Vec<ProjectSummaryData>) -> impl IntoView {
    let t = use_translations();
    let contact_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <Hero data=data.clone() />
        <About data=data.clone() />
        <LatestProjectsSection projects=projects />
        <footer class="border-t border-border/70 mt-8">
            <div class="max-w-5xl mx-auto px-5 py-10 flex flex-col sm:flex-row gap-4 sm:items-center sm:justify-between">
                <p class="text-sm text-muted-foreground">"Let's build something useful."</p>
                <a href="/contact" class=contact_btn_class>
                    {move || t.get().home_footer_contact_cta}
                </a>
            </div>
        </footer>
    }
}

// ── Home page component ───────────────────────────────────────────────────

/// Home page – 100% client-hydrated.
///
/// Data is fetched from `/api/v1/home` on the client after hydration.
/// The server sends a loading skeleton; the client fills in the content.
#[component]
pub fn HomePage() -> impl IntoView {
    let home_data: RwSignal<Option<HomeData>> = RwSignal::new(None);
    let projects_data: RwSignal<Vec<ProjectSummaryData>> = RwSignal::new(vec![]);
    let loading = RwSignal::new(true);
    let fetch_error = RwSignal::new(false);

    let do_fetch = move || {
        loading.set(true);
        fetch_error.set(false);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let projects = load_projects_data().await;
                projects_data.set(projects);

                match load_home_data().await {
                    Some(data) => {
                        home_data.set(Some(data));
                        fetch_error.set(false);
                    }
                    None => {
                        fetch_error.set(true);
                    }
                }
                loading.set(false);
            });
        }
    };

    // Trigger initial fetch on mount (client-only); SSR keeps loading=true so
    // the initial SSR HTML matches the WASM initial render.
    #[cfg(not(feature = "ssr"))]
    do_fetch();

    let retry = Callback::new(move |_: ()| do_fetch());

    view! {
        <div class="home-page">
            <Show when=move || loading.get()>
                <LoadingScreen />
            </Show>
            <Show when=move || fetch_error.get() && !loading.get()>
                <ErrorScreen on_retry=retry />
            </Show>
            <Show when=move || {
                !loading.get() && !fetch_error.get()
            }>
                {move || {
                    home_data
                        .get()
                        .map(|data| view! { <HomeContent data=data projects=projects_data.get() /> })
                }}
            </Show>
        </div>
    }
}
