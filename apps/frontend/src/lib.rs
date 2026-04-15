#![recursion_limit = "512"]
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod i18n;
mod pages;

use components::navbar::NavBar;
use i18n::provide_i18n;
use pages::{home::HomePage, not_found::NotFoundPage};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

/// Root application component.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_i18n();

    view! {
        <Title text="Maxime Leriche - Portfolio" />
        <Meta name="description" content="Maxime Leriche's portfolio website." />
        <Meta name="apple-mobile-web-app-title" content="Maxime Leriche Portfolio" />
        <Router>
            <NavBar />
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route
                        path=path!("/about")
                        view=|| view! { <p>"About page - coming soon."</p> }
                    />
                    <Route
                        path=path!("/blog/*any")
                        view=|| view! { <p>"Blog page - coming soon."</p> }
                    />
                </Routes>
            </main>
        </Router>
    }
}
