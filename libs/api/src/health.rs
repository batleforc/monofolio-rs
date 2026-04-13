use actix_web::{get, HttpResponse, Responder};
use tracing::{info, instrument};

/// Returns `200 OK` when the server is up.
///
/// Intended for liveness and readiness probes.
#[utoipa::path(
    tag = "health",
    responses(
        (status = 200, description = "Server is healthy."),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/health")]
#[instrument(name = "health_check")]
pub async fn health() -> impl Responder {
    info!("Health check OK");
    HttpResponse::Ok().body("OK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn health_returns_200() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
