use leptos::prelude::*;

use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::HomeData;

/// About / presentation section.
#[component]
pub fn About(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    let presentation = {
        let fr = data.presentation.clone();
        let en = data
            .presentation_en
            .clone()
            .unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    view! {
        <section class="about-section" id="about">
            <div class="section-inner">
                <h2 class="section-title">{move || t.get().about_title}</h2>
                <div class="about-text">
                    {move || {
                        presentation()
                            .lines()
                            .filter(|l| !l.trim().is_empty())
                            .map(|line| view! { <p>{line.to_string()}</p> })
                            .collect_view()
                    }}
                </div>
            </div>
        </section>
    }
}
