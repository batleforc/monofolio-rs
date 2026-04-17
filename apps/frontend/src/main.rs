#![recursion_limit = "512"]
#[cfg(feature = "ssr")]
use actix_web::{get, web::Data, HttpResponse, Responder};
#[cfg(feature = "ssr")]
use content::ContentDatabase;

#[cfg(feature = "ssr")]
fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(feature = "ssr")]
fn build_sitemap_xml(database: &ContentDatabase) -> String {
    let mut urls: Vec<String> = vec![
        "https://maxleriche.net/".to_string(),
        "https://maxleriche.net/about".to_string(),
        "https://maxleriche.net/projects".to_string(),
        "https://maxleriche.net/blog".to_string(),
        "https://maxleriche.net/docs".to_string(),
        "https://maxleriche.net/contact".to_string(),
    ];
    urls.extend(database.entries.iter().map(|entry| {
        format!(
            "https://maxleriche.net/{}",
            entry.handle.trim_start_matches('/')
        )
    }));
    urls.sort();
    urls.dedup();

    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );
    for url in urls {
        xml.push_str("<url><loc>");
        xml.push_str(&escape_xml(&url));
        xml.push_str("</loc></url>");
    }
    xml.push_str("</urlset>");
    xml
}

#[cfg(feature = "ssr")]
#[get("/sitemap.xml")]
async fn get_sitemap(database: Data<ContentDatabase>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(build_sitemap_xml(database.get_ref()))
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_files::Files;
    use actix_web::{middleware::Compress, web::Data, App, HttpServer};
    use anyhow::Context as _;
    use api::{api_v1_scope, public::{public_scope, PublicRoot}, rss::RssFeed, ApiDoc};
    use content::{ContentDatabase, HomeConfig};
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::{Link, Meta};
    use std::net::Ipv4Addr;
    use trace::{shutdown_tracing, start_tracing, Context};
    use tracing_actix_web::TracingLogger;
    use utoipa::OpenApi;
    use utoipa_actix_web::AppExt;
    use utoipa_scalar::{Scalar, Servable};

    let tracing_output = start_tracing(&Context {
        pod_name: std::env::var("POD_NAME").unwrap_or_else(|_| "not_a_pod".to_string()),
        service_name: "monofolio".to_string(),
    });

    let content_base_path =
        std::env::var("CONTENT_DB_PATH").unwrap_or_else(|_| "target/content-build".to_string());
    let home_yaml = std::fs::read_to_string(format!("{}/home.yaml", content_base_path))
        .unwrap_or_else(|_| String::new());
    let home_config: HomeConfig =
        serde_yaml::from_str(&home_yaml).expect("Failed to parse contents/home.yaml");
    let content_db_path = format!("{}/db.json", content_base_path);
    let content_database_json = std::fs::read_to_string(&content_db_path)
        .with_context(|| format!("Failed to read content database at {content_db_path}"))?;
    let content_database: ContentDatabase = serde_json::from_str(&content_database_json)
        .with_context(|| format!("Failed to parse content database at {content_db_path}"))?;
    let rss_path = format!("{content_base_path}/rss.xml");
    let rss_feed = RssFeed(
        std::fs::read_to_string(&rss_path)
            .with_context(|| format!("Failed to read RSS feed at {rss_path}"))?,
    );
    let public_root = PublicRoot(std::path::PathBuf::from(format!("{content_base_path}/public")));

    let conf = leptos::config::get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(frontend::App);

    let mut api_doc = ApiDoc::openapi();
    api_doc.info.version = env!("CARGO_PKG_VERSION").to_string();

    let server = HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        let site_root = leptos_options.site_root.clone();
        let media_path =
            std::env::var("MEDIA_PATH").unwrap_or_else(|_| "contents/media".to_string());

        let (app, api) = App::new()
            .into_utoipa_app()
            .openapi(api_doc.clone())
            .map(|app| app.wrap(TracingLogger::default()))
            .map(|app| app.wrap(Compress::default()))
            .app_data(Data::new(home_config.clone()))
            .app_data(Data::new(content_database.clone()))
            .app_data(Data::new(rss_feed.clone()))
            .app_data(Data::new(public_root.clone()))
            .app_data(Data::new(leptos_options.clone()))
            .service(api_v1_scope())
            .service(api::rss::get_rss)
            .split_for_parts();

        app.service(public_scope())
            .service(Scalar::with_url("/api/docs", api))
            .service(get_sitemap)
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                let home_config = home_config.clone();
                let content_database = content_database.clone();
                move || {
                    use frontend::App;
                    provide_context(home_config.clone());
                    provide_context(content_database.clone());
                    view! {
                        <!DOCTYPE html>
                        <html lang="fr">
                            <head>
                                <Meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <Meta charset="UTF-8" />

                                <Link rel="stylesheet" href="/pkg/frontend.css" />
                                <Link rel="shortcut icon" href="/assets/favicon.ico" />
                                <Link
                                    rel="icon"
                                    href="/assets/favicon-96x96.png"
                                    sizes="96x96"
                                    type_="image/png"
                                />
                                <Link rel="icon" href="/assets/favicon.svg" type_="image/svg+xml" />
                                <Link
                                    rel="apple-touch-icon"
                                    sizes="180x180"
                                    href="/assets/apple-touch-icon.png"
                                />
                                <Link rel="manifest" href="/assets/manifest.json" />
                                <leptos_meta::MetaTags />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .service(Files::new("/assets", site_root.to_string()))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/media", media_path))
    })
    .bind((Ipv4Addr::UNSPECIFIED, addr.port()))?;

    server.run().await?;

    if let Err(e) = shutdown_tracing(tracing_output) {
        eprintln!("Error during shutdown of tracing: {e}");
    }
    Ok(())
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::build_sitemap_xml;
    use content::{ContentDatabase, ContentDates, ContentEntry, ContentKind, MarkdownContent};

    #[test]
    fn sitemap_contains_static_and_content_urls() {
        let database = ContentDatabase {
            generated_at_unix: 0,
            entries: vec![ContentEntry {
                title: "Post".to_string(),
                description: String::new(),
                handle: "blogs/test".to_string(),
                source_path: "blogs/test.md".to_string(),
                kind: ContentKind {
                    blog: true,
                    project: false,
                    doc: false,
                },
                dates: ContentDates {
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    updated_at_unix: 0,
                    released_at: "2024-01-01".to_string(),
                },
                draft: false,
                tags: vec![],
                techno: vec![],
                image: String::new(),
                reading_time_minutes: 1,
                toc: vec![],
                content: MarkdownContent {
                    format: "markdown_ast".to_string(),
                    nodes: vec![],
                },
            }],
            sidebar: vec![],
            blog_timeline: vec![],
        };

        let xml = build_sitemap_xml(&database);
        assert!(xml.contains("<loc>https://maxleriche.net/</loc>"));
        assert!(xml.contains("<loc>https://maxleriche.net/about</loc>"));
        assert!(xml.contains("<loc>https://maxleriche.net/blogs/test</loc>"));
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // The WASM entry point is handled by leptos hydration.
}
