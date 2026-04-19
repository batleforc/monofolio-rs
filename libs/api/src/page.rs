use actix_web::{get, web::Data, HttpResponse, Responder};
use content::{ContentDatabase, ContentEntry};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

/// Simplified page payload looked up by content handle.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct PageResponse {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub source_path: String,
    pub created_at: String,
    pub updated_at: String,
    pub blog: bool,
    pub project: bool,
    pub doc: bool,
    pub tags: Vec<String>,
    pub techno: Vec<String>,
    pub image: String,
    pub reading_time_minutes: usize,
    pub content: serde_json::Value,
}

impl From<&ContentEntry> for PageResponse {
    fn from(entry: &ContentEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            source_path: entry.source_path.clone(),
            created_at: entry.dates.created_at.clone(),
            updated_at: entry.dates.updated_at.clone(),
            blog: entry.kind.blog,
            project: entry.kind.project,
            doc: entry.kind.doc,
            tags: entry.tags.clone(),
            techno: entry.techno.clone(),
            image: entry.image.clone(),
            reading_time_minutes: entry.reading_time_minutes,
            content: serde_json::to_value(&entry.content).unwrap_or(serde_json::Value::Null),
        }
    }
}

/// Basic API error response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct ApiErrorResponse {
    pub error: String,
}

/// Return one page by its content handle.
#[utoipa::path(
    tag = "page",
    params(
        ("handle" = String, Path, description = "Content handle, e.g. docs/CICD/portfolio-cicd")
    ),
    responses(
        (status = 200, description = "Page found.", body = PageResponse),
        (status = 404, description = "Page not found.", body = ApiErrorResponse),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/page/{handle:.*}")]
#[instrument(name = "get_page", skip(database))]
pub async fn get_page(path: actix_web::web::Path<String>, database: Data<ContentDatabase>) -> impl Responder {
    let handle = path.into_inner().trim_matches('/').to_string();
    info!(%handle, "Serving page by handle");

    if handle.is_empty() {
        return HttpResponse::NotFound().json(ApiErrorResponse {
            error: "Page not found".to_string(),
        });
    }

    match database.entries.iter().find(|entry| entry.handle == handle) {
        Some(entry) => HttpResponse::Ok().json(PageResponse::from(entry)),
        None => HttpResponse::NotFound().json(ApiErrorResponse {
            error: "Page not found".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test as actix_test, web::Data, App};
    use content::{ContentDatabase, ContentDates, ContentEntry, ContentKind, MarkdownContent};

    fn sample_database() -> ContentDatabase {
        ContentDatabase {
            generated_at_unix: 0,
            entries: vec![ContentEntry {
                title: "Doc page".to_string(),
                description: "A page".to_string(),
                handle: "docs/guide/getting-started".to_string(),
                source_path: "docs/guide/getting-started.md".to_string(),
                minia: None,
                kind: ContentKind {
                    blog: false,
                    project: false,
                    doc: true,
                },
                dates: ContentDates {
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    updated_at: "2024-01-01T00:00:00Z".to_string(),
                    updated_at_unix: 0,
                    released_at: "".to_string(),
                },
                draft: false,
                tags: vec!["guide".to_string()],
                techno: vec![],
                image: "".to_string(),
                reading_time_minutes: 2,
                toc: vec![],
                content: MarkdownContent {
                    format: "markdown_ast".to_string(),
                    nodes: vec![],
                },
            }],
            sidebar: vec![],
            blog_timeline: vec![],
        }
    }

    #[actix_web::test]
    async fn page_response_from_entry() {
        let db = sample_database();
        let resp = PageResponse::from(&db.entries[0]);
        assert_eq!(resp.handle, "docs/guide/getting-started");
        assert!(resp.doc);
    }

    #[actix_web::test]
    async fn get_page_returns_200_when_found() {
        let app = actix_test::init_service(
            App::new()
                .app_data(Data::new(sample_database()))
                .service(get_page),
        )
        .await;

        let req = actix_test::TestRequest::get()
            .uri("/page/docs/guide/getting-started")
            .to_request();
        let resp = actix_test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body["handle"], "docs/guide/getting-started");
    }

    #[actix_web::test]
    async fn get_page_returns_404_when_missing() {
        let app = actix_test::init_service(
            App::new()
                .app_data(Data::new(sample_database()))
                .service(get_page),
        )
        .await;

        let req = actix_test::TestRequest::get().uri("/page/docs/missing").to_request();
        let resp = actix_test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body["error"], "Page not found");
    }
}
