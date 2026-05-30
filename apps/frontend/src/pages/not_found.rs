use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::dino_game::DinoGame;
use crate::components::ui::{
    ButtonClass, ButtonSize, ButtonVariant, Card, SectionInner, SectionTitle,
};
use crate::i18n::{use_language, Language};

/// Increment the localStorage 404 counter and return the new count.
/// Returns 0 on SSR or if localStorage is unavailable.
fn increment_not_found_counter() -> u64 {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let count = storage
                    .get_item("not_found_visit_count")
                    .ok()
                    .flatten()
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0)
                    + 1;
                let _ = storage.set_item("not_found_visit_count", &count.to_string());
                return count;
            }
        }
    }
    0
}

/// Custom 404 page. Shows a cyberpunk mini-game on every 5th visit.
#[component]
pub fn NotFoundPage() -> impl IntoView {
    let lang = use_language();

    // — Normal 404 labels —
    let title = move || match lang.get() {
        Language::Fr => "404 - Page introuvable",
        Language::En => "404 - Page not found",
    };
    let description = move || match lang.get() {
        Language::Fr => "La page demandée n'existe pas (ou plus).",
        Language::En => "The page you requested does not exist (or no longer exists).",
    };

    // — Game 404 labels —
    let game_title = move || match lang.get() {
        Language::Fr => "404 — Joue pendant que tu es perdu",
        Language::En => "404 — Play while you're lost",
    };
    let game_description = move || match lang.get() {
        Language::Fr => "La page n'existe pas, mais le jeu, oui.",
        Language::En => "The page doesn't exist, but the game does.",
    };

    // — Shared button labels —
    let home_label = move || match lang.get() {
        Language::Fr => "Retour à l'accueil",
        Language::En => "Back to home",
    };
    let teapot_label = move || match lang.get() {
        Language::Fr => "Voir la page 418",
        Language::En => "Open the 418 page",
    };

    // ButtonClass is Copy so both branches can capture it without issue
    let primary_btn = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    };
    let ghost_btn = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    };

    // show_game starts false (SSR-safe); an Effect sets it after hydration.
    let show_game = RwSignal::new(false);

    Effect::new(move |_| {
        let count = increment_not_found_counter();
        if count > 0 && count % 5 == 0 {
            show_game.set(true);
        }
    });

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <SectionInner>
                <Show
                    when=move || show_game.get()
                    fallback=move || {
                        view! {
                            // ---- Normal 404 view ----
                            <div class="max-w-2xl mx-auto pt-12">
                                <SectionTitle>{title}</SectionTitle>
                                <Card class="p-6 sm:p-8">
                                    <p class="text-muted-foreground mb-6">{description}</p>
                                    <div class="flex flex-wrap items-center gap-3">
                                        <a href="/" class=primary_btn.to_class()>
                                            {home_label}
                                        </a>
                                        <a href="/418" class=ghost_btn.to_class()>
                                            {teapot_label}
                                        </a>
                                    </div>
                                </Card>
                            </div>
                        }
                    }
                >
                    // ---- Game 404 view (every 5th visit) ----
                    <div class="max-w-[700px] mx-auto pt-12">
                        <SectionTitle>{game_title}</SectionTitle>
                        <p class="text-muted-foreground mb-4 font-mono text-sm">
                            {game_description}
                        </p>
                        <DinoGame />
                        <div class="mt-6 flex flex-wrap items-center gap-3">
                            <a href="/" class=primary_btn.to_class()>
                                {home_label}
                            </a>
                            <a href="/418" class=ghost_btn.to_class()>
                                {teapot_label}
                            </a>
                        </div>
                    </div>
                </Show>
            </SectionInner>
        </section>
    }
}

