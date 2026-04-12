use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/monofolio.css" />
        <Title text="Monofolio" />
        <Meta name="description" content="Monofolio – my personal portfolio and blog." />

        <Router>
            <main>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("about") view=AboutPage />
                    <Route path=StaticSegment("projects") view=ProjectsPage />
                    <Route path=StaticSegment("blog") view=BlogPage />
                </Routes>
            </main>
        </Router>
    }
}

/// Home page
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home – Monofolio" />
        <section class="home">
            <h1>"Welcome to Monofolio"</h1>
            <p>"My personal portfolio and blog."</p>
        </section>
    }
}

/// About page
#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About – Monofolio" />
        <section class="about">
            <h1>"About Me"</h1>
            <p>"More about me coming soon."</p>
        </section>
    }
}

/// Projects page
#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="Projects – Monofolio" />
        <section class="projects">
            <h1>"Projects"</h1>
            <p>"My projects coming soon."</p>
        </section>
    }
}

/// Blog page
#[component]
fn BlogPage() -> impl IntoView {
    view! {
        <Title text="Blog – Monofolio" />
        <section class="blog">
            <h1>"Blog"</h1>
            <p>"Blog posts coming soon."</p>
        </section>
    }
}

/// 404 page
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="404 – Not Found" />
        <section class="not-found">
            <h1>"404"</h1>
            <p>"Page not found."</p>
            <a href="/">"Go home"</a>
        </section>
    }
}
