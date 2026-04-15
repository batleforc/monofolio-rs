use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::components::{about::About, hero::Hero, timeline::Timeline};
use crate::i18n::use_translations;

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
    pub url: Vec<SocialLinkData>,
    pub history: Vec<HistoryEntryData>,
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

/// Renders the full home page content once data is available.
#[component]
fn HomeContent(data: HomeData) -> impl IntoView {
    view! {
        <Hero data=data.clone() />
        <About data=data.clone() />
        <Timeline data=data />
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
    let loading = RwSignal::new(true);
    let fetch_error = RwSignal::new(false);

    let do_fetch = move || {
        loading.set(true);
        fetch_error.set(false);
        #[cfg(not(feature = "ssr"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
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
            <Show when=move || !loading.get() && !fetch_error.get()>
                {move || home_data.get().map(|data| view! { <HomeContent data=data /> })}
            </Show>
        </div>
    }
}
