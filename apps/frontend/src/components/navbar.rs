use leptos::prelude::*;

use crate::i18n::{toggle_language, use_language, use_translations};

/// Top navigation bar with language toggle.
#[component]
pub fn NavBar() -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    view! {
        <header class="navbar">
            <div class="navbar-inner">
                <a href="/" class="navbar-logo" aria-label="Home">
                    <span class="navbar-logo-text">"Max."</span>
                </a>

                <nav class="navbar-links" aria-label="Main navigation">
                    <a href="/" class="nav-link">
                        {move || t.get().nav_home}
                    </a>
                    <a href="/about" class="nav-link">
                        {move || t.get().nav_about}
                    </a>
                </nav>

                <button
                    class="lang-toggle"
                    aria-label="Toggle language"
                    on:click=move |_| toggle_language(lang)
                >
                    {move || lang.get().toggle_label()}
                </button>
            </div>
        </header>
    }
}
