use leptos::prelude::*;
use tw_merge::IntoTailwindClass;

use crate::components::ui::{ButtonClass, ButtonSize, ButtonVariant};
use crate::i18n::use_translations;

#[component]
pub fn LoadingScreen() -> impl IntoView {
    let t = use_translations();

    view! {
        <div
            class="min-h-[calc(100svh-3.5rem)] flex flex-col items-center justify-center gap-5 text-muted-foreground"
            role="status"
            aria-live="polite"
        >
            <div
                class="w-10 h-10 border-[3px] border-border border-t-primary rounded-full animate-spin"
                aria-hidden="true"
            ></div>
            <p class="text-sm">{move || t.get().loading}</p>
        </div>
    }
}

#[component]
pub fn ErrorScreen(#[prop(into)] on_retry: Callback<()>) -> impl IntoView {
    let t = use_translations();
    let btn_class = ButtonClass {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Default,
    }
    .to_class();

    view! {
        <div class="min-h-[calc(100svh-3.5rem)] flex flex-col items-center justify-center gap-4 text-muted-foreground">
            <p class="text-base font-medium text-foreground">{move || t.get().error_loading}</p>
            <button type="button" class=btn_class on:click=move |_| on_retry.run(())>
                {move || t.get().error_retry}
            </button>
        </div>
    }
}
