use actix_web::{get, web::Data, HttpResponse, Responder};
use content::HomeConfig;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

/// Summarised home-page data returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HomeResponse {
    pub name: String,
    pub short_description: String,
    pub cover_title: Vec<String>,
    pub cv_url: String,
}

impl From<HomeConfig> for HomeResponse {
    fn from(cfg: HomeConfig) -> Self {
        Self {
            name: cfg.name,
            short_description: cfg.short_description,
            cover_title: cfg.cover_title,
            cv_url: cfg.cv_url,
        }
    }
}

/// Return the home-page configuration data.
#[utoipa::path(
    tag = "home",
    responses(
        (status = 200, description = "Home page data.", body = HomeResponse),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/home")]
#[instrument(name = "get_home", skip(home_config))]
pub async fn get_home(home_config: Data<HomeConfig>) -> impl Responder {
    info!("Serving home config");
    let response = HomeResponse::from(home_config.get_ref().clone());
    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Data, App};

    fn sample_config() -> HomeConfig {
        HomeConfig {
            name: "Max".to_string(),
            presentation: "Hello".to_string(),
            short_description: "Dev".to_string(),
            cover_title: vec!["Ops, Back, Front.".to_string()],
            cv_url: "cv.pdf".to_string(),
            url: vec![],
            history: vec![],
        }
    }

    #[actix_web::test]
    async fn home_response_from_config() {
        let cfg = sample_config();
        let resp = HomeResponse::from(cfg);
        assert_eq!(resp.name, "Max");
        assert_eq!(resp.cv_url, "cv.pdf");
    }

    #[actix_web::test]
    async fn get_home_returns_json() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(sample_config()))
                .service(get_home),
        )
        .await;
        let req = test::TestRequest::get().uri("/home").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["name"], "Max");
    }
}
