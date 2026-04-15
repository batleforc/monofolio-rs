use leptos::prelude::*;

use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::{HistoryEntryData, HomeData};

fn timeline_icon_svg(ico: &str) -> &'static str {
    match ico {
        "ico#school" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 3L1 9l11 6 9-4.91V17h2V9L12 3zM5 13.18v4L12 21l7-3.82v-4L12 17l-7-3.82z"/></svg>"#,
        "ico#work" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M20 7h-4V5c0-1.1-.9-2-2-2h-4C8.9 3 8 3.9 8 5v2H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zm-8-2h4v2h-4V5zM4 20V9h16v11H4z"/></svg>"#,
        "ico#handyman" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M21.67 18.17l-5.3-5.3a7.16 7.16 0 0 0 .65-6.62C15.95 3.58 13.14 2 10.09 2c-.99 0-1.95.18-2.85.51L11 6.27 6.27 11l-3.76-3.77A7.037 7.037 0 0 0 2 10.09c0 3.05 1.58 5.86 4.25 6.93a7.16 7.16 0 0 0 6.62-.65l5.3 5.3a1.5 1.5 0 0 0 2.13 0l1.37-1.37c.57-.57.57-1.57 0-2.13z"/></svg>"#,
        _ => r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><circle cx="12" cy="12" r="6"/></svg>"#,
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
            <div class="section-inner">
                <h2 class="section-title">{move || t.get().timeline_title}</h2>
                <ol class="timeline-list">
                    {entries
                        .into_iter()
                        .map(|entry| {
                            let icon = timeline_icon_svg(&entry.ico_url);
                            let date = entry.date.clone();
                            let lieux = entry.lieux.clone();
                            let entry_c = entry.clone();
                            view! {
                                <li class="timeline-item">
                                    <div class="timeline-icon" inner_html=icon></div>
                                    <div class="timeline-body">
                                        <span class="timeline-date">{date}</span>
                                        <h3 class="timeline-title">
                                            {move || entry_title(&entry_c, lang.get()).to_string()}
                                        </h3>
                                        <p class="timeline-lieux">{lieux}</p>
                                        <div class="timeline-desc">
                                            {move || {
                                                entry_description(&entry, lang.get())
                                                    .lines()
                                                    .filter(|l| !l.trim().is_empty())
                                                    .map(|line| view! { <p>{line.to_string()}</p> })
                                                    .collect_view()
                                            }}
                                        </div>
                                    </div>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </div>
        </section>
    }
}
