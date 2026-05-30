#[cfg(feature = "ssr")]
use api::visibility::is_content_visible;
#[cfg(feature = "ssr")]
use content::{ContentEntry, HomeConfig};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};
use tw_merge::IntoTailwindClass;

use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::components::{
    about::About,
    hero::Hero,
    state::{ErrorScreen, LoadingScreen},
};
use crate::date_utils::format_display_date;
use crate::i18n::{use_language, use_translations, Language};
use crate::seo::{PersonJsonLd, StaticPageSeo};
use crate::services::api::fetch_json;

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
    pub useful_links: Vec<SocialLinkData>,
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

#[cfg(feature = "ssr")]
impl From<HomeConfig> for HomeData {
    fn from(cfg: HomeConfig) -> Self {
        Self {
            name: cfg.name,
            presentation: cfg.presentation,
            presentation_en: cfg.presentation_en,
            short_description: cfg.short_description,
            short_description_en: cfg.short_description_en,
            cover_title: cfg.cover_title,
            cover_title_en: cfg.cover_title_en,
            cv_url: cfg.cv_url,
            contact_email: cfg.contact_email,
            contact_location: cfg.contact_location,
            current_work: cfg.current_work,
            contact_availability: cfg.contact_availability,
            contact_availability_en: cfg.contact_availability_en,
            url: cfg
                .url
                .into_iter()
                .map(|link| SocialLinkData {
                    name: link.name,
                    url: link.url,
                    primaire: link.primaire,
                    img_url: link.img_url,
                })
                .collect(),
            useful_links: cfg
                .useful_links
                .into_iter()
                .map(|link| SocialLinkData {
                    name: link.name,
                    url: link.url,
                    primaire: link.primaire,
                    img_url: link.img_url,
                })
                .collect(),
            history: cfg
                .history
                .into_iter()
                .map(|entry| HistoryEntryData {
                    title: entry.title,
                    title_en: entry.title_en,
                    lieux: entry.lieux,
                    date: entry.date,
                    weight: entry.weight,
                    img_url: entry.img_url,
                    ico_url: entry.ico_url,
                    description: entry.description,
                    description_en: entry.description_en,
                    url: entry
                        .url
                        .into_iter()
                        .map(|link| SocialLinkData {
                            name: link.name,
                            url: link.url,
                            primaire: link.primaire,
                            img_url: link.img_url,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<&ContentEntry> for ProjectSummaryData {
    fn from(entry: &ContentEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            tags: entry.tags.clone(),
            techno: entry.techno.clone(),
            image: entry.image.clone(),
            released_at: entry.dates.released_at.clone(),
        }
    }
}

// ── Fetch (client-only) ───────────────────────────────────────────────────

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_home_data() -> Option<HomeData> {
    fetch_json("/api/v1/home").await
}

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_projects_data() -> Vec<ProjectSummaryData> {
    fetch_json::<Vec<ProjectSummaryData>>("/api/v1/nav/projects")
        .await
        .unwrap_or_default()
}

#[cfg(not(feature = "ssr"))]
async fn load_home_data_send_safe() -> Option<HomeData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Option<HomeData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_home_data().await);
    });
    match rx.await {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("oneshot receiver canceled in send-safe fetch: {err}");
            None
        }
    }
}

#[cfg(not(feature = "ssr"))]
async fn load_projects_data_send_safe() -> Vec<ProjectSummaryData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Vec<ProjectSummaryData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_projects_data().await);
    });
    match rx.await {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("oneshot receiver canceled in send-safe fetch: {err}");
            Vec::new()
        }
    }
}

// ── Sub-components ────────────────────────────────────────────────────────

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

    let open_label = move || match lang.get() {
        Language::Fr => "Ouvrir",
        Language::En => "Open",
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
                                let released_at_raw = project.released_at.clone();
                                let tags = project.tags.clone();
                                let techno = project.techno.clone();
                                let project_handle = project.handle.clone();
                                let project_url = format!(
                                    "/{}",
                                    project.handle.trim_start_matches('/'),
                                );
                                view! {
                                    <Card class="p-4">
                                        <span class="text-[0.7rem] uppercase tracking-widest font-mono font-semibold text-accent">
                                            {move || format_display_date(&released_at_raw, lang.get())}
                                        </span>
                                        <h3 class="text-base font-bold mt-1 mb-1">
                                            <a
                                                href=project_url.clone()
                                                class="hover:text-primary transition-colors underline-offset-2 hover:underline"
                                            >
                                                {title}
                                            </a>
                                        </h3>
                                        <p class="text-sm text-muted-foreground line-clamp-3">
                                            {description}
                                        </p>
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
                                        <div class="mt-3 flex items-center justify-between gap-2">
                                            <p class="text-xs text-muted-foreground font-mono truncate">
                                                {project_handle}
                                            </p>
                                            <a
                                                href=project_url
                                                class="text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                            >
                                                {open_label}
                                            </a>
                                        </div>
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
        <PersonJsonLd data=data.clone() />
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

/// Home page - 100% client-hydrated.
///
/// Data is fetched from `/api/v1/home` on the client after hydration.
/// The server sends a loading skeleton; the client fills in the content.
#[component]
pub fn HomePage() -> impl IntoView {
    let location = use_location();
    #[cfg(feature = "ssr")]
    let home_config = use_context::<HomeConfig>();
    #[cfg(feature = "ssr")]
    let content_database = use_context::<content::ContentDatabase>();
    let home_data = Resource::new(move || location.pathname.get(), {
        #[cfg(feature = "ssr")]
        let home_config = home_config.clone();
        move |_| {
            #[cfg(feature = "ssr")]
            let home_config = home_config.clone();
            async move {
                #[cfg(not(feature = "ssr"))]
                {
                    load_home_data_send_safe().await
                }
                #[cfg(feature = "ssr")]
                {
                    home_config.map(HomeData::from)
                }
            }
        }
    });
    let projects_data = Resource::new(move || location.pathname.get(), {
        #[cfg(feature = "ssr")]
        let content_database = content_database.clone();
        move |_| {
            #[cfg(feature = "ssr")]
            let content_database = content_database.clone();
            async move {
                #[cfg(not(feature = "ssr"))]
                {
                    load_projects_data_send_safe().await
                }
                #[cfg(feature = "ssr")]
                {
                    content_database
                        .map(|db| {
                            db.entries
                                .iter()
                                .filter(|entry| entry.kind.project && is_content_visible(entry))
                                .map(ProjectSummaryData::from)
                                .collect()
                        })
                        .unwrap_or_default()
                }
            }
        }
    });

    let home_data_for_retry = home_data.clone();
    let projects_data_for_retry = projects_data.clone();
    let retry = Callback::new(move |_: ()| {
        home_data_for_retry.refetch();
        projects_data_for_retry.refetch();
    });

    view! {
        <div class="home-page">
            <StaticPageSeo
                title="Maxime Leriche | Portfolio DevOps passionné"
                description="Portfolio de Maxime Leriche : projets, expériences, articles et contact."
                path="/"
            />
            <Suspense fallback=move || {
                view! { <LoadingScreen /> }
            }>
                {move || {
                    match home_data.get() {
                        Some(Some(data)) => {
                            view! {
                                <HomeContent
                                    data=data
                                    projects=projects_data.get().unwrap_or_default()
                                />
                            }
                                .into_any()
                        }
                        Some(None) => view! { <ErrorScreen on_retry=retry /> }.into_any(),
                        None => view! { <></> }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}
