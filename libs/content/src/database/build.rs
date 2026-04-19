use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::markdown::{parse_markdown_document, MarkdownMeta};

use super::types::SidebarNode;
use super::{
    load_dir_meta, BlogTimelineEntry, ContentDatabase, ContentDatabaseError, ContentDates,
    ContentEntry, ContentKind, SidebarItem,
};

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

pub(crate) fn handle_from_relative_path(relative_path: &Path, content_root: &Path) -> String {
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

fn git_history_dates(content_root: &Path, relative_path: &Path) -> Option<(String, String, u64)> {
    let rel = relative_path.to_string_lossy().replace('\\', "/");

    let output = Command::new("git")
        .arg("-C")
        .arg(content_root)
        .arg("log")
        .arg("--follow")
        .arg("--format=%ct|%aI")
        .arg("--")
        .arg(&rel)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut lines = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.splitn(2, '|');
            let unix = parts.next().unwrap_or_default().trim();
            let iso = parts.next().unwrap_or_default().trim();
            (unix.to_string(), iso.to_string())
        })
        .filter(|(_, iso)| !iso.is_empty())
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return None;
    }

    let latest = lines.remove(0);
    let oldest_iso = lines
        .last()
        .map(|(_, iso)| iso.clone())
        .unwrap_or_else(|| latest.1.clone());
    let latest_unix = latest.0.parse::<u64>().ok().unwrap_or(0);

    Some((oldest_iso, latest.1, latest_unix))
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

fn sidebar_to_items(
    node: SidebarNode,
    parent_handle: Option<&str>,
    dir_path: Option<&Path>,
) -> Vec<SidebarItem> {
    let dir_meta = dir_path.and_then(load_dir_meta);
    let order_list: &[String] = dir_meta.as_ref().map(|m| m.order.as_slice()).unwrap_or(&[]);

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
        let file_updated_at_unix = metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        let git_dates = git_history_dates(content_root, relative_path);
        let created_at = if !doc.meta.date.trim().is_empty() {
            doc.meta.date.clone()
        } else {
            git_dates
                .as_ref()
                .map(|(oldest, _, _)| oldest.clone())
                .unwrap_or_default()
        };
        let updated_at = git_dates
            .as_ref()
            .map(|(_, latest, _)| latest.clone())
            .unwrap_or_default();
        let updated_at_unix = git_dates
            .as_ref()
            .map(|(_, _, latest_unix)| *latest_unix)
            .filter(|unix| *unix > 0)
            .unwrap_or(file_updated_at_unix);

        let kind = infer_kind(relative_path, &doc.meta);
        let dates = ContentDates {
            created_at: created_at.clone(),
            updated_at,
            updated_at_unix,
            released_at: if doc.meta.release {
                if !doc.meta.date.trim().is_empty() {
                    doc.meta.date.clone()
                } else {
                    created_at
                }
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
            minia: None,
            reading_time_minutes: doc.reading_time_minutes,
            toc: doc.headings,
            content: doc.content,
        });
    }

    entries.sort_by(|left, right| left.handle.cmp(&right.handle));

    let mut sidebar_root = SidebarNode::default();
    for entry in entries.iter().filter(|entry| entry.kind.doc) {
        update_sidebar(&mut sidebar_root, entry);
    }
    let sidebar = sidebar_to_items(sidebar_root, None, Some(content_root));

    let mut blog_timeline: Vec<BlogTimelineEntry> = entries
        .iter()
        .filter(|entry| {
            entry.kind.blog && !entry.draft && !entry.dates.released_at.trim().is_empty()
        })
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
