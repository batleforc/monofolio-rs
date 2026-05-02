use actix_web::{get, web::Data, HttpResponse, Responder};
use content::ContentDatabase;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

/// Summary of content database stats exposed by the API.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct ContentDatabaseStatusResponse {
    pub generated_at_unix: u64,
    pub total_entries: usize,
    pub blog_entries: usize,
    pub doc_entries: usize,
    pub project_entries: usize,
}

impl From<&ContentDatabase> for ContentDatabaseStatusResponse {
    fn from(database: &ContentDatabase) -> Self {
        let blog_entries = database
            .entries
            .iter()
            .filter(|entry| entry.kind.blog)
            .count();
        let doc_entries = database
            .entries
            .iter()
            .filter(|entry| entry.kind.doc)
            .count();
        let project_entries = database
            .entries
            .iter()
            .filter(|entry| entry.kind.project)
            .count();

        Self {
            generated_at_unix: database.generated_at_unix,
            total_entries: database.entries.len(),
            blog_entries,
            doc_entries,
            project_entries,
        }
    }
}

/// Return summary metrics for the content database currently used by the API.
#[utoipa::path(
    tag = "content-db",
    responses(
        (status = 200, description = "Content database stats.", body = ContentDatabaseStatusResponse),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/content-db/status")]
#[instrument(name = "get_content_database_status", skip(database))]
pub async fn get_content_database_status(database: Data<ContentDatabase>) -> impl Responder {
    info!(
        entries = database.entries.len(),
        "Serving content database status"
    );
    HttpResponse::Ok().json(ContentDatabaseStatusResponse::from(database.get_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Data, App};

    fn sample_database() -> ContentDatabase {
        ContentDatabase {
            generated_at_unix: 42,
            entries: vec![],
            sidebar: vec![],
            blog_timeline: vec![],
            technology_map: vec![],
        }
    }

    #[actix_web::test]
    async fn get_content_database_status_returns_json() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(sample_database()))
                .service(get_content_database_status),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/content-db/status")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: ContentDatabaseStatusResponse = test::read_body_json(resp).await;
        assert_eq!(
            body,
            ContentDatabaseStatusResponse {
                generated_at_unix: 42,
                total_entries: 0,
                blog_entries: 0,
                doc_entries: 0,
                project_entries: 0,
            }
        );
    }
}
