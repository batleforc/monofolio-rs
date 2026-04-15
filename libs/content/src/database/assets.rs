use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::ImageReader;
use mermaid_rs_renderer::render as render_mermaid_svg;
use sha2::{Digest, Sha256};

use crate::markdown::MarkdownNode;

use super::{
    copy_file_to_bundle, media_reference_to_relative_path, public_media_url, public_mermaid_url,
    ContentDatabase, ContentDatabaseError, ContentOutputBundle,
};

fn rewrite_media_urls_in_nodes(
    nodes: &mut [MarkdownNode],
    content_media_root: &Path,
    bundle_media_root: &Path,
) -> Result<(), ContentDatabaseError> {
    for node in nodes {
        if (node.kind == "image" || node.kind == "link") && node.attrs.contains_key("url") {
            if let Some(current) = node.attrs.get("url").cloned() {
                if let Some(relative_path) = media_reference_to_relative_path(&current) {
                    let src = content_media_root.join(&relative_path);
                    let dst = bundle_media_root.join(&relative_path);
                    optimize_or_copy_media_file(&src, &dst)?;
                    node.attrs
                        .insert("url".to_string(), public_media_url(&relative_path));
                }
            }
        }

        rewrite_media_urls_in_nodes(&mut node.children, content_media_root, bundle_media_root)?;
    }

    Ok(())
}

fn collect_media_refs_in_yaml(value: &serde_yaml::Value, refs: &mut BTreeSet<String>) {
    match value {
        serde_yaml::Value::String(raw) => {
            if let Some(relative_path) = media_reference_to_relative_path(raw) {
                refs.insert(relative_path);
            }
        }
        serde_yaml::Value::Sequence(items) => {
            for item in items {
                collect_media_refs_in_yaml(item, refs);
            }
        }
        serde_yaml::Value::Mapping(map) => {
            for value in map.values() {
                collect_media_refs_in_yaml(value, refs);
            }
        }
        _ => {}
    }
}

fn collect_cv_refs_in_yaml(value: &serde_yaml::Value, refs: &mut BTreeSet<String>) {
    // get cvUrl key (if any) and collect media refs from its value
    if let serde_yaml::Value::Mapping(map) = value {
        for (key, value) in map {
            if let serde_yaml::Value::String(key_str) = key {
                if key_str == "cvUrl" {
                    if let Some(url) = value.as_str() {
                        refs.insert(url.to_string());
                    }
                }
            }
        }
    }
}

fn node_text(node: &MarkdownNode) -> String {
    let mut text = node.text.clone();
    for child in &node.children {
        text.push_str(&node_text(child));
    }
    text
}

fn replace_mermaid_blocks_in_nodes<F>(
    nodes: &mut [MarkdownNode],
    entry_handle: &str,
    bundle_mermaid_root: &Path,
    render_mermaid: &F,
) -> Result<(), ContentDatabaseError>
where
    F: Fn(&str, &Path) -> Result<(), ContentDatabaseError>,
{
    for node in nodes {
        if node.kind == "code_block"
            && node.attrs.get("language").map(String::as_str) == Some("mermaid")
        {
            let source = node_text(node);
            let mut hasher = Sha256::new();
            hasher.update(entry_handle.as_bytes());
            hasher.update(b"::");
            hasher.update(source.as_bytes());
            let digest = format!("{:x}", hasher.finalize());
            let file_name = format!("{}-{}.svg", entry_handle.replace('/', "-"), &digest[..16]);
            let svg_path = bundle_mermaid_root.join(&file_name);

            render_mermaid(&source, &svg_path)?;

            node.kind = "image".to_string();
            node.text.clear();
            node.children.clear();
            node.attrs.clear();
            node.attrs
                .insert("url".to_string(), public_mermaid_url(&file_name));
            node.attrs
                .insert("alt".to_string(), "Mermaid diagram".to_string());
            continue;
        }

        replace_mermaid_blocks_in_nodes(
            &mut node.children,
            entry_handle,
            bundle_mermaid_root,
            render_mermaid,
        )?;
    }

    Ok(())
}

fn render_mermaid_with_renderer(
    source: &str,
    output_path: &Path,
) -> Result<(), ContentDatabaseError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|source| ContentDatabaseError::CreateDir {
            path: parent.display().to_string(),
            source,
        })?;
    }

    let svg = render_mermaid_svg(source).map_err(|error| ContentDatabaseError::MermaidRender {
        path: output_path.display().to_string(),
        message: error.to_string(),
    })?;

    fs::write(output_path, svg).map_err(|source| ContentDatabaseError::WriteFile {
        path: output_path.display().to_string(),
        source,
    })?;

    Ok(())
}

pub(crate) fn process_mermaid_codeblocks_for_bundle_with<F>(
    bundle: &ContentOutputBundle,
    database: &mut ContentDatabase,
    render_mermaid: F,
) -> Result<(), ContentDatabaseError>
where
    F: Fn(&str, &Path) -> Result<(), ContentDatabaseError>,
{
    let bundle_mermaid_root = bundle.public_dir.join("mermaid");
    fs::create_dir_all(&bundle_mermaid_root).map_err(|source| ContentDatabaseError::CreateDir {
        path: bundle_mermaid_root.display().to_string(),
        source,
    })?;

    for entry in &mut database.entries {
        replace_mermaid_blocks_in_nodes(
            &mut entry.content.nodes,
            &entry.handle,
            &bundle_mermaid_root,
            &render_mermaid,
        )?;
    }

    Ok(())
}

/// Stage 1.6: translate mermaid code blocks into SVG files and update the AST.
pub fn process_mermaid_codeblocks_for_bundle(
    bundle: &ContentOutputBundle,
    database: &mut ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    process_mermaid_codeblocks_for_bundle_with(bundle, database, render_mermaid_with_renderer)
}

/// Stage 1.5: process markdown media references and populate `public/media`.
///
/// For every `image` or `link` node whose URL references `media#...`,
/// `/media/...`, `media/...` or `/public/media/...`, this step:
/// - optimizes/copies the file from `<content_root>/media/...`
/// - writes it into `<bundle.public_dir>/media/...`
/// - rewrites the AST URL to `/public/media/...`
pub fn process_markdown_media_for_bundle(
    content_root: impl AsRef<Path>,
    bundle: &ContentOutputBundle,
    database: &mut ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    let content_media_root = content_root.as_ref().join("media");
    let bundle_media_root = bundle.public_dir.join("media");

    fs::create_dir_all(&bundle_media_root).map_err(|source| ContentDatabaseError::CreateDir {
        path: bundle_media_root.display().to_string(),
        source,
    })?;

    for entry in &mut database.entries {
        rewrite_media_urls_in_nodes(
            &mut entry.content.nodes,
            &content_media_root,
            &bundle_media_root,
        )?;

        if let Some(relative_path) = media_reference_to_relative_path(&entry.image) {
            let src = content_media_root.join(&relative_path);
            let dst = bundle_media_root.join(&relative_path);
            optimize_or_copy_media_file(&src, &dst)?;
            entry.image = public_media_url(&relative_path);
        }
    }

    for item in &mut database.blog_timeline {
        if let Some(relative_path) = media_reference_to_relative_path(&item.image) {
            item.image = public_media_url(&relative_path);
        }
    }

    Ok(())
}

/// Stage 1.55: copy media files referenced in `home.yaml` into `public/media`.
///
/// This scans all string values in the YAML document and resolves media
/// references using the same conventions as markdown media handling:
/// `media#...`, `/media/...`, `media/...`, `/public/media/...`.
pub fn process_home_yaml_media_for_bundle(
    content_root: impl AsRef<Path>,
    bundle: &ContentOutputBundle,
) -> Result<(), ContentDatabaseError> {
    let content_media_root = content_root.as_ref().join("media");
    let bundle_media_root = bundle.public_dir.join("media");

    fs::create_dir_all(&bundle_media_root).map_err(|source| ContentDatabaseError::CreateDir {
        path: bundle_media_root.display().to_string(),
        source,
    })?;

    let home_content =
        fs::read_to_string(&bundle.home_path).map_err(|source| ContentDatabaseError::ReadFile {
            path: bundle.home_path.display().to_string(),
            source,
        })?;

    let home_yaml: serde_yaml::Value =
        serde_yaml::from_str(&home_content).map_err(|error| ContentDatabaseError::ParseYaml {
            path: bundle.home_path.display().to_string(),
            message: error.to_string(),
        })?;

    let mut refs = BTreeSet::new();
    collect_media_refs_in_yaml(&home_yaml, &mut refs);
    collect_cv_refs_in_yaml(&home_yaml, &mut refs);

    for relative_path in refs {
        let src = content_media_root.join(&relative_path);
        let dst = bundle_media_root.join(&relative_path);
        optimize_or_copy_media_file(&src, &dst)?;
    }

    Ok(())
}

pub(super) fn optimize_or_copy_media_file(
    src: &Path,
    dst: &Path,
) -> Result<(), ContentDatabaseError> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(|source| ContentDatabaseError::CreateDir {
            path: parent.display().to_string(),
            source,
        })?;
    }

    let ext = src
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" => {
            let image = ImageReader::open(src)
                .map_err(|error| ContentDatabaseError::DecodeImage {
                    path: src.display().to_string(),
                    message: error.to_string(),
                })?
                .decode()
                .map_err(|error| ContentDatabaseError::DecodeImage {
                    path: src.display().to_string(),
                    message: error.to_string(),
                })?;

            let mut file =
                fs::File::create(dst).map_err(|source| ContentDatabaseError::WriteFile {
                    path: dst.display().to_string(),
                    source,
                })?;
            let mut encoder = JpegEncoder::new_with_quality(&mut file, 80);
            encoder
                .encode_image(&image)
                .map_err(|error| ContentDatabaseError::EncodeImage {
                    path: dst.display().to_string(),
                    message: error.to_string(),
                })?;
            Ok(())
        }
        "png" | "webp" => {
            let image = ImageReader::open(src)
                .map_err(|error| ContentDatabaseError::DecodeImage {
                    path: src.display().to_string(),
                    message: error.to_string(),
                })?
                .decode()
                .map_err(|error| ContentDatabaseError::DecodeImage {
                    path: src.display().to_string(),
                    message: error.to_string(),
                })?;

            image
                .save(dst)
                .map_err(|error| ContentDatabaseError::EncodeImage {
                    path: dst.display().to_string(),
                    message: error.to_string(),
                })
        }
        _ => copy_file_to_bundle(src, dst),
    }
}
