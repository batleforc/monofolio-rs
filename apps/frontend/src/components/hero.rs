use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;
use leptos_icons::Icon;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::use_translations;
use crate::pages::about::resolve_cv_url;
use crate::pages::home::{HomeData, SocialLinkData};

/// Map a home.yaml imgUrl value to the appropriate `IconType`.
enum SocialIconType {
    Builtin(IconType),
    Gitea,
    Bluesky,
    Minia(String),
    Icomoon(String),
}

fn resolve_icomoon_symbol(raw: &str) -> Option<String> {
    let name = raw.strip_prefix("icomoon#")?.trim().to_ascii_lowercase();
    if name.is_empty() {
        None
    } else {
        Some(format!("/assets/icon/symbol-defs.svg#ico-{name}"))
    }
}

fn social_icon_type(img_url: &str) -> SocialIconType {
    match img_url {
        "ico#github" => SocialIconType::Builtin(IconType::Github),
        "ico#linkedin2" => SocialIconType::Builtin(IconType::Linkedin),
        "ico#gitea" => SocialIconType::Gitea,
        "ico#bluesky" => SocialIconType::Bluesky,
        _ if img_url.starts_with("/public/minia/") => SocialIconType::Minia(img_url.to_string()),
        _ => resolve_icomoon_symbol(img_url)
            .map(SocialIconType::Icomoon)
            .unwrap_or(SocialIconType::Builtin(IconType::ExternalLink)),
    }
}

/// A single social icon button rendered with `LeptosIcon`.
#[component]
fn SocialIconButton(link: SocialLinkData) -> impl IntoView {
    let icon = social_icon_type(&link.img_url);
    let btn_class = ButtonClass {
        variant: ButtonVariant::Icon,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <a
            href=link.url
            class=btn_class
            target="_blank"
            rel="noopener noreferrer"
            title=link.name.clone()
            aria-label=link.name
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
                SocialIconType::Bluesky => {
                    view! {
                        <Icon
                            icon=icondata_si::SiBluesky
                            width="1rem"
                            height="1rem"
                            style="color: currentColor;"
                        />
                    }
                        .into_any()
                }
                SocialIconType::Minia(url) => {
                    view! {
                        <img src=url class="w-4 h-4 object-contain" alt="" aria-hidden="true" />
                    }
                        .into_any()
                }
                SocialIconType::Icomoon(href) => {
                    view! {
                        <svg class="w-4 h-4" aria-hidden="true" focusable="false">
                            <use href=href></use>
                        </svg>
                    }
                        .into_any()
                }
            }}
        </a>
    }
}

/// Hero section: name, current role, positioning sentence, CTA buttons and social links.
#[component]
pub fn Hero(data: HomeData) -> impl IntoView {
    let t = use_translations();

    let name = data.name.clone();
    let cta_url = "/projects";
    let current_work = data.current_work.clone();
    let cv_url = (!data.cv_url.trim().is_empty()).then(|| resolve_cv_url(&data.cv_url));
    let cv_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();
    let social_links_primary: Vec<_> = data.url.iter().filter(|s| s.primaire).cloned().collect();

    let primary_btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <section class="relative overflow-hidden min-h-[calc(100svh-3.5rem)] flex items-center cyber-grid-bg before:absolute before:inset-0 before:bg-[radial-gradient(ellipse_70%_60%_at_70%_30%,oklch(from_var(--primary)_l_c_h_/_0.14),transparent_70%)] before:pointer-events-none">
            <div class="relative max-w-5xl mx-auto px-5 py-20 w-full">
                <p class="text-muted-foreground mb-2 font-mono text-sm tracking-widest uppercase">
                    {move || t.get().hero_greeting}
                </p>

                <h1 class="text-5xl sm:text-7xl font-extrabold tracking-tighter leading-[1.05] mb-4 text-foreground glitch">
                    {name.clone()}
                </h1>

                {current_work
                    .map(|work| {
                        view! {
                            <p class="font-mono text-lg font-semibold text-primary mb-5 text-balance">
                                {work}
                            </p>
                        }
                    })}

                <p class="text-xl sm:text-2xl text-foreground max-w-2xl leading-snug text-balance">
                    {move || t.get().hero_pitch}
                </p>

                <div class="flex flex-wrap items-center gap-4 mt-8">
                    <a href=cta_url class=primary_btn_class>
                        <LeptosIcon icon=IconType::ArrowRight class="w-4 h-4" />
                        {move || t.get().hero_cta}
                    </a>
                    {cv_url
                        .map(|href| {
                            view! {
                                <a
                                    href=href
                                    class=cv_btn_class
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    {move || t.get().hero_cv}
                                </a>
                            }
                        })}

                    <div class="flex gap-3 items-center">
                        {social_links_primary
                            .into_iter()
                            .map(|link| view! { <SocialIconButton link=link /> })
                            .collect_view()}
                    </div>
                </div>
            </div>

            // Decorative blob: static neon halo behind the hero copy
            <div
                class="absolute right-[-8rem] top-1/2 -translate-y-1/2 pointer-events-none hidden md:block"
                aria-hidden="true"
            >
                <div class="w-[28rem] h-[28rem] rounded-[50%_45%_55%_48%/48%_55%_45%_52%] bg-primary/[0.07] cyber-glow border border-primary/30"></div>
            </div>
        </section>
    }
}
