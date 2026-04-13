#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_files::Files;
    use actix_web::{middleware::Compress, web::Data, App, HttpServer};
    use api::{health::health, home::get_home, ApiDoc};
    use content::HomeConfig;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use std::net::Ipv4Addr;
    use trace::{shutdown_tracing, start_tracing, Context};
    use tracing_actix_web::TracingLogger;
    use utoipa::OpenApi;
    use utoipa_actix_web::{scope, AppExt};
    use utoipa_scalar::{Scalar, Servable};

    let tracing_output = start_tracing(&Context {
        pod_name: std::env::var("POD_NAME").unwrap_or_else(|_| "not_a_pod".to_string()),
        service_name: "monofolio".to_string(),
    });

    let home_yaml = std::fs::read_to_string("contents/home.yaml").unwrap_or_else(|_| String::new());
    let home_config: HomeConfig =
        serde_yaml::from_str(&home_yaml).expect("Failed to parse contents/home.yaml");

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
            .app_data(Data::new(leptos_options.clone()))
            .service(scope("/api/v1").service(health).service(get_home))
            .split_for_parts();

        app.service(Scalar::with_url("/api/docs", api))
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || {
                    use frontend::App;
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
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
