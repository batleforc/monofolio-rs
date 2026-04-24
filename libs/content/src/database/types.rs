use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::markdown::{MarkdownContent, MarkdownHeading};

fn default_index_is_main_item() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirMeta {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub order: Vec<String>,
    #[serde(default = "default_index_is_main_item")]
    pub index_is_main_item: bool,
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
    #[error("failed to create directory {path}: {source}")]
    CreateDir {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to copy file from {from} to {to}: {source}")]
    CopyFile {
        from: String,
        to: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to decode image {path}: {message}")]
    DecodeImage { path: String, message: String },
    #[error("failed to encode image to {path}: {message}")]
    EncodeImage { path: String, message: String },
    #[error("failed to render mermaid diagram to {path}: {message}")]
    MermaidRender { path: String, message: String },
    #[error("failed to render miniature to {path}: {message}")]
    MiniatureRender { path: String, message: String },
    #[error("failed to parse yaml file {path}: {message}")]
    ParseYaml { path: String, message: String },
    #[error("failed to fetch favicon for {url}: {message}")]
    FetchFavicon { url: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContentKind {
    pub blog: bool,
    pub project: bool,
    pub doc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContentDates {
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub updated_at_unix: u64,
    #[serde(default)]
    pub released_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SidebarItem {
    pub title: String,
    pub handle: String,
    pub order: usize,
    pub children: Vec<SidebarItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogTimelineEntry {
    pub title: String,
    pub description: String,
    pub handle: String,
    pub date: String,
    pub tags: Vec<String>,
    pub image: String,
    pub minia: Option<String>,
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
    pub minia: Option<String>,
    pub reading_time_minutes: usize,
    pub toc: Vec<MarkdownHeading>,
    pub content: MarkdownContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentDatabase {
    pub generated_at_unix: u64,
    pub entries: Vec<ContentEntry>,
    pub sidebar: Vec<SidebarItem>,
    pub blog_timeline: Vec<BlogTimelineEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentOutputBundle {
    pub root_dir: PathBuf,
    pub db_path: PathBuf,
    pub config_path: PathBuf,
    pub home_path: PathBuf,
    pub public_dir: PathBuf,
}

#[derive(Debug, Default)]
pub(super) struct SidebarNode {
    pub title: Option<String>,
    pub handle: Option<String>,
    pub children: std::collections::BTreeMap<String, SidebarNode>,
}
