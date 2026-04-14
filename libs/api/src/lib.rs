pub mod content_db;
pub mod doc;
pub mod health;
pub mod home;
pub mod public;
pub mod rss;

pub use doc::ApiDoc;

/// Register all `/api/v1` routes on a [`utoipa_actix_web`] app.
///
/// Returns a configured scope ready to be passed to `UtoipaApp::service`.
pub fn api_v1_scope(
) -> impl actix_web::dev::HttpServiceFactory + utoipa_actix_web::OpenApiFactory + 'static {
    utoipa_actix_web::scope("/api/v1")
        .service(health::health)
        .service(home::get_home)
        .service(content_db::get_content_database_status)
}
