use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::{HomeData, SocialLinkData};

/// Map a home.yaml imgUrl value to the appropriate `IconType`.
fn social_icon_type(img_url: &str) -> IconType {
    match img_url {
        "ico#github" => IconType::Github,
        "ico#linkedin2" => IconType::Linkedin,
        _ => IconType::ExternalLink,
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
            <LeptosIcon icon class="w-4 h-4" />
        </a>
    }
}

/// Hero section: name, animated cover title, description, CTA buttons and social links.
#[component]
pub fn Hero(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    let cover_titles = {
        let fr = data.cover_title.clone();
        let en = data.cover_title_en.clone().unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    let short_desc = {
        let fr = data.short_description.clone();
        let en = data
            .short_description_en
            .clone()
            .unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    let name = data.name.clone();
    let cv_url = data.cv_url.clone();
    let social_links_primary: Vec<_> = data.url.iter().filter(|s| s.primaire).cloned().collect();

    let primary_btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <section class="relative overflow-hidden min-h-[calc(100svh-3.5rem)] flex items-center before:absolute before:inset-0 before:bg-[radial-gradient(ellipse_70%_60%_at_70%_30%,oklch(from_var(--primary)_l_c_h_/_0.12),transparent_70%)] before:pointer-events-none">
            <div class="relative max-w-5xl mx-auto px-5 py-20 w-full">
                <p class="text-muted-foreground mb-2">{move || t.get().hero_greeting}</p>

                <h1 class="text-5xl sm:text-7xl font-extrabold tracking-tighter leading-[1.05] mb-4 bg-gradient-to-br from-foreground to-primary bg-clip-text text-transparent">
                    {name.clone()}
                </h1>

                // Animated cover-title ticker
                <div class="relative h-8 overflow-hidden mb-5" aria-live="polite">
                    {move || {
                        let titles = cover_titles();
                        let n = titles.len();
                        if n == 0 {
                            return view! { <span class="text-primary font-semibold">"…"</span> }
                                .into_any();
                        }
                        let total_s = n * 3;
                        let items = titles
                            .into_iter()
                            .enumerate()
                            .map(|(i, title)| {
                                let delay = format!(
                                    "animation-delay:{}s;animation-duration:{}s",
                                    i * 3,
                                    total_s
                                );
                                view! {
                                    <span
                                        class="hero-ticker-item absolute left-0 top-0 text-lg font-semibold text-primary opacity-0 whitespace-nowrap"
                                        style=delay
                                    >
                                        {title}
                                    </span>
                                }
                            })
                            .collect_view();
                        view! { <div class="relative h-full">{items}</div> }.into_any()
                    }}
                </div>

                <p class="text-muted-foreground text-base max-w-lg mb-8 leading-relaxed">
                    {move || short_desc()}
                </p>

                <div class="flex flex-wrap items-center gap-4">
                    <a
                        href=cv_url.clone()
                        class=primary_btn_class
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <LeptosIcon icon=IconType::Download class="w-4 h-4" />
                        {move || t.get().hero_cta_cv}
                    </a>

                    <div class="flex gap-3 items-center">
                        {social_links_primary
                            .into_iter()
                            .map(|link| view! { <SocialIconButton link=link /> })
                            .collect_view()}
                    </div>
                </div>
            </div>

            // Decorative blob
            <div
                class="absolute right-[-8rem] top-1/2 -translate-y-1/2 pointer-events-none hidden md:block"
                aria-hidden="true"
            >
                <div class="hero-blob w-[28rem] h-[28rem] rounded-[50%_45%_55%_48%/48%_55%_45%_52%] bg-primary/[0.08]"></div>
            </div>
        </section>
    }
}
