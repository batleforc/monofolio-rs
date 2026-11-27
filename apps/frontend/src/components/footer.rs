use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::use_translations;

/// Site-wide footer: closing line and the contact action, on every route.
#[component]
pub fn SiteFooter() -> impl IntoView {
    let t = use_translations();
    let contact_btn_class = ButtonClass {
        variant: ButtonVariant::Outline,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <footer class="border-t border-border/70">
            <div class="max-w-5xl mx-auto px-5 py-10 flex flex-col sm:flex-row gap-4 sm:items-center sm:justify-between">
                <div class="flex flex-col gap-1">
                    <p class="text-sm text-muted-foreground">
                        {move || t.get().home_footer_tagline}
                    </p>
                    <a
                        href="/404"
                        class="self-start font-mono text-xs text-muted-foreground underline-offset-4 hover:text-primary hover:underline"
                    >
                        {move || t.get().footer_lost}
                    </a>
                </div>
                <a href="/contact" class=contact_btn_class>
                    {move || t.get().home_footer_contact_cta}
                </a>
            </div>
        </footer>
    }
}
