use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::markdown::{parse_markdown_document, MarkdownContent, MarkdownHeading, MarkdownMeta};

// ---------------------------------------------------------------------------
// meta.json schema (one per directory, optional)
// ---------------------------------------------------------------------------

/// Optional per-directory metadata file (`meta.json`).
///
/// When present in a directory, it controls the display order of children in
/// the sidebar and can override the display title of that section.
///
/// `order` lists the **stem names** (without `.md`) or sub-directory names in
/// the desired display order. Items not listed are appended after the ordered
/// ones, sorted alphabetically. `index` entries are always hoisted first
/// regardless.
fn default_index_is_main_item() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirMeta {
    /// Display title override for this directory section in the sidebar.
    #[serde(default)]
    pub title: Option<String>,
    /// Explicit ordering of children (stem names / sub-dir names).
    #[serde(default)]
    pub order: Vec<String>,
    /// Whether `index.md` should represent the category root itself.
    ///
    /// If `true` (the default, including when `meta.json` is absent),
    /// `docs/foo/index.md` gets the handle `docs/foo`.
    /// If `false`, it keeps its own child handle: `docs/foo/index`.
    #[serde(default = "default_index_is_main_item")]
    pub index_is_main_item: bool,
}

fn load_dir_meta(dir: &Path) -> Option<DirMeta> {
    let path = dir.join("meta.json");
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

#[derive(Debug, Error)]
pub enum ContentDatabaseError {
    #[error("failed to read file {path}: {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read directory {path}: {source}")]
    ReadDir {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("path {path} is outside content root")]
    InvalidContentPath { path: String },
    #[error("failed to serialize content database: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to write file {path}: {source}")]
    WriteFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContentKind {
    pub blog: bool,
    pub project: bool,
    pub doc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContentDates {
    pub created_at: String,
    pub updated_at_unix: u64,
    pub released_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SidebarItem {
    pub title: String,
    pub handle: String,
    /// Explicit position inside the parent (0-based). Items that appear in a
    /// `meta.json` `order` list get a low index; others are appended after
    /// them sorted alphabetically.
    pub order: usize,
    pub children: Vec<SidebarItem>,
}

/// A single entry on the blog timeline (no HTML body, for listing pages).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogTimelineEntry {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub date: String,
    pub tags: Vec<String>,
    pub image: String,
    pub reading_time_minutes: usize,
    pub draft: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentEntry {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub source_path: String,
    pub kind: ContentKind,
    pub dates: ContentDates,
    pub draft: bool,
    pub tags: Vec<String>,
    pub techno: Vec<String>,
    pub image: String,
    pub reading_time_minutes: usize,
    pub toc: Vec<MarkdownHeading>,
    pub content: MarkdownContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDatabase {
    pub generated_at_unix: u64,
    pub entries: Vec<ContentEntry>,
    /// Sidebar tree for the documentation section, ordered according to
    /// directory structure and optional `meta.json` files.
    pub sidebar: Vec<SidebarItem>,
    /// Blog articles sorted chronologically (newest first), without HTML bodies.
    pub blog_timeline: Vec<BlogTimelineEntry>,
}

#[derive(Debug, Default)]
struct SidebarNode {
    title: Option<String>,
    handle: Option<String>,
    /// key → child node
    children: BTreeMap<String, SidebarNode>,
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn slugify_segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    let mut previous_dash = false;

    for ch in segment.chars().flat_map(|c| c.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            out.push('-');
            previous_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn handle_from_relative_path(relative_path: &Path, content_root: &Path) -> String {
    let mut segments: Vec<String> = relative_path
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();

    if let Some(last) = segments.last_mut() {
        if let Some(stripped) = last.strip_suffix(".md") {
            *last = stripped.to_string();
        }
    }

    let is_index_file = segments.last().map(|s| s.eq_ignore_ascii_case("index")) == Some(true);
    let parent_dir = relative_path.parent().unwrap_or_else(|| Path::new(""));
    let index_is_main_item = load_dir_meta(&content_root.join(parent_dir))
        .map(|meta| meta.index_is_main_item)
        .unwrap_or(true);

    if is_index_file && index_is_main_item {
        let _ = segments.pop();
    }

    segments
        .into_iter()
        .map(|segment| slugify_segment(&segment))
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/")
}

fn infer_kind(relative_path: &Path, meta: &MarkdownMeta) -> ContentKind {
    let first_segment = relative_path
        .iter()
        .next()
        .map(|segment| segment.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let has_explicit_spec = meta.spec.blog || meta.spec.project || meta.spec.doc;
    if has_explicit_spec {
        return ContentKind {
            blog: meta.spec.blog,
            project: meta.spec.project,
            doc: meta.spec.doc,
        };
    }

    ContentKind {
        blog: first_segment == "blogs",
        project: relative_path
            .to_string_lossy()
            .to_lowercase()
            .contains("/project/"),
        doc: first_segment == "docs",
    }
}

fn collect_markdown_files(
    root: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), ContentDatabaseError> {
    let entries = fs::read_dir(root).map_err(|source| ContentDatabaseError::ReadDir {
        path: root.display().to_string(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| ContentDatabaseError::ReadDir {
            path: root.display().to_string(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, files)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(path);
        }
    }

    Ok(())
}

fn update_sidebar(sidebar_root: &mut SidebarNode, entry: &ContentEntry) {
    let segments: Vec<&str> = entry.handle.split('/').collect();
    if segments.is_empty() {
        return;
    }

    let mut node = sidebar_root;
    for segment in segments {
        node = node.children.entry(segment.to_string()).or_default();
    }

    node.title = Some(entry.title.clone());
    node.handle = Some(entry.handle.clone());
}

fn segment_to_title(segment: &str) -> String {
    segment
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut out = String::new();
                    out.push(first.to_ascii_uppercase());
                    out.push_str(chars.as_str());
                    out
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert a `SidebarNode` tree into a sorted `Vec<SidebarItem>`.
///
/// `dir_path` is the on-disk directory that corresponds to `node`'s children —
/// used to locate an optional `meta.json` file for custom ordering.
fn sidebar_to_items(
    node: SidebarNode,
    parent_handle: Option<&str>,
    dir_path: Option<&Path>,
) -> Vec<SidebarItem> {
    // Load optional meta.json for this directory level.
    let dir_meta = dir_path.and_then(load_dir_meta);
    let order_list: &[String] = dir_meta.as_ref().map(|m| m.order.as_slice()).unwrap_or(&[]);

    // Build a position map from the meta.json order list.
    let explicit_pos: BTreeMap<&str, usize> = order_list
        .iter()
        .enumerate()
        .map(|(index, key)| (key.as_str(), index))
        .collect();

    let mut items: Vec<SidebarItem> = node
        .children
        .into_iter()
        .map(|(key, child)| {
            let title = child
                .title
                .clone()
                .unwrap_or_else(|| segment_to_title(&key));
            let handle = child.handle.clone().unwrap_or_else(|| match parent_handle {
                Some(parent) if !parent.is_empty() => format!("{parent}/{key}"),
                _ => key.clone(),
            });
            // The child directory lives at <dir_path>/<key> (original casing).
            // We don't have the original un-slugified name here, so we look in
            // dir_path for a child entry whose slug matches `key`.
            let child_dir = dir_path.and_then(|dir| {
                fs::read_dir(dir).ok().and_then(|mut entries| {
                    entries.find_map(|result| {
                        let path = result.ok()?.path();
                        if path.is_dir()
                            && slugify_segment(&path.file_name()?.to_string_lossy()) == key
                        {
                            Some(path)
                        } else {
                            None
                        }
                    })
                })
            });
            let children = sidebar_to_items(child, Some(&handle), child_dir.as_deref());
            let order = explicit_pos
                .get(key.as_str())
                .copied()
                .unwrap_or(usize::MAX);
            SidebarItem {
                title,
                handle,
                order,
                children,
            }
        })
        .collect();

    // Sort: explicit order first (by index), then alphabetically by handle.
    items.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.handle.cmp(&b.handle)));
    items
}

pub fn build_content_database(
    content_root: impl AsRef<Path>,
) -> Result<ContentDatabase, ContentDatabaseError> {
    let content_root = content_root.as_ref();
    let mut markdown_files = Vec::new();
    collect_markdown_files(content_root, &mut markdown_files)?;
    markdown_files.sort();

    let mut entries = Vec::new();
    for file_path in markdown_files {
        let raw =
            fs::read_to_string(&file_path).map_err(|source| ContentDatabaseError::ReadFile {
                path: file_path.display().to_string(),
                source,
            })?;

        let doc = parse_markdown_document(&raw);

        let relative_path = file_path.strip_prefix(content_root).map_err(|_| {
            ContentDatabaseError::InvalidContentPath {
                path: file_path.display().to_string(),
            }
        })?;
        let source_path = relative_path.to_string_lossy().replace('\\', "/");

        let metadata =
            fs::metadata(&file_path).map_err(|source| ContentDatabaseError::ReadFile {
                path: file_path.display().to_string(),
                source,
            })?;
        let updated_at_unix = metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        let kind = infer_kind(relative_path, &doc.meta);
        let dates = ContentDates {
            created_at: doc.meta.date.clone(),
            updated_at_unix,
            released_at: if doc.meta.release {
                doc.meta.date.clone()
            } else {
                String::new()
            },
        };

        entries.push(ContentEntry {
            title: doc.meta.title.clone(),
            description: doc.meta.description.clone(),
            handle: handle_from_relative_path(relative_path, content_root),
            source_path,
            kind,
            dates,
            draft: doc.meta.draft,
            tags: doc.meta.tags.clone(),
            techno: doc.meta.techno.clone(),
            image: doc.meta.image.clone(),
            reading_time_minutes: doc.reading_time_minutes,
            toc: doc.headings,
            content: doc.content,
        });
    }

    entries.sort_by(|left, right| left.handle.cmp(&right.handle));

    // --- Sidebar (doc entries only, ordered by dir structure + meta.json) ---
    let mut sidebar_root = SidebarNode::default();
    for entry in entries.iter().filter(|entry| entry.kind.doc) {
        update_sidebar(&mut sidebar_root, entry);
    }
    // Pass content_root so sidebar_to_items can find meta.json at each level.
    let sidebar = sidebar_to_items(sidebar_root, None, Some(content_root));

    // --- Blog timeline (blog entries, newest first) -------------------------
    let mut blog_timeline: Vec<BlogTimelineEntry> = entries
        .iter()
        .filter(|entry| entry.kind.blog && !entry.draft)
        .map(|entry| BlogTimelineEntry {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            date: entry.dates.created_at.clone(),
            tags: entry.tags.clone(),
            image: entry.image.clone(),
            reading_time_minutes: entry.reading_time_minutes,
            draft: entry.draft,
        })
        .collect();

    // Sort newest first.  ISO-8601 dates are lexicographically comparable.
    blog_timeline.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(ContentDatabase {
        generated_at_unix: now_unix(),
        entries,
        sidebar,
        blog_timeline,
    })
}

pub fn write_content_database_json(
    database: &ContentDatabase,
    output_path: impl AsRef<Path>,
) -> Result<(), ContentDatabaseError> {
    let output_path = output_path.as_ref();
    let payload = serde_json::to_vec_pretty(database)?;
    fs::write(output_path, payload).map_err(|source| ContentDatabaseError::WriteFile {
        path: output_path.display().to_string(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn temporary_directory() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("content-db-test-{nanos}"))
    }

    #[test]
    fn index_files_generate_parent_handle() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("docs/Techno")).expect("should create docs tree");
        let handle = handle_from_relative_path(Path::new("docs/Techno/index.md"), &root);
        assert_eq!(handle, "docs/techno");
        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn index_files_keep_index_handle_when_meta_disables_main_item() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("docs/Guide")).expect("should create docs tree");
        fs::write(
            root.join("docs/Guide/meta.json"),
            r#"{"index_is_main_item": false}"#,
        )
        .expect("should write meta.json");

        let handle = handle_from_relative_path(Path::new("docs/Guide/index.md"), &root);
        assert_eq!(handle, "docs/guide/index");

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn builds_database_sidebar_and_timeline() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("docs/Guide")).expect("should create docs tree");
        fs::create_dir_all(root.join("blogs")).expect("should create blog tree");

        fs::write(
            root.join("docs/index.md"),
            "---\ntitle: Docs\nspec:\n  doc: true\n---\n# Docs",
        )
        .expect("should write docs index");
        fs::write(
            root.join("docs/Guide/intro.md"),
            "---\ntitle: Intro\nspec:\n  doc: true\n---\n## Intro",
        )
        .expect("should write intro page");
        fs::write(
            root.join("blogs/first.md"),
            "---\ntitle: First\ndate: 2024-01-01T00:00:00Z\nspec:\n  blog: true\n---\nHello",
        )
        .expect("should write first blog");
        fs::write(
            root.join("blogs/second.md"),
            "---\ntitle: Second\ndate: 2025-06-01T00:00:00Z\nspec:\n  blog: true\n---\nWorld",
        )
        .expect("should write second blog");

        let database = build_content_database(&root).expect("database build should work");
        assert_eq!(database.entries.len(), 4);
        assert!(database
            .entries
            .iter()
            .any(|entry| entry.handle == "docs" && entry.kind.doc));
        assert!(database
            .entries
            .iter()
            .any(|entry| entry.handle == "blogs/first" && entry.kind.blog));
        assert!(!database.sidebar.is_empty());

        // Timeline: newest first
        assert_eq!(database.blog_timeline.len(), 2);
        assert_eq!(database.blog_timeline[0].handle, "blogs/second");
        assert_eq!(database.blog_timeline[1].handle, "blogs/first");

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn meta_json_controls_sidebar_order() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("docs")).expect("should create docs");

        // Write three pages; meta.json enforces beta before alpha.
        fs::write(
            root.join("docs/alpha.md"),
            "---\ntitle: Alpha\nspec:\n  doc: true\n---",
        )
        .expect("alpha");
        fs::write(
            root.join("docs/beta.md"),
            "---\ntitle: Beta\nspec:\n  doc: true\n---",
        )
        .expect("beta");
        fs::write(
            root.join("docs/gamma.md"),
            "---\ntitle: Gamma\nspec:\n  doc: true\n---",
        )
        .expect("gamma");
        fs::write(
            root.join("docs/meta.json"),
            r#"{"order": ["beta", "alpha", "gamma"]}"#,
        )
        .expect("meta.json");

        let database = build_content_database(&root).expect("build");

        // Find the docs node children (the sidebar root wraps docs/).
        let docs_node = database.sidebar.iter().find(|item| item.handle == "docs");
        assert!(docs_node.is_some(), "docs node should be in sidebar");
        let children = &docs_node.unwrap().children;
        assert_eq!(children[0].handle, "docs/beta");
        assert_eq!(children[1].handle, "docs/alpha");
        assert_eq!(children[2].handle, "docs/gamma");

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn meta_json_can_keep_index_as_child_page() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("docs/Guide")).expect("should create docs tree");

        fs::write(
            root.join("docs/Guide/index.md"),
            "---\ntitle: Guide home\nspec:\n  doc: true\n---",
        )
        .expect("index");
        fs::write(
            root.join("docs/Guide/intro.md"),
            "---\ntitle: Intro\nspec:\n  doc: true\n---",
        )
        .expect("intro");
        fs::write(
            root.join("docs/Guide/meta.json"),
            r#"{"index_is_main_item": false, "order": ["index", "intro"]}"#,
        )
        .expect("meta");

        let database = build_content_database(&root).expect("build");
        let guide_node = database
            .sidebar
            .iter()
            .find(|item| item.handle == "docs")
            .and_then(|docs| {
                docs.children
                    .iter()
                    .find(|item| item.handle == "docs/guide")
            });

        assert!(guide_node.is_some(), "guide node should be present");
        let guide_children = &guide_node.expect("guide node").children;
        assert_eq!(guide_children[0].handle, "docs/guide/index");
        assert_eq!(guide_children[1].handle, "docs/guide/intro");

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn draft_blog_posts_excluded_from_timeline() {
        let root = temporary_directory();
        fs::create_dir_all(root.join("blogs")).expect("create blogs");

        fs::write(
            root.join("blogs/published.md"),
            "---\ntitle: Published\ndate: 2025-01-01T00:00:00Z\nspec:\n  blog: true\n---\nContent",
        )
        .expect("published");
        fs::write(
            root.join("blogs/draft.md"),
            "---\ntitle: Draft\ndraft: true\ndate: 2025-06-01T00:00:00Z\nspec:\n  blog: true\n---\nDraft content",
        )
        .expect("draft");

        let database = build_content_database(&root).expect("build");
        assert_eq!(database.blog_timeline.len(), 1);
        assert_eq!(database.blog_timeline[0].handle, "blogs/published");
        fs::remove_dir_all(root).expect("cleanup");
    }
}
