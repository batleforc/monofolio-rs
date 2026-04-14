use actix_web::{get, web::Data, HttpResponse, Responder};
use tracing::{info, instrument};

/// Newtype holding the pre-built RSS feed XML string.
#[derive(Debug, Clone)]
pub struct RssFeed(pub String);

/// Serve the pre-built RSS 2.0 feed.
#[utoipa::path(
    tag = "rss",
    responses(
        (status = 200, description = "RSS 2.0 feed.", content_type = "application/rss+xml"),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/rss.xml")]
#[instrument(name = "get_rss", skip(feed))]
pub async fn get_rss(feed: Data<RssFeed>) -> impl Responder {
    info!("Serving RSS feed");
    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(feed.0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Data, App};

    #[actix_web::test]
    async fn get_rss_returns_xml_content_type() {
        let feed = RssFeed("<?xml version=\"1.0\"?><rss version=\"2.0\"/>".to_string());
        let app = test::init_service(App::new().app_data(Data::new(feed)).service(get_rss)).await;

        let req = test::TestRequest::get().uri("/rss.xml").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let content_type = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.contains("application/rss+xml"));
    }
}
