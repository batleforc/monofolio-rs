use leptos::prelude::*;

use crate::components::ui::{SectionInner, SectionTitle};
use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::HomeData;

/// About / presentation section.
#[component]
pub fn About(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    let presentation = {
        let fr = data.presentation.clone();
        let en = data.presentation_en.clone().unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    view! {
        <section class="border-t border-border" id="about">
            <SectionInner>
                <SectionTitle>{move || t.get().about_title}</SectionTitle>
                <div class="flex flex-col gap-3 max-w-[65ch]">
                        {move || {
                            presentation()
                                .lines()
                                .filter(|l| !l.trim().is_empty())
                                .map(|line| {
                                    view! {
                                        <p class="leading-7 text-foreground/85">
                                            {line.to_string()}
                                        </p>
                                    }
                                })
                                .collect_view()
                        }}
                </div>
            </SectionInner>
        </section>
    }
}
