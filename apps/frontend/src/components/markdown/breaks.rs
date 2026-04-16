use leptos::prelude::*;

pub fn render_hard_break() -> impl IntoView {
    view! { <br /> }
}

pub fn render_soft_break() -> impl IntoView {
    view! { {" "} }
}
