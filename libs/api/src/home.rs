use actix_web::{get, web::Data, HttpResponse, Responder};
use content::{HistoryEntry, HomeConfig, SocialLink};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

/// A social link in the home-page API response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SocialLinkResponse {
    pub name: String,
    pub url: String,
    pub primaire: bool,
    pub img_url: String,
}

impl From<SocialLink> for SocialLinkResponse {
    fn from(s: SocialLink) -> Self {
        Self {
            name: s.name,
            url: s.url,
            primaire: s.primaire,
            img_url: s.img_url,
        }
    }
}

/// A career/education timeline entry in the home-page API response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HistoryEntryResponse {
    pub title: String,
    pub title_en: Option<String>,
    pub lieux: String,
    pub date: String,
    pub weight: u32,
    pub img_url: String,
    pub ico_url: String,
    pub description: String,
    pub description_en: Option<String>,
    pub url: Vec<SocialLinkResponse>,
}

impl From<HistoryEntry> for HistoryEntryResponse {
    fn from(h: HistoryEntry) -> Self {
        Self {
            title: h.title,
            title_en: h.title_en,
            lieux: h.lieux,
            date: h.date,
            weight: h.weight,
            img_url: h.img_url,
            ico_url: h.ico_url,
            description: h.description,
            description_en: h.description_en,
            url: h.url.into_iter().map(SocialLinkResponse::from).collect(),
        }
    }
}

/// Full home-page data returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HomeResponse {
    pub name: String,
    pub presentation: String,
    pub presentation_en: Option<String>,
    pub short_description: String,
    pub short_description_en: Option<String>,
    pub cover_title: Vec<String>,
    pub cover_title_en: Option<Vec<String>>,
    pub cv_url: String,
    pub contact_email: Option<String>,
    pub contact_location: Option<String>,
    pub current_work: Option<String>,
    pub contact_availability: Option<String>,
    pub contact_availability_en: Option<String>,
    pub url: Vec<SocialLinkResponse>,
    pub useful_links: Vec<SocialLinkResponse>,
    pub history: Vec<HistoryEntryResponse>,
}

impl From<HomeConfig> for HomeResponse {
    fn from(cfg: HomeConfig) -> Self {
        Self {
            name: cfg.name,
            presentation: cfg.presentation,
            presentation_en: cfg.presentation_en,
            short_description: cfg.short_description,
            short_description_en: cfg.short_description_en,
            cover_title: cfg.cover_title,
            cover_title_en: cfg.cover_title_en,
            cv_url: cfg.cv_url,
            contact_email: cfg.contact_email,
            contact_location: cfg.contact_location,
            current_work: cfg.current_work,
            contact_availability: cfg.contact_availability,
            contact_availability_en: cfg.contact_availability_en,
            url: cfg.url.into_iter().map(SocialLinkResponse::from).collect(),
            useful_links: cfg
                .useful_links
                .into_iter()
                .map(SocialLinkResponse::from)
                .collect(),
            history: cfg
                .history
                .into_iter()
                .map(HistoryEntryResponse::from)
                .collect(),
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
            presentation_en: Some("Hello EN".to_string()),
            short_description: "Dev".to_string(),
            short_description_en: None,
            cover_title: vec!["Ops, Back, Front.".to_string()],
            cover_title_en: None,
            cv_url: "cv.pdf".to_string(),
            contact_email: Some("contact@example.dev".to_string()),
            contact_location: Some("Niort, France".to_string()),
            current_work: Some(
                "Ingénieur Socle de fabrication / Couche d'échange à la Macif".to_string(),
            ),
            contact_availability: Some("Disponible".to_string()),
            contact_availability_en: Some("Available".to_string()),
            url: vec![],
            useful_links: vec![],
            history: vec![],
        }
    }

    #[actix_web::test]
    async fn home_response_from_config() {
        let cfg = sample_config();
        let resp = HomeResponse::from(cfg);
        assert_eq!(resp.name, "Max");
        assert_eq!(resp.cv_url, "cv.pdf");
        assert_eq!(resp.presentation_en, Some("Hello EN".to_string()));
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
        assert_eq!(body["presentation_en"], "Hello EN");
    }
}
