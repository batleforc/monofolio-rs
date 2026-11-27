pub mod translations;

use leptos::prelude::*;

pub use translations::{Language, Translations, EN, FR};

/// Install the i18n context.  Must be called once near the root of the app.
pub fn provide_i18n() {
    // Server and client must both start in French so hydration sees the same DOM;
    // the stored choice is applied right after, in an effect (client-only).
    let lang: RwSignal<Language> = RwSignal::new(Language::Fr);
    provide_context(lang);
    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        let stored = load_lang_from_storage();
        if stored != lang.get_untracked() {
            lang.set(stored);
        }
    });
}

/// Returns the reactive language signal from context.
///
/// # Panics
/// Panics if `provide_i18n` was not called in an ancestor.
pub fn use_language() -> RwSignal<Language> {
    use_context::<RwSignal<Language>>()
        .expect("i18n context not found - call provide_i18n() in an ancestor component")
}

/// Returns a derived signal that tracks the current [`Translations`] set.
pub fn use_translations() -> Signal<&'static Translations> {
    let lang = use_language();
    Signal::derive(move || match lang.get() {
        Language::Fr => &FR,
        Language::En => &EN,
    })
}

/// Toggle the language and persist the choice.
pub fn toggle_language(lang: RwSignal<Language>) {
    let next = match lang.get_untracked() {
        Language::Fr => Language::En,
        Language::En => Language::Fr,
    };
    lang.set(next);
    persist_language(next);
}

// ── Internal helpers ────────────────────────────────────────────────────────

fn persist_language(lang: Language) {
    #[cfg(not(feature = "ssr"))]
    {
        save_lang_to_storage(lang);
    }
    #[cfg(feature = "ssr")]
    {
        let _ = lang;
    }
}

#[cfg(not(feature = "ssr"))]
fn load_lang_from_storage() -> Language {
    let stored = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|ls| ls.get_item("lang").ok().flatten());
    match stored.as_deref() {
        Some("en") => Language::En,
        _ => Language::Fr,
    }
}

#[cfg(not(feature = "ssr"))]
fn save_lang_to_storage(lang: Language) {
    if let Some(ls) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = ls.set_item("lang", lang.as_str());
    }
}
