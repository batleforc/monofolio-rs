use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;

use crate::components::ui::{Card, SectionInner, SectionTitle};
use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::{HistoryEntryData, HomeData};

/// Convert a `media#filename.ext` value to a served URL.
fn media_url(raw: &str) -> Option<String> {
    raw.strip_prefix("media#")
        .map(|filename| format!("/public/media/{}", filename))
}

/// Map a home.yaml `icoUrl` value to the appropriate `IconType`.
fn timeline_icon_type(ico: &str) -> IconType {
    match ico {
        "ico#school" => IconType::GraduationCap,
        "ico#work" => IconType::Briefcase,
        "ico#handyman" => IconType::Wrench,
        _ => IconType::Circle,
    }
}

fn entry_title<'a>(entry: &'a HistoryEntryData, lang: Language) -> &'a str {
    match lang {
        Language::En => entry.title_en.as_deref().unwrap_or(entry.title.as_str()),
        Language::Fr => entry.title.as_str(),
    }
}

fn entry_description<'a>(entry: &'a HistoryEntryData, lang: Language) -> &'a str {
    match lang {
        Language::En => entry
            .description_en
            .as_deref()
            .unwrap_or(entry.description.as_str()),
        Language::Fr => entry.description.as_str(),
    }
}

/// Career / education timeline section.
#[component]
pub fn Timeline(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    // Sort by weight descending (latest first)
    let mut entries = data.history.clone();
    entries.sort_by(|a, b| b.weight.cmp(&a.weight));

    view! {
        <section class="timeline-section" id="timeline">
            <SectionInner>
                <SectionTitle>{move || t.get().timeline_title}</SectionTitle>
                <ol class="relative ps-10 before:absolute before:start-3 before:top-2 before:bottom-2 before:w-0.5 before:bg-gradient-to-b before:from-primary before:via-border before:to-accent before:rounded-sm">
                    {entries
                        .into_iter()
                        .map(|entry| {
                            let icon = timeline_icon_type(&entry.ico_url);
                            let ico_url_norm = entry.ico_url.trim().to_ascii_lowercase();
                            let title_norm = entry.title.trim().to_ascii_lowercase();
                            let is_pacman = ico_url_norm.contains("pacman")
                                || title_norm.contains("take over the world");
                            let date = entry.date.clone();
                            let lieux = entry.lieux.clone();
                            let img = media_url(&entry.img_url);
                            let links = entry.url.clone();
                            let entry_c = entry.clone();
                            view! {
                                <li class="relative pb-10 last:pb-0 flex gap-5">
                                    // Icon bubble on the timeline rail — cyan neon (cyber)
                                    <div class="absolute start-[-0.6875rem] top-0.5 w-7 h-7 rounded border-2 border-primary bg-card flex items-center justify-center text-primary z-10 cyber-glow">
                                        {if is_pacman {
                                            view! {
                                                <svg
                                                    class="w-4 h-4 pacman-icon"
                                                    viewBox="0 0 32 24"
                                                    fill="none"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    aria-hidden="true"
                                                >
                                                    <circle
                                                        class="pacman-pellet pellet-1"
                                                        cx="24"
                                                        cy="12"
                                                        r="1.5"
                                                        fill="var(--accent)"
                                                    >
                                                        <animate
                                                            attributeName="cx"
                                                            values="24;12.8"
                                                            dur="0.9s"
                                                            repeatCount="indefinite"
                                                        />
                                                        <animate
                                                            attributeName="opacity"
                                                            values="1;1;0"
                                                            dur="0.9s"
                                                            repeatCount="indefinite"
                                                        />
                                                    </circle>
                                                    <circle
                                                        class="pacman-pellet pellet-2"
                                                        cx="28"
                                                        cy="12"
                                                        r="1.2"
                                                        fill="var(--accent)"
                                                    >
                                                        <animate
                                                            attributeName="cx"
                                                            values="28;12.8"
                                                            dur="0.9s"
                                                            begin="0.3s"
                                                            repeatCount="indefinite"
                                                        />
                                                        <animate
                                                            attributeName="opacity"
                                                            values="1;1;0"
                                                            dur="0.9s"
                                                            begin="0.3s"
                                                            repeatCount="indefinite"
                                                        />
                                                    </circle>
                                                    <path
                                                        class="pacman-body"
                                                        d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2Z"
                                                        fill="currentColor"
                                                    />
                                                    <path
                                                        class="pacman-mouth"
                                                        d="M12 12L22 7.2L22 16.8Z"
                                                        fill="var(--background)"
                                                    >
                                                        <animate
                                                            attributeName="d"
                                                            dur="0.42s"
                                                            repeatCount="indefinite"
                                                            values="M12 12L22 7.2L22 16.8Z;M12 12L22 10.2L22 13.8Z;M12 12L22 7.2L22 16.8Z"
                                                        />
                                                    </path>
                                                    <circle
                                                        cx="13.2"
                                                        cy="8.4"
                                                        r="1.25"
                                                        fill="var(--background)"
                                                    />
                                                </svg>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <LeptosIcon icon class="w-3.5 h-3.5 stroke-current" />
                                            }
                                                .into_any()
                                        }}
                                    </div>
                                    // Card body
                                    <Card class="flex-1 p-4">
                                        <div class="flex items-start gap-3">
                                            {img.map(|src| view! {
                                                <img
                                                    src=src
                                                    alt=""
                                                    class="w-10 h-10 rounded object-contain shrink-0 bg-muted p-0.5"
                                                />
                                            })}
                                            <div class="flex-1 min-w-0">
                                                <span class="text-[0.7rem] uppercase tracking-widest font-mono font-semibold text-accent">
                                                    {date}
                                                </span>
                                                <h3 class="text-base font-bold mt-1 mb-0.5">
                                                    {move || entry_title(&entry_c, lang.get()).to_string()}
                                                </h3>
                                                <p class="text-sm text-muted-foreground mb-2">{lieux}</p>
                                            </div>
                                        </div>
                                        <div class="flex flex-col gap-1 mt-2">
                                            {move || {
                                                entry_description(&entry, lang.get())
                                                    .lines()
                                                    .filter(|l| !l.trim().is_empty())
                                                    .map(|line| {
                                                        view! {
                                                            <p class="text-sm text-muted-foreground leading-relaxed">
                                                                {line.to_string()}
                                                            </p>
                                                        }
                                                    })
                                                    .collect_view()
                                            }}
                                        </div>
                                        {if !links.is_empty() {
                                            view! {
                                                <div class="flex flex-wrap gap-2 mt-3">
                                                    {links.into_iter().map(|link| view! {
                                                        <a
                                                            href=link.url
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                            class="inline-flex items-center gap-1 text-xs font-medium text-primary hover:text-primary/80 underline-offset-2 hover:underline transition-colors"
                                                        >
                                                            <LeptosIcon icon=IconType::ExternalLink class="w-3 h-3 stroke-current" />
                                                            {link.name}
                                                        </a>
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <></> }.into_any()
                                        }}
                                    </Card>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </SectionInner>
        </section>
    }
}
