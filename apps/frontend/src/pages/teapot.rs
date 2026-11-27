use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, PageTitle, SectionInner,
};
use crate::i18n::{use_language, Language};

/// Custom 418 page (I'm a teapot).
#[component]
pub fn TeapotPage() -> impl IntoView {
    let lang = use_language();

    let title = move || match lang.get() {
        Language::Fr => "418 - Je suis une théière",
        Language::En => "418 - I'm a teapot",
    };

    let description = move || match lang.get() {
        Language::Fr => "Cette page refuse de préparer du café. Elle préfère clairement le thé.",
        Language::En => "This page refuses to brew coffee. It clearly prefers tea.",
    };

    let home_label = move || match lang.get() {
        Language::Fr => "Retour à l'accueil",
        Language::En => "Back to home",
    };

    let not_found_label = move || match lang.get() {
        Language::Fr => "Aller vers 404",
        Language::En => "Go to 404",
    };

    let outline_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();

    let ghost_btn_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <div class="max-w-2xl mx-auto pt-12">
                    <PageTitle>{title}</PageTitle>
                    <Card class="p-6 sm:p-8">
                        <p class="text-6xl leading-none mb-4" aria-hidden="true">
                            "(  )"
                        </p>
                        <p class="text-muted-foreground mb-6">{description}</p>
                        <div class="flex flex-wrap items-center gap-3">
                            <a href="/" class=outline_btn_class>
                                {home_label}
                            </a>
                            <a href="/404" class=ghost_btn_class>
                                {not_found_label}
                            </a>
                        </div>
                    </Card>
                </div>
            </SectionInner>
        </section>
    }
}
