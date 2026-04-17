use std::collections::BTreeSet;
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::ImageReader;
use mermaid_rs_renderer::render as render_mermaid_svg;
use sha2::{Digest, Sha256};

use crate::markdown::MarkdownNode;

use super::{
    copy_file_to_bundle, media_reference_to_relative_path, public_media_url, public_mermaid_url,
    public_minia_url, ContentDatabase, ContentDatabaseError, ContentOutputBundle,
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

fn guess_extension_from_https_url(url: &str) -> &'static str {
    let clean = url.split('?').next().unwrap_or(url);
    let file = clean.rsplit('/').next().unwrap_or("");
    if let Some(ext) = file.rsplit('.').next() {
        match ext.to_ascii_lowercase().as_str() {
            "png" => return "png",
            "jpg" => return "jpg",
            "jpeg" => return "jpeg",
            "webp" => return "webp",
            "svg" => return "svg",
            "ico" => return "ico",
            "gif" => return "gif",
            "avif" => return "avif",
            _ => {}
        }
    }
    "png"
}

#[cfg(not(target_arch = "wasm32"))]
fn fetch_remote_media(url: &str) -> Option<Vec<u8>> {
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(12))
        .call()
        .ok()?;

    if response.status() != 200 {
        return None;
    }

    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok()?;
    if bytes.is_empty() {
        return None;
    }
    Some(bytes)
}

#[cfg(target_arch = "wasm32")]
fn fetch_remote_media(_url: &str) -> Option<Vec<u8>> {
    None
}

fn rewrite_https_img_urls_in_yaml(
    value: &mut serde_yaml::Value,
    bundle_media_root: &Path,
) -> Result<(), ContentDatabaseError> {
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (key, inner) in map.iter_mut() {
                let is_img_url = matches!(key, serde_yaml::Value::String(k) if k == "imgUrl");
                if is_img_url {
                    if let serde_yaml::Value::String(raw) = inner {
                        if raw.starts_with("https://") {
                            let source_url = raw.clone();
                            let ext = guess_extension_from_https_url(&source_url);
                            let mut hasher = Sha256::new();
                            hasher.update(source_url.as_bytes());
                            let hash = hasher.finalize();
                            let digest: String =
                                hash.iter().map(|b| format!("{:02x}", b)).collect();
                            let file_name = format!("remote-{}.{}", &digest[..16], ext);
                            let target = bundle_media_root.join(&file_name);

                            if !target.exists() {
                                if let Some(bytes) = fetch_remote_media(&source_url) {
                                    fs::write(&target, bytes).map_err(|source| {
                                        ContentDatabaseError::WriteFile {
                                            path: target.display().to_string(),
                                            source,
                                        }
                                    })?;
                                } else {
                                    tracing::warn!(
                                        "Could not download remote imgUrl {source_url}, keeping original URL"
                                    );
                                    continue;
                                }
                            }

                            *raw = public_media_url(&file_name);
                        }
                    }
                } else {
                    rewrite_https_img_urls_in_yaml(inner, bundle_media_root)?;
                }
            }
        }
        serde_yaml::Value::Sequence(items) => {
            for item in items {
                rewrite_https_img_urls_in_yaml(item, bundle_media_root)?;
            }
        }
        _ => {}
    }

    Ok(())
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
            let hash = hasher.finalize();
            let digest: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
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

pub fn process_mermaid_codeblocks_for_bundle(
    bundle: &ContentOutputBundle,
    database: &mut ContentDatabase,
) -> Result<(), ContentDatabaseError> {
    process_mermaid_codeblocks_for_bundle_with(bundle, database, render_mermaid_with_renderer)
}

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

    let mut home_yaml: serde_yaml::Value =
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

    // Convert remote `imgUrl: https://...` entries into bundled `/public/media/...` assets.
    rewrite_https_img_urls_in_yaml(&mut home_yaml, &bundle_media_root)?;

    let rewritten =
        serde_yaml::to_string(&home_yaml).map_err(|error| ContentDatabaseError::ParseYaml {
            path: bundle.home_path.display().to_string(),
            message: error.to_string(),
        })?;

    fs::write(&bundle.home_path, rewritten).map_err(|source| ContentDatabaseError::WriteFile {
        path: bundle.home_path.display().to_string(),
        source,
    })?;

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

fn minia_reference_to_site_url(raw: &str) -> Option<&str> {
    raw.strip_prefix("minia#")
}

#[cfg(not(target_arch = "wasm32"))]
fn fetch_favicon(site_url: &str) -> Option<(Vec<u8>, &'static str)> {
    let base = site_url.trim_end_matches('/');

    for (path, ext) in [("/favicon.ico", "ico"), ("/favicon.png", "png")] {
        let url = format!("{base}{path}");
        let response = match ureq::get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .call()
        {
            Ok(response) => response,
            Err(_) => continue,
        };

        if response.status() != 200 {
            continue;
        }

        let mut bytes = Vec::new();
        if response.into_reader().read_to_end(&mut bytes).is_ok()
            && !bytes.is_empty()
            && !bytes.starts_with(b"<!")
            && !bytes.starts_with(b"<h")
        {
            return Some((bytes, ext));
        }
    }

    None
}

#[cfg(target_arch = "wasm32")]
fn fetch_favicon(_site_url: &str) -> Option<(Vec<u8>, &'static str)> {
    None
}

fn rewrite_minia_refs_in_yaml(
    value: &mut serde_yaml::Value,
    bundle_minia_root: &Path,
) -> Result<(), ContentDatabaseError> {
    match value {
        serde_yaml::Value::String(raw) => {
            if let Some(site_url) = minia_reference_to_site_url(raw) {
                let site_url = site_url.to_string();

                let mut hasher = Sha256::new();
                hasher.update(site_url.as_bytes());
                let hash = hasher.finalize();
                let digest: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
                let prefix = &digest[..16];

                let cached_ext = ["ico", "png"]
                    .into_iter()
                    .find(|ext| bundle_minia_root.join(format!("{prefix}.{ext}")).exists());

                let ext = if let Some(ext) = cached_ext {
                    ext
                } else if let Some((bytes, ext)) = fetch_favicon(&site_url) {
                    let path = bundle_minia_root.join(format!("{prefix}.{ext}"));
                    fs::write(&path, bytes).map_err(|source| ContentDatabaseError::WriteFile {
                        path: path.display().to_string(),
                        source,
                    })?;
                    ext
                } else {
                    tracing::warn!("Could not fetch favicon for {site_url}, keeping minia ref");
                    return Ok(());
                };

                *raw = public_minia_url(&format!("{prefix}.{ext}"));
            }
        }
        serde_yaml::Value::Sequence(items) => {
            for item in items {
                rewrite_minia_refs_in_yaml(item, bundle_minia_root)?;
            }
        }
        serde_yaml::Value::Mapping(map) => {
            for value in map.values_mut() {
                rewrite_minia_refs_in_yaml(value, bundle_minia_root)?;
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn process_home_yaml_minia_for_bundle(
    bundle: &ContentOutputBundle,
) -> Result<(), ContentDatabaseError> {
    let minia_root = bundle.public_dir.join("minia");
    fs::create_dir_all(&minia_root).map_err(|source| ContentDatabaseError::CreateDir {
        path: minia_root.display().to_string(),
        source,
    })?;

    let home_content =
        fs::read_to_string(&bundle.home_path).map_err(|source| ContentDatabaseError::ReadFile {
            path: bundle.home_path.display().to_string(),
            source,
        })?;

    let mut home_yaml: serde_yaml::Value =
        serde_yaml::from_str(&home_content).map_err(|error| ContentDatabaseError::ParseYaml {
            path: bundle.home_path.display().to_string(),
            message: error.to_string(),
        })?;

    rewrite_minia_refs_in_yaml(&mut home_yaml, &minia_root)?;

    let rewritten =
        serde_yaml::to_string(&home_yaml).map_err(|error| ContentDatabaseError::ParseYaml {
            path: bundle.home_path.display().to_string(),
            message: error.to_string(),
        })?;

    fs::write(&bundle.home_path, rewritten).map_err(|source| ContentDatabaseError::WriteFile {
        path: bundle.home_path.display().to_string(),
        source,
    })?;

    Ok(())
}
