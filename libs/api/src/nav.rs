use std::collections::HashSet;

use actix_web::{get, web::Data, HttpResponse, Responder};
use content::{BlogTimelineEntry, ContentDatabase, ContentEntry, SidebarItem};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

/// A lightweight project summary derived from a [`ContentEntry`].
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct ProjectSummary {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub tags: Vec<String>,
    pub techno: Vec<String>,
    pub image: String,
    pub released_at: String,
}

impl From<&ContentEntry> for ProjectSummary {
    fn from(entry: &ContentEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            tags: entry.tags.clone(),
            techno: entry.techno.clone(),
            image: entry.image.clone(),
            released_at: entry.dates.released_at.clone(),
        }
    }
}

/// Lightweight entry used by the global navbar search.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct SearchEntry {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub href: String,
    pub kind: String,
}

impl From<&ContentEntry> for SearchEntry {
    fn from(entry: &ContentEntry) -> Self {
        let kind = if entry.kind.doc {
            "doc"
        } else if entry.kind.blog {
            "blog"
        } else if entry.kind.project {
            "project"
        } else {
            "content"
        };

        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            href: format!("/{}", entry.handle.trim_start_matches('/')),
            kind: kind.to_string(),
        }
    }
}

/// Blog timeline entry used for the blog sidebar.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct BlogSidebarEntry {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub date: String,
    pub tags: Vec<String>,
    pub image: String,
    pub reading_time_minutes: usize,
    pub draft: bool,
}

impl From<&BlogTimelineEntry> for BlogSidebarEntry {
    fn from(entry: &BlogTimelineEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            date: entry.date.clone(),
            tags: entry.tags.clone(),
            image: entry.image.clone(),
            reading_time_minutes: entry.reading_time_minutes,
            draft: entry.draft,
        }
    }
}

/// Whether a doc sidebar item is a plain markdown file, a pure folder, or a folder with an index.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocSidebarItemKind {
    /// A leaf node backed by a single markdown file.
    Markdown,
    /// A directory with no associated `index.md`.
    Folder,
    /// A directory that has an associated `index.md` file.
    FolderWithIndex,
}

/// Recursive doc sidebar tree node.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct DocSidebarItem {
    pub title: String,
    pub handle: String,
    pub order: usize,
    pub kind: DocSidebarItemKind,
    #[schema(no_recursion)]
    pub children: Vec<DocSidebarItem>,
}

fn doc_sidebar_item_from(item: &SidebarItem, known_handles: &HashSet<String>) -> DocSidebarItem {
    let kind = if item.children.is_empty() {
        DocSidebarItemKind::Markdown
    } else if known_handles.contains(&item.handle) {
        DocSidebarItemKind::FolderWithIndex
    } else {
        DocSidebarItemKind::Folder
    };
    let children = item
        .children
        .iter()
        .map(|child| doc_sidebar_item_from(child, known_handles))
        .collect();
    DocSidebarItem {
        title: item.title.clone(),
        handle: item.handle.clone(),
        order: item.order,
        kind,
        children,
    }
}

fn build_doc_sidebar(database: &ContentDatabase) -> Vec<DocSidebarItem> {
    let known_handles: HashSet<String> = database
        .entries
        .iter()
        .filter(|e| e.kind.doc)
        .map(|e| e.handle.clone())
        .collect();
    database
        .sidebar
        .iter()
        .map(|item| doc_sidebar_item_from(item, &known_handles))
        .collect()
}

// ── Placeholder to satisfy the compiler (NavResponse kept for internal use) ──
// (removed — replaced by 3 dedicated handlers below)

// the `projects` arm of the old NavResponse::from was:
//   projects: database.entries.iter().filter(|entry| entry.kind.project).map(...).collect(),
// kept here as a comment so the compiler can locate the old deletion point
// ── Endpoints ────────────────────────────────────────────────────────────────

/// Return the blog sidebar (chronological list of blog posts).
#[utoipa::path(
    tag = "nav",
    responses(
        (status = 200, description = "Blog sidebar entries.", body = Vec<BlogSidebarEntry>),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/nav/blog")]
#[instrument(name = "get_blog_nav", skip(database))]
pub async fn get_blog_nav(database: Data<ContentDatabase>) -> impl Responder {
    info!("Serving blog nav");
    let entries: Vec<BlogSidebarEntry> =
        database.blog_timeline.iter().map(BlogSidebarEntry::from).collect();
    HttpResponse::Ok().json(entries)
}

/// Return the doc sidebar (documentation tree with item type hints).
#[utoipa::path(
    tag = "nav",
    responses(
        (status = 200, description = "Doc sidebar tree.", body = Vec<DocSidebarItem>),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/nav/doc")]
#[instrument(name = "get_doc_nav", skip(database))]
pub async fn get_doc_nav(database: Data<ContentDatabase>) -> impl Responder {
    info!("Serving doc nav");
    HttpResponse::Ok().json(build_doc_sidebar(database.get_ref()))
}

/// Return the project list.
#[utoipa::path(
    tag = "nav",
    responses(
        (status = 200, description = "Project summaries.", body = Vec<ProjectSummary>),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/nav/projects")]
#[instrument(name = "get_projects_nav", skip(database))]
pub async fn get_projects_nav(database: Data<ContentDatabase>) -> impl Responder {
    info!("Serving projects nav");
    let projects: Vec<ProjectSummary> = database
        .entries
        .iter()
        .filter(|e| e.kind.project)
        .map(ProjectSummary::from)
        .collect();
    HttpResponse::Ok().json(projects)
}

/// Return a flat search index spanning all content-backed pages.
#[utoipa::path(
    tag = "nav",
    responses(
        (status = 200, description = "Search index entries.", body = Vec<SearchEntry>),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/search")]
#[instrument(name = "get_search_index", skip(database))]
pub async fn get_search_index(database: Data<ContentDatabase>) -> impl Responder {
    info!("Serving search index");
    let mut entries: Vec<SearchEntry> = database.entries.iter().map(SearchEntry::from).collect();
    entries.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    HttpResponse::Ok().json(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test as actix_test, web::Data, App};
    use content::{
        BlogTimelineEntry, ContentDatabase, ContentDates, ContentEntry, ContentKind, MarkdownContent,
        SidebarItem,
    };

    fn sample_database() -> ContentDatabase {
        ContentDatabase {
            generated_at_unix: 0,
            entries: vec![
                ContentEntry {
                    title: "My Project".to_string(),
                    description: "A cool project".to_string(),
                    handle: "project/my-project".to_string(),
                    source_path: "project/my-project.md".to_string(),
                    kind: ContentKind { blog: false, project: true, doc: false },
                    dates: ContentDates {
                        created_at: "2024-01-01T00:00:00Z".to_string(),
                        updated_at_unix: 0,
                        released_at: "2024-01-01".to_string(),
                    },
                    draft: false,
                    tags: vec!["rust".to_string()],
                    techno: vec!["actix".to_string()],
                    image: "".to_string(),
                    reading_time_minutes: 3,
                    toc: vec![],
                    content: MarkdownContent { format: "markdown_ast".to_string(), nodes: vec![] },
                },
                ContentEntry {
                    title: "My Blog Post".to_string(),
                    description: "".to_string(),
                    handle: "blogs/first".to_string(),
                    source_path: "blogs/first.md".to_string(),
                    kind: ContentKind { blog: true, project: false, doc: false },
                    dates: ContentDates {
                        created_at: "2024-02-01T00:00:00Z".to_string(),
                        updated_at_unix: 0,
                        released_at: "".to_string(),
                    },
                    draft: false,
                    tags: vec![],
                    techno: vec![],
                    image: "".to_string(),
                    reading_time_minutes: 1,
                    toc: vec![],
                    content: MarkdownContent { format: "markdown_ast".to_string(), nodes: vec![] },
                },
                // Doc entry that acts as the index of the "docs" folder.
                ContentEntry {
                    title: "Docs Index".to_string(),
                    description: "".to_string(),
                    handle: "docs".to_string(),
                    source_path: "docs/index.md".to_string(),
                    kind: ContentKind { blog: false, project: false, doc: true },
                    dates: ContentDates {
                        created_at: "2024-01-01T00:00:00Z".to_string(),
                        updated_at_unix: 0,
                        released_at: "".to_string(),
                    },
                    draft: false,
                    tags: vec![],
                    techno: vec![],
                    image: "".to_string(),
                    reading_time_minutes: 1,
                    toc: vec![],
                    content: MarkdownContent { format: "markdown_ast".to_string(), nodes: vec![] },
                },
            ],
            sidebar: vec![
                // FolderWithIndex: has children AND "docs" handle exists in entries.
                SidebarItem {
                    title: "Docs".to_string(),
                    handle: "docs".to_string(),
                    order: 0,
                    children: vec![
                        // Markdown leaf.
                        SidebarItem {
                            title: "A Page".to_string(),
                            handle: "docs/a-page".to_string(),
                            order: 0,
                            children: vec![],
                        },
                        // Pure folder: has children but no matching ContentEntry.
                        SidebarItem {
                            title: "Sub Folder".to_string(),
                            handle: "docs/sub-folder".to_string(),
                            order: 1,
                            children: vec![SidebarItem {
                                title: "Sub Page".to_string(),
                                handle: "docs/sub-folder/sub-page".to_string(),
                                order: 0,
                                children: vec![],
                            }],
                        },
                    ],
                },
            ],
            blog_timeline: vec![BlogTimelineEntry {
                title: "My Blog Post".to_string(),
                description: "".to_string(),
                handle: "blogs/first".to_string(),
                date: "2024-02-01T00:00:00Z".to_string(),
                tags: vec![],
                image: "".to_string(),
                reading_time_minutes: 1,
                draft: false,
            }],
        }
    }

    // ── DocSidebarItemKind inference ──────────────────────────────────────────

    #[test]
    fn item_kinds_are_inferred_correctly() {
        let db = sample_database();
        let nav = build_doc_sidebar(&db);
        // docs: FolderWithIndex (has children + "docs" exists in entries)
        assert_eq!(nav[0].kind, DocSidebarItemKind::FolderWithIndex);
        // docs/a-page: Markdown (leaf)
        assert_eq!(nav[0].children[0].kind, DocSidebarItemKind::Markdown);
        // docs/sub-folder: Folder (has children, no matching content entry)
        assert_eq!(nav[0].children[1].kind, DocSidebarItemKind::Folder);
        // docs/sub-folder/sub-page: Markdown (leaf)
        assert_eq!(nav[0].children[1].children[0].kind, DocSidebarItemKind::Markdown);
    }

    // ── GET /nav/blog ─────────────────────────────────────────────────────────

    #[test]
    fn blog_nav_matches_timeline() {
        let db = sample_database();
        let entries: Vec<BlogSidebarEntry> =
            db.blog_timeline.iter().map(BlogSidebarEntry::from).collect();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].handle, "blogs/first");
    }

    #[actix_web::test]
    async fn get_blog_nav_returns_200() {
        let app = actix_test::init_service(
            App::new().app_data(Data::new(sample_database())).service(get_blog_nav),
        )
        .await;
        let req = actix_test::TestRequest::get().uri("/nav/blog").to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["handle"], "blogs/first");
    }

    // ── GET /nav/doc ──────────────────────────────────────────────────────────

    #[actix_web::test]
    async fn get_doc_nav_returns_200_with_kinds() {
        let app = actix_test::init_service(
            App::new().app_data(Data::new(sample_database())).service(get_doc_nav),
        )
        .await;
        let req = actix_test::TestRequest::get().uri("/nav/doc").to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["kind"], "folder_with_index");
        assert_eq!(body[0]["children"][0]["kind"], "markdown");
        assert_eq!(body[0]["children"][1]["kind"], "folder");
    }

    // ── GET /nav/projects ─────────────────────────────────────────────────────

    #[test]
    fn projects_nav_only_includes_project_kind() {
        let db = sample_database();
        let projects: Vec<ProjectSummary> =
            db.entries.iter().filter(|e| e.kind.project).map(ProjectSummary::from).collect();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].handle, "project/my-project");
    }

    #[actix_web::test]
    async fn get_projects_nav_returns_200() {
        let app = actix_test::init_service(
            App::new().app_data(Data::new(sample_database())).service(get_projects_nav),
        )
        .await;
        let req = actix_test::TestRequest::get().uri("/nav/projects").to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["handle"], "project/my-project");
    }

    #[actix_web::test]
    async fn get_search_index_returns_entries() {
        let app = actix_test::init_service(
            App::new().app_data(Data::new(sample_database())).service(get_search_index),
        )
        .await;
        let req = actix_test::TestRequest::get().uri("/search").to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: serde_json::Value = actix_test::read_body_json(resp).await;
        assert_eq!(body.as_array().unwrap().len(), 3);
        assert_eq!(body[0]["href"], "/blogs/first");
    }
}
