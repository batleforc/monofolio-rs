use leptos::prelude::*;

use crate::components::ui::{Card, SectionInner, SectionTitle};
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
        <section
            class="border-y border-border bg-card"
            id="about"
        >
            <SectionInner>
                <SectionTitle>{move || t.get().about_title}</SectionTitle>
                <Card class="p-6 max-w-3xl">
                    <div class="flex flex-col gap-3">
                        {move || {
                            presentation()
                                .lines()
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
            </SectionInner>
        </section>
    }
}
