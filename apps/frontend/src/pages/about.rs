use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_location;
use tw_merge::IntoTailwindClass;
#[cfg(feature = "ssr")]
use content::HomeConfig;

use crate::components::timeline::Timeline;
use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::i18n::{use_language, Language};
use crate::pages::home::{HomeData, SocialLinkData};
use crate::seo::StaticPageSeo;

#[cfg_attr(feature = "ssr", allow(dead_code))]
async fn load_home_data() -> Option<HomeData> {
    #[cfg(not(feature = "ssr"))]
    {
        let resp = gloo_net::http::Request::get("/api/v1/home")
            .send()
            .await
            .ok()?;
        let text = resp.text().await.ok()?;
        serde_json::from_str::<HomeData>(&text).ok()
    }
    #[cfg(feature = "ssr")]
    {
        None
    }
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

fn resolve_cv_url(raw: &str) -> String {
    if raw.starts_with("http://") || raw.starts_with("https://") {
        raw.to_string()
    } else {
        format!("/public/media/{}", raw.trim_start_matches("media#"))
    }
}

enum SocialIconType {
    Builtin(IconType),
    Gitea,
}

fn social_icon_type(img_url: &str) -> SocialIconType {
    match img_url {
        "ico#github" => SocialIconType::Builtin(IconType::Github),
        "ico#linkedin2" => SocialIconType::Builtin(IconType::Linkedin),
        "ico#gitea" => SocialIconType::Gitea,
        _ => SocialIconType::Builtin(IconType::ExternalLink),
    }
}

#[component]
fn SocialTextLink(link: SocialLinkData) -> impl IntoView {
    let icon = social_icon_type(&link.img_url);
    let name_text = link.name.clone();
    let name_title = link.name.clone();
    let aria_label = link.name.clone();

    view! {
        <a
            href=link.url
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-2 text-primary hover:text-primary/80 underline-offset-2 hover:underline"
            title=name_title
            aria-label=aria_label
        >
            {match icon {
                SocialIconType::Builtin(icon) => {
                    view! { <LeptosIcon icon class="w-4 h-4" /> }.into_any()
                }
                SocialIconType::Gitea => {
                    view! {
                        <Icon
                            icon=icondata_si::SiGitea
                            width="1rem"
                            height="1rem"
                            style="color: currentColor;"
                        />
                    }
                        .into_any()
                }
            }}
            <span>{name_text}</span>
        </a>
    }
}

#[component]
fn MoreAboutContent(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let cv_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();

    let title = move || match lang.get() {
        Language::Fr => "More about me",
        Language::En => "More about me",
    };

    let cv_contact_title = move || match lang.get() {
        Language::Fr => "CV & Contact",
        Language::En => "Resume & Contact",
    };

    let cv_label = move || match lang.get() {
        Language::Fr => "Télécharger mon CV",
        Language::En => "Download my CV",
    };

    let work_label = move || match lang.get() {
        Language::Fr => "Poste actuel",
        Language::En => "Current work",
    };

    let location_label = move || match lang.get() {
        Language::Fr => "Location",
        Language::En => "Location",
    };

    let availability_label = move || match lang.get() {
        Language::Fr => "Disponibilité",
        Language::En => "Availability",
    };

    let links_label = move || match lang.get() {
        Language::Fr => "Liens",
        Language::En => "Links",
    };

    let presentation_fr = data.presentation.clone();
    let presentation_en = data
        .presentation_en
        .clone()
        .unwrap_or_else(|| presentation_fr.clone());

    let availability_fr = data.contact_availability.clone();
    let availability_en = data
        .contact_availability_en
        .clone()
        .or(availability_fr.clone());
    let contact_email = data
        .contact_email
        .clone()
        .unwrap_or_else(|| "max@maxleriche.net".to_string());
    let contact_location = data
        .contact_location
        .clone()
        .unwrap_or_else(|| "Nouvelle-Aquitaine, France".to_string());
    let current_work = data
        .current_work
        .clone()
        .unwrap_or_else(|| "Ingénieur plateforme".to_string());
    let social_links_intro = data.url.clone();
    let social_links_contact = data.url.clone();

    let cv_url = resolve_cv_url(&data.cv_url);
    let timeline_data = data.clone();

    view! {
        <section class="border-b border-border/70 cyber-grid-bg">
            <SectionInner>
                <SectionTitle>{title}</SectionTitle>
                <Card class="p-6 max-w-4xl">
                    <div class="flex flex-col gap-3">
                        {move || {
                            let text = match lang.get() {
                                Language::Fr => presentation_fr.clone(),
                                Language::En => presentation_en.clone(),
                            };
                            text.lines()
                                .filter(|l| !l.trim().is_empty())
                                .map(|line| {
                                    view! {
                                        <p class="leading-7 text-muted-foreground">
                                            {line.to_string()}
                                        </p>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </Card>

                <Card class="p-4 mt-4 max-w-4xl">
                    <p class="text-foreground font-medium mb-2">{links_label}</p>
                    <div class="flex flex-wrap gap-3">
                        {social_links_intro
                            .iter()
                            .map(|link| {
                                view! { <SocialTextLink link=link.clone() /> }
                            })
                            .collect_view()}
                    </div>
                </Card>
            </SectionInner>
        </section>

        <Timeline data=timeline_data />

        <section>
            <SectionInner>
                <SectionTitle>{cv_contact_title}</SectionTitle>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                    <Card class="p-5">
                        <h3 class="font-semibold mb-3">"CV"</h3>
                        <a href=cv_url class=cv_btn_class target="_blank" rel="noopener noreferrer">
                            {cv_label}
                        </a>
                    </Card>

                    <Card class="p-5">
                        <h3 class="font-semibold mb-3">"Contact"</h3>
                        <div class="space-y-2 text-sm text-muted-foreground">
                            <p>
                                <span class="text-foreground font-medium">"Email: "</span>
                                {contact_email.clone()}
                            </p>
                            <p>
                                <span class="text-foreground font-medium">
                                    {move || format!("{}: ", location_label())}
                                </span>
                                {contact_location.clone()}
                            </p>
                            <p>
                                <span class="text-foreground font-medium">
                                    {move || format!("{}: ", work_label())}
                                </span>
                                {current_work.clone()}
                            </p>
                            <p>
                                <span class="text-foreground font-medium">
                                    {move || format!("{}: ", availability_label())}
                                </span>
                                {move || {
                                    match lang.get() {
                                        Language::Fr => availability_fr.clone(),
                                        Language::En => availability_en.clone(),
                                    }
                                        .unwrap_or_else(|| "Open to collaborations".to_string())
                                }}
                            </p>
                            <div class="pt-2">
                                <p class="text-foreground font-medium mb-1">{links_label}</p>
                                <div class="flex flex-wrap gap-3">
                                    {social_links_contact
                                        .iter()
                                        .map(|link| {
                                            view! { <SocialTextLink link=link.clone() /> }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        </div>
                    </Card>
                </div>
            </SectionInner>
        </section>
    }
}

#[component]
pub fn AboutPage() -> impl IntoView {
    let location = use_location();
    #[cfg(feature = "ssr")]
    let home_config = use_context::<HomeConfig>();
    let home_data = Resource::new(
        move || location.pathname.get(),
        {
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
        },
    );

    view! {
        <StaticPageSeo
            title="À propos | Maxime Leriche"
            description="Parcours, expériences et informations de contact de Maxime Leriche."
            path="/about"
        />
        <Suspense fallback=move || {
            view! {
                <div class="min-h-[calc(100svh-3.5rem)] flex items-center justify-center text-muted-foreground">
                    "Loading…"
                </div>
            }
        }>
            {move || {
                match home_data.get().flatten() {
                    Some(data) => view! { <MoreAboutContent data=data /> }.into_any(),
                    None => {
                        view! {
                            <div class="min-h-[calc(100svh-3.5rem)] flex items-center justify-center text-muted-foreground">
                                "Unable to load data for this page."
                            </div>
                        }
                        .into_any()
                    }
                }
            }}
        </Suspense>
    }
}
