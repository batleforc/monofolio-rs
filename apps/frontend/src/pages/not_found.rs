use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::i18n::{use_language, Language};

/// Custom 404 page.
#[component]
pub fn NotFoundPage() -> impl IntoView {
    let lang = use_language();

    let title = move || match lang.get() {
        Language::Fr => "404 - Page introuvable",
        Language::En => "404 - Page not found",
    };

    let description = move || match lang.get() {
        Language::Fr => "La page demandée n'existe pas (ou plus).",
        Language::En => "The page you requested does not exist (or no longer exists).",
    };

    let home_label = move || match lang.get() {
        Language::Fr => "Retour à l'accueil",
        Language::En => "Back to home",
    };

    let teapot_label = move || match lang.get() {
        Language::Fr => "Voir la page 418",
        Language::En => "Open the 418 page",
    };

    let primary_btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
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
                    <SectionTitle>{title}</SectionTitle>
                    <Card class="p-6 sm:p-8">
                        <p class="text-muted-foreground mb-6">{description}</p>
                        <div class="flex flex-wrap items-center gap-3">
                            <a href="/" class=primary_btn_class>
                                {home_label}
                            </a>
                            <a href="/418" class=ghost_btn_class>
                                {teapot_label}
                            </a>
                        </div>
                    </Card>
                </div>
            </SectionInner>
        </section>
    }
}
