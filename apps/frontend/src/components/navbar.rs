use leptos::prelude::*;
use tw_merge::{tw_merge, IntoTailwindClass};

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::{toggle_language, use_language, use_translations};

/// Top navigation bar with language toggle.
#[component]
pub fn NavBar() -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    let lang_btn_class = tw_merge!(
        ButtonClass {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
        }
        .to_class(),
        "border border-border ml-auto font-mono text-xs tracking-widest"
    );

    view! {
        <header class="sticky top-0 z-50 border-b border-border backdrop-blur-md bg-background/90 cyber-grid-bg">
            <div class="max-w-5xl mx-auto px-5 h-14 flex items-center gap-8">
                <a
                    href="/"
                    class="font-mono font-bold text-xl text-primary tracking-tight cyber-text-glow"
                    aria-label="Home"
                >
                    "Max."
                </a>

                <nav class="flex gap-6 flex-1" aria-label="Main navigation">
                    <a
                        href="/"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                    >
                        {move || t.get().nav_home}
                    </a>
                    <a
                        href="/contact"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                    >
                        {move || t.get().nav_contact}
                    </a>
                    <a
                        href="/about"
                        class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                    >
                        {move || t.get().nav_about}
                    </a>

                </nav>

                <button
                    class=lang_btn_class
                    aria-label="Toggle language"
                    on:click=move |_| toggle_language(lang)
                >
                    {move || lang.get().toggle_label()}
                </button>
            </div>
        </header>
    }
}
