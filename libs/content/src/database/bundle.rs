use std::fs;
use std::path::Path;

use super::{
    build_content_database, copy_file_to_bundle, process_home_yaml_media_for_bundle,
    process_markdown_media_for_bundle, process_mermaid_codeblocks_for_bundle,
    write_content_database_json, ContentDatabase, ContentDatabaseError, ContentOutputBundle,
};

pub fn prepare_content_output_bundle(
    content_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<ContentOutputBundle, ContentDatabaseError> {
    let content_root = content_root.as_ref();
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir).map_err(|source| ContentDatabaseError::CreateDir {
        path: output_dir.display().to_string(),
        source,
    })?;

    let config_src = content_root.join("config.yaml");
    let home_src = content_root.join("home.yaml");
    let config_dst = output_dir.join("config.yaml");
    let home_dst = output_dir.join("home.yaml");

    copy_file_to_bundle(&config_src, &config_dst)?;
    copy_file_to_bundle(&home_src, &home_dst)?;

    let public_dir = output_dir.join("public");
    fs::create_dir_all(&public_dir).map_err(|source| ContentDatabaseError::CreateDir {
        path: public_dir.display().to_string(),
        source,
    })?;

    Ok(ContentOutputBundle {
        root_dir: output_dir.to_path_buf(),
        db_path: output_dir.join("db.json"),
        config_path: config_dst,
        home_path: home_dst,
        public_dir,
    })
}

pub fn finalize_content_output_bundle(
    bundle: &ContentOutputBundle,
    database: &ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    write_content_database_json(database, &bundle.db_path)
}

pub fn build_content_database_and_prepare_bundle(
    content_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<(ContentDatabase, ContentOutputBundle), ContentDatabaseError> {
    let content_root = content_root.as_ref();
    let database = build_content_database(content_root)?;
    let bundle = prepare_content_output_bundle(content_root, output_dir)?;
    Ok((database, bundle))
}

pub fn build_content_database_and_bundle(
    content_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<(ContentDatabase, ContentOutputBundle), ContentDatabaseError> {
    let content_root = content_root.as_ref();
    let (mut database, bundle) =
        build_content_database_and_prepare_bundle(content_root, output_dir)?;
    process_markdown_media_for_bundle(content_root, &bundle, &mut database)?;
    process_home_yaml_media_for_bundle(content_root, &bundle)?;
    process_mermaid_codeblocks_for_bundle(&bundle, &mut database)?;
    finalize_content_output_bundle(&bundle, &database)?;
    Ok((database, bundle))
}

pub fn create_content_output_bundle(
    content_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    database: &ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    let bundle = prepare_content_output_bundle(content_root, output_dir)?;
    finalize_content_output_bundle(&bundle, database)
}
