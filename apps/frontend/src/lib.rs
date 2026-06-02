#![recursion_limit = "512"]
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod date_utils;
mod i18n;
mod pages;
mod seo;
mod services;

const _CONTENT_RELOAD_SENTINEL: &str = include_str!("content_reload_sentinel.rs");

use components::navbar::NavBar;
use i18n::provide_i18n;
use pages::{
    about::AboutPage, blog::BlogReferencePage, contact::ContactPage, content::ContentHandlePage,
    docs::DocsReferencePage, home::HomePage, not_found::NotFoundPage, projects::ProjectsPage,
    teapot::TeapotPage, technologies::TechnologiesPage,
};

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
        <Link rel="alternate" type_="application/rss+xml" title="RSS" href="/rss.xml" />
        <Router>
            <NavBar />
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/404") view=NotFoundPage />
                    <Route path=path!("/418") view=TeapotPage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/projects") view=ProjectsPage />
                    <Route path=path!("/technologies") view=TechnologiesPage />
                    <Route path=path!("/blogs/*any") view=ContentHandlePage />
                    <Route path=path!("/blog") view=BlogReferencePage />
                    <Route path=path!("/docs") view=DocsReferencePage />
                    <Route path=path!("/docs/*any") view=ContentHandlePage />
                    <Route path=path!("/contact") view=ContactPage />
                    <Route path=path!("/*any") view=NotFoundPage />
                </Routes>
            </main>
        </Router>
    }
}
