use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod pages;

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

    view! {
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Title text="Monofolio" />

        <Router>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=|| view! { <p>"About page - coming soon."</p> } />
                </Routes>
            </main>
        </Router>
    }
}
