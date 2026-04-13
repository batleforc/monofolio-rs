use leptos::prelude::*;

/// Custom 404 page.
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 – Page Not Found"</h1>
            <p><a href="/">"Back to home"</a></p>
        </div>
    }
}
