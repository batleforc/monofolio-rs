use std::fs;
use std::path::Path;

mod assets;
mod build;
mod bundle;
mod rss;
#[cfg(test)]
mod tests;
mod types;

#[cfg(test)]
pub(crate) use assets::process_mermaid_codeblocks_for_bundle_with;
pub use assets::{
    process_home_yaml_media_for_bundle, process_home_yaml_minia_for_bundle,
    process_markdown_media_for_bundle, process_mermaid_codeblocks_for_bundle,
};
#[cfg(test)]
pub(crate) use build::handle_from_relative_path;
pub use build::{build_content_database, write_content_database_json};
pub use bundle::{
    build_content_database_and_bundle, build_content_database_and_prepare_bundle,
    create_content_output_bundle, finalize_content_output_bundle, prepare_content_output_bundle,
};
pub use types::{
    BlogTimelineEntry, ContentDatabase, ContentDatabaseError, ContentDates, ContentEntry,
    ContentKind, ContentOutputBundle, DirMeta, SidebarItem,
};

pub(super) fn copy_file_to_bundle(from: &Path, to: &Path) -> Result<(), ContentDatabaseError> {
    fs::copy(from, to).map_err(|source| ContentDatabaseError::CopyFile {
        from: from.display().to_string(),
        to: to.display().to_string(),
        source,
    })?;
    Ok(())
}

pub(super) fn media_reference_to_relative_path(url: &str) -> Option<String> {
    if let Some(rest) = url.strip_prefix("media#") {
        return Some(rest.to_string());
    }
    if let Some(rest) = url.strip_prefix("/media/") {
        return Some(rest.to_string());
    }
    if let Some(rest) = url.strip_prefix("media/") {
        return Some(rest.to_string());
    }
    if let Some(rest) = url.strip_prefix("/public/media/") {
        return Some(rest.to_string());
    }
    None
}

pub(super) fn public_media_url(relative_path: &str) -> String {
    format!("/public/media/{relative_path}")
}

pub(super) fn public_mermaid_url(relative_path: &str) -> String {
    format!("/public/mermaid/{relative_path}")
}

pub(super) fn public_minia_url(relative_path: &str) -> String {
    format!("/public/minia/{relative_path}")
}

pub(super) fn load_dir_meta(dir: &Path) -> Option<DirMeta> {
    let path = dir.join("meta.json");
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}
