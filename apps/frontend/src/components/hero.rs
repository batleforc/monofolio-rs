use leptos::prelude::*;

use crate::i18n::{use_language, use_translations, Language};
use crate::pages::home::HomeData;

/// Inline SVG icon by identifier (from `imgUrl` field in home.yaml).
fn social_icon_svg(ico: &str) -> &'static str {
    match ico {
        "ico#github" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 2C6.477 2 2 6.484 2 12.021c0 4.428 2.865 8.185 6.839 9.504.5.092.682-.217.682-.482 0-.237-.009-.868-.013-1.703-2.782.605-3.369-1.342-3.369-1.342-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.339-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337c1.909-1.296 2.747-1.026 2.747-1.026.546 1.378.202 2.397.1 2.65.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.2 22 16.447 22 12.021 22 6.484 17.523 2 12 2z"/></svg>"#,
        "ico#gitea" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M11.9971 0C5.3719 0 0 5.3766 0 12.0058c0 5.3013 3.438 9.8031 8.2066 11.3985.6006.1103.8194-.2607.8194-.5789 0-.2856-.0104-1.0428-.0161-2.047-3.3407.7261-4.046-1.6098-4.046-1.6098-.546-1.3875-1.3327-1.7571-1.3327-1.7571-1.0893-.7444.0824-.7294.0824-.7294 1.2042.085 1.8381 1.2368 1.8381 1.2368 1.0706 1.8345 2.8093 1.3041 3.4945.9968.109-.7749.4193-1.3041.7617-1.6041-2.664-.303-5.467-1.332-5.467-5.9295 0-1.3095.468-2.3808 1.2351-3.2196-.1238-.3027-.5354-1.522.1174-3.173 0 0 1.007-.3223 3.3 1.2303A11.491 11.491 0 0 1 12 5.8035c1.0208.005 2.048.138 3.0084.404 2.291-1.5526 3.297-1.2303 3.297-1.2303.654 1.651.2424 2.8703.1186 3.173.769.8388 1.2338 1.9101 1.2338 3.2196 0 4.6084-2.807 5.6228-5.479 5.9192.4307.3708.8141 1.1025.8141 2.2209 0 1.6029-.0148 2.8948-.0148 3.2871 0 .3209.2166.6952.8237.5778C20.565 21.8029 24 17.3037 24 12.0058 24 5.3766 18.6253 0 11.9971 0z"/></svg>"#,
        "ico#linkedin2" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M4.98 3.5C4.98 4.88 3.87 6 2.5 6S.02 4.88.02 3.5C.02 2.11 1.13 1 2.5 1s2.48 1.11 2.48 2.5zM.5 8.25h4V24h-4V8.25zm6.5 0h3.83v2.15h.05c.53-1 1.83-2.05 3.77-2.05 4.04 0 4.78 2.66 4.78 6.12V24H15.4v-8.44c0-2.01-.04-4.6-2.8-4.6-2.81 0-3.24 2.19-3.24 4.45V24H5.44V8.25z"/></svg>"#,
        _ => r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><circle cx="12" cy="12" r="10"/></svg>"#,
    }
}

/// Hero section: name, animated cover title, description, CTA buttons and social links.
#[component]
pub fn Hero(data: HomeData) -> impl IntoView {
    let lang = use_language();
    let t = use_translations();

    let cover_titles = {
        let fr = data.cover_title.clone();
        let en = data.cover_title_en.clone().unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    let short_desc = {
        let fr = data.short_description.clone();
        let en = data
            .short_description_en
            .clone()
            .unwrap_or_else(|| fr.clone());
        move || match lang.get() {
            Language::Fr => fr.clone(),
            Language::En => en.clone(),
        }
    };

    let name = data.name.clone();
    let cv_url = data.cv_url.clone();
    let social_links = data.url.clone();
    let social_links_primary: Vec<_> =
        social_links.iter().filter(|s| s.primaire).cloned().collect();

    view! {
        <section class="hero">
            <div class="hero-content">
                <p class="hero-greeting">{move || t.get().hero_greeting}</p>

                <h1 class="hero-name">{name.clone()}</h1>

                // Animated cover-title ticker
                <div class="hero-ticker" aria-live="polite">
                    {move || {
                        let titles = cover_titles();
                        let n = titles.len();
                        if n == 0 {
                            return view! { <span class="hero-ticker-item">"..."</span> }.into_any();
                        }
                        let total_s = (n * 3) as f32;
                        let items = titles
                            .into_iter()
                            .enumerate()
                            .map(|(i, title)| {
                                let delay = format!(
                                    "animation-delay:{}s;animation-duration:{}s",
                                    i * 3,
                                    total_s as usize
                                );
                                view! {
                                    <span
                                        class="hero-ticker-item"
                                        style=delay
                                    >
                                        {title}
                                    </span>
                                }
                            })
                            .collect_view();
                        view! { <div class="hero-ticker-inner">{items}</div> }.into_any()
                    }}
                </div>

                <p class="hero-desc">{move || short_desc()}</p>

                <div class="hero-actions">
                    <a
                        href=cv_url.clone()
                        class="btn-primary"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {move || t.get().hero_cta_cv}
                    </a>

                    <div class="hero-social">
                        {social_links_primary
                            .iter()
                            .map(|link| {
                                let url = link.url.clone();
                                let name = link.name.clone();
                                let svg = social_icon_svg(&link.img_url);
                                view! {
                                    <a
                                        href=url
                                        class="social-link"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        title=name
                                        inner_html=svg
                                    ></a>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>

            <div class="hero-visual" aria-hidden="true">
                <div class="hero-blob"></div>
            </div>
        </section>
    }
}
