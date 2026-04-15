#![recursion_limit = "512"]
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
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || {
                    use frontend::App;
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // The WASM entry point is handled by leptos hydration.
}
