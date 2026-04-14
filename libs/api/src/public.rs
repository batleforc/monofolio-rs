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
}
