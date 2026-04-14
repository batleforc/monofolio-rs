use utoipa::OpenApi;

/// OpenAPI document for the Monofolio REST API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Monofolio API",
        description = "REST API powering the Monofolio personal website.",
        contact(
            name = "Batleforc",
            email = "maxleriche.60@gmail.com",
            url = "https://maxleriche.net"
        ),
        license(name = "MIT", url = "https://opensource.org/license/mit/")
    ),
    tags(
        (name = "health", description = "Health and readiness probes."),
        (name = "home", description = "Home page content."),
        (name = "content-db", description = "Content database connectivity and stats."),
        (name = "rss", description = "RSS 2.0 feed."),
        (name = "nav", description = "Navigation data: blog sidebar, doc sidebar and project list."),
        (name = "page", description = "Single page retrieval by content handle."),
    ),
    servers(
        (url = "/", description = "Current server.")
    )
)]
pub struct ApiDoc;
