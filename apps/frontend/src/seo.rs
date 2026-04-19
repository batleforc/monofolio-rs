use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

pub const SITE_URL: &str = "https://maxleriche.net";
pub const DEFAULT_OG_IMAGE: &str = "https://maxleriche.net/assets/apple-touch-icon.png";

pub fn canonical_url(path: &str) -> String {
    if path.is_empty() || path == "/" {
        return SITE_URL.to_string();
    }
    format!("{SITE_URL}/{}", path.trim_start_matches('/'))
}

pub fn og_minia_url(path: &str) -> String {
    let path = if path.is_empty() || path == "/" {
        "home".to_string()
    } else {
        path.trim_start_matches('/').replace("/", "_")
    };
    format!("{SITE_URL}/public/minia/{}.webp", path)
}

#[component]
pub fn StaticPageSeo(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] path: String,
) -> impl IntoView {
    let canonical = canonical_url(&path);
    view! {
        <Title text=title.clone() />
        <Meta name="description" content=description.clone() />
        <Link rel="canonical" href=canonical.clone() />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=title.clone() />
        <Meta property="og:description" content=description.clone() />
        <Meta property="og:url" content=canonical.clone() />
        <Meta property="og:image" content=og_minia_url(&path) />
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=title.clone() />
        <Meta name="twitter:description" content=description.clone() />
        <Meta name="twitter:image" content=og_minia_url(&path) />
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_url;

    #[test]
    fn canonical_url_handles_root_and_paths() {
        assert_eq!(canonical_url("/"), "https://maxleriche.net");
        assert_eq!(canonical_url("about"), "https://maxleriche.net/about");
        assert_eq!(
            canonical_url("/docs/test"),
            "https://maxleriche.net/docs/test"
        );
    }
}
