use actix_files::NamedFile;
use actix_web::{error::ErrorNotFound, web, HttpResponse, Result};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PublicRoot(pub PathBuf);

async fn public_root_not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

async fn get_public_file(
    path: web::Path<String>,
    public_root: web::Data<PublicRoot>,
) -> Result<NamedFile> {
    let raw_path = path.into_inner();
    if raw_path.is_empty() || raw_path.ends_with('/') {
        return Err(ErrorNotFound("Not Found"));
    }

    let mut relative = PathBuf::new();
    for component in Path::new(&raw_path).components() {
        match component {
            Component::Normal(segment) => relative.push(segment),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(ErrorNotFound("Not Found"));
            }
        }
    }

    if relative.as_os_str().is_empty() {
        return Err(ErrorNotFound("Not Found"));
    }

    let file_path = public_root.0.join(relative);
    let metadata = std::fs::metadata(&file_path).map_err(|_| ErrorNotFound("Not Found"))?;
    if !metadata.is_file() {
        return Err(ErrorNotFound("Not Found"));
    }

    NamedFile::open_async(file_path)
        .await
        .map_err(|_| ErrorNotFound("Not Found"))
}

pub fn public_scope() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/public")
        .route("", web::get().to(public_root_not_found))
        .route("/{path:.*}", web::get().to(get_public_file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web::Data, App};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("monofolio-public-{suffix}"))
    }

    #[actix_web::test]
    async fn public_root_returns_404() {
        let root = unique_temp_dir();
        std::fs::create_dir_all(&root).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get().uri("/public").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[actix_web::test]
    async fn directory_target_returns_404() {
        let root = unique_temp_dir();
        std::fs::create_dir_all(root.join("docs")).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get().uri("/public/docs/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[actix_web::test]
    async fn parent_dir_segments_return_404() {
        let root = unique_temp_dir();
        let suffix = root.file_name().unwrap().to_str().unwrap().to_string();
        let outside_dir = std::env::temp_dir().join(format!("escape-{suffix}"));
        let outside_file = outside_dir.join("secret.txt");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside_dir).unwrap();
        std::fs::write(&outside_file, "secret").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/public/../escape-target/secret.txt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_dir_all(outside_dir).unwrap();
    }

    #[actix_web::test]
    async fn encoded_parent_dir_segments_return_404() {
        let root = unique_temp_dir();
        let suffix = root.file_name().unwrap().to_str().unwrap().to_string();
        let outside_dir = std::env::temp_dir().join(format!("escape-{suffix}"));
        let outside_file = outside_dir.join("secret.txt");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside_dir).unwrap();
        std::fs::write(&outside_file, "secret").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/public/%2e%2e/escape-target/secret.txt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_dir_all(outside_dir).unwrap();
    }

    #[actix_web::test]
    async fn double_encoded_parent_dir_return_404() {
        // %252e%252e is the double-encoded form of ".." (%25 = '%', so %252e = '%2e' = '.')
        let root = unique_temp_dir();
        let suffix = root.file_name().unwrap().to_str().unwrap().to_string();
        let outside_dir = std::env::temp_dir().join(format!("escape-dbl-{suffix}"));
        let outside_file = outside_dir.join("secret.txt");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside_dir).unwrap();
        std::fs::write(&outside_file, "secret").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/public/%252e%252e/escape-target-dbl/secret.txt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
        std::fs::remove_dir_all(outside_dir).unwrap();
    }

    #[actix_web::test]
    async fn absolute_path_returns_404() {
        // /etc/passwd or any absolute path should be rejected
        let root = unique_temp_dir();
        std::fs::create_dir_all(&root).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/public/%2fetc%2fpasswd")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[actix_web::test]
    async fn absolute_path_with_drive_prefix_returns_404() {
        // Windows-style absolute path: C:\secret shouldn't escape root on any OS
        let root = unique_temp_dir();
        std::fs::create_dir_all(&root).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(PublicRoot(root.clone())))
                .service(public_scope()),
        )
        .await;

        // Encoded form of "C:\secret.txt" as a URL segment
        let req = test::TestRequest::get()
            .uri("/public/C%3A%5Csecret.txt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        // Either served from inside root (file won't exist → 404) or rejected outright
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        std::fs::remove_dir_all(root).unwrap();
    }
}
