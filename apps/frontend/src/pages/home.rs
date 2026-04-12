use leptos::prelude::*;

/// Home page – shows the owner's name and a short presentation.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home">
            <h1>"Welcome to Monofolio"</h1>
            <p>"Max pour vous servir."</p>
        </div>
    }
}
