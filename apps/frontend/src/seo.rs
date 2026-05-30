use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};
use serde_json::json;

use crate::pages::home::HomeData;

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

pub fn build_person_json_ld(data: &HomeData) -> String {
    let mut person = json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": data.name,
        "url": SITE_URL,
        "description": data.short_description_en.as_deref().unwrap_or(&data.short_description),
        "sameAs": data.url.iter().map(|l| l.url.clone()).collect::<Vec<_>>(),
    });

    if let Some(email) = &data.contact_email {
        person["email"] = json!(email);
    }
    if let Some(job) = &data.current_work {
        person["jobTitle"] = json!(job);
    }
    if let Some(location) = &data.contact_location {
        person["address"] = json!({
            "@type": "PostalAddress",
            "addressLocality": location,
        });
    }

    person.to_string()
}

#[component]
pub fn PersonJsonLd(data: HomeData) -> impl IntoView {
    let json_string = build_person_json_ld(&data);
    view! {
        <Script type_="application/ld+json">{json_string}</Script>
    }
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
    use super::{build_person_json_ld, canonical_url};
    use crate::pages::home::{HomeData, SocialLinkData};

    #[test]
    fn canonical_url_handles_root_and_paths() {
        assert_eq!(canonical_url("/"), "https://maxleriche.net");
        assert_eq!(canonical_url("about"), "https://maxleriche.net/about");
        assert_eq!(
            canonical_url("/docs/test"),
            "https://maxleriche.net/docs/test"
        );
    }

    #[test]
    fn person_json_ld_contains_required_fields() {
        let data = HomeData {
            name: "Maxime Leriche".to_string(),
            short_description: "Dev, Ops".to_string(),
            short_description_en: Some("Dev, Ops, Arch".to_string()),
            contact_email: Some("max@maxleriche.net".to_string()),
            current_work: Some("Engineer".to_string()),
            contact_location: Some("Nouvelle-Aquitaine, France".to_string()),
            url: vec![SocialLinkData {
                name: "GitHub".to_string(),
                url: "https://github.com/batleforc".to_string(),
                primaire: true,
                img_url: String::new(),
            }],
            ..Default::default()
        };

        let json_string = build_person_json_ld(&data);
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(parsed["@type"], "Person");
        assert_eq!(parsed["name"], "Maxime Leriche");
        assert_eq!(parsed["description"], "Dev, Ops, Arch");
        assert_eq!(parsed["email"], "max@maxleriche.net");
        assert_eq!(parsed["jobTitle"], "Engineer");
        assert_eq!(
            parsed["address"]["addressLocality"],
            "Nouvelle-Aquitaine, France"
        );
        let same_as = parsed["sameAs"].as_array().unwrap();
        assert_eq!(same_as.len(), 1);
        assert_eq!(same_as[0], "https://github.com/batleforc");
    }
}
