use icons::common::icon_type::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;

use crate::components::ui::{Card, SectionInner, SectionTitle};
use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::{HistoryEntryData, HomeData};

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
        Language::En => entry
            .title_en
            .as_deref()
            .unwrap_or(entry.title.as_str()),
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

    // Sort by weight ascending (chronological order)
    let mut entries = data.history.clone();
    entries.sort_by_key(|e| e.weight);

    view! {
        <section class="timeline-section" id="timeline">
            <SectionInner>
                <SectionTitle>{move || t.get().timeline_title}</SectionTitle>
                <ol class="relative ps-10 before:absolute before:start-3 before:top-2 before:bottom-2 before:w-0.5 before:bg-gradient-to-b before:from-primary before:to-border before:rounded-sm">
                    {entries
                        .into_iter()
                        .map(|entry| {
                            let icon = timeline_icon_type(&entry.ico_url);
                            let date = entry.date.clone();
                            let lieux = entry.lieux.clone();
                            let entry_c = entry.clone();
                            view! {
                                <li class="relative pb-10 last:pb-0 flex gap-5">
                                    // Icon bubble on the timeline rail
                                    <div class="absolute start-[-0.6875rem] top-0.5 w-7 h-7 rounded-full border-2 border-primary bg-card flex items-center justify-center text-primary z-10">
                                        <LeptosIcon icon class="w-3.5 h-3.5 stroke-current" />
                                    </div>
                                    // Card body
                                    <Card class="flex-1 p-4 hover:border-primary transition-colors">
                                        <span class="text-[0.7rem] uppercase tracking-wider font-semibold text-primary">
                                            {date}
                                        </span>
                                        <h3 class="text-base font-bold mt-1 mb-0.5">
                                            {move || entry_title(&entry_c, lang.get()).to_string()}
                                        </h3>
                                        <p class="text-sm text-muted-foreground mb-2">{lieux}</p>
                                        <div class="flex flex-col gap-1">
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
