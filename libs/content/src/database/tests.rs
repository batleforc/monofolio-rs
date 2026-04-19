use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use image::{ImageBuffer, Rgba};

use crate::markdown::{MarkdownContent, MarkdownNode};

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

    assert_eq!(database.blog_timeline.len(), 2);
    assert_eq!(database.blog_timeline[0].handle, "blogs/second");
    assert_eq!(database.blog_timeline[1].handle, "blogs/first");

    fs::remove_dir_all(root).expect("cleanup should succeed");
}

#[test]
fn meta_json_controls_sidebar_order() {
    let root = temporary_directory();
    fs::create_dir_all(root.join("docs")).expect("should create docs");

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

    let docs_node = database.sidebar.iter().find(|item| item.handle == "docs");
    assert!(docs_node.is_some(), "docs node should be in sidebar");
    let children = &docs_node.expect("docs node").children;
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

#[test]
fn creates_output_bundle_with_required_files() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\n",
    )
    .expect("home");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let db = build_content_database(&root).expect("db build");
    create_content_output_bundle(&root, &output, &db).expect("bundle");

    assert!(output.join("db.json").exists());
    assert!(output.join("config.yaml").exists());
    assert!(output.join("home.yaml").exists());
    assert!(output.join("public").is_dir());

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn supports_two_stage_bundle_workflow() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\n",
    )
    .expect("home");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let (db, bundle) = build_content_database_and_prepare_bundle(&root, &output).expect("stage1");

    assert!(bundle.config_path.exists());
    assert!(bundle.home_path.exists());
    assert!(bundle.public_dir.is_dir());
    assert!(!bundle.db_path.exists());

    finalize_content_output_bundle(&bundle, &db).expect("stage2");
    assert!(bundle.db_path.exists());

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn processes_media_references_and_writes_public_media() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\n",
    )
    .expect("home");
    fs::write(root.join("media/test.svg"), "<svg></svg>").expect("media file");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\nimage: media#test.svg\n---\n![Alt](media#test.svg)",
    )
    .expect("doc");

    let (mut db, bundle) =
        build_content_database_and_prepare_bundle(&root, &output).expect("stage1");
    process_markdown_media_for_bundle(&root, &bundle, &mut db).expect("process media");

    let first_entry = db.entries.first().expect("entry");
    assert_eq!(first_entry.image, "/public/media/test.svg");
    assert!(bundle.public_dir.join("media/test.svg").exists());

    let serialized = serde_json::to_string(&first_entry.content).expect("serialize content");
    assert!(serialized.contains("/public/media/test.svg"));

    finalize_content_output_bundle(&bundle, &db).expect("finalize");
    assert!(bundle.db_path.exists());

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn processes_media_references_in_home_yaml_and_writes_public_media() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\nhistory:\n  - title: T\n    lieux: L\n    date: D\n    weight: 1\n    imgUrl: media#timeline.svg\n",
    )
    .expect("home");
    fs::write(root.join("media/timeline.svg"), "<svg></svg>").expect("media file");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let (_db, bundle) = build_content_database_and_prepare_bundle(&root, &output).expect("stage1");
    process_home_yaml_media_for_bundle(&root, &bundle).expect("process home media");

    assert!(bundle.public_dir.join("media/timeline.svg").exists());

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn rewrites_history_img_url_to_40x40_thumbnail_in_bundle() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\nhistory:\n  - title: T\n    lieux: L\n    date: D\n    weight: 1\n    imgUrl: media#timeline.png\n",
    )
    .expect("home");
    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(320, 160, Rgba([255, 0, 0, 255]))
        .save(root.join("media/timeline.png"))
        .expect("write png");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let (_db, bundle) = build_content_database_and_prepare_bundle(&root, &output).expect("stage1");
    process_home_yaml_media_for_bundle(&root, &bundle).expect("process home media");

    let rewritten_home = fs::read_to_string(&bundle.home_path).expect("read rewritten home");
    let home_yaml: serde_yaml::Value = serde_yaml::from_str(&rewritten_home).expect("parse home");
    let img_url = home_yaml
        .get("history")
        .and_then(|v| v.as_sequence())
        .and_then(|v| v.first())
        .and_then(|v| v.get("imgUrl"))
        .and_then(|v| v.as_str())
        .expect("timeline imgUrl");

    assert!(
        img_url.starts_with("media#timeline-40x40-"),
        "history imgUrl should be rewritten to bundled 40x40 timeline asset"
    );

    let relative = img_url.trim_start_matches("media#");
    let thumb_path = bundle.public_dir.join("media").join(relative);
    assert!(
        thumb_path.exists(),
        "timeline thumbnail should exist in bundle"
    );

    let thumb = image::ImageReader::open(&thumb_path)
        .expect("open thumb")
        .decode()
        .expect("decode thumb");
    assert!(thumb.width() <= 40, "thumbnail width should be <= 40");
    assert!(thumb.height() <= 40, "thumbnail height should be <= 40");

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn reuses_already_bundled_remote_home_img_url_assets() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\nusefulLinks:\n  - name: Remote\n    url: https://example.test\n    imgUrl: https://example.test/logo.png\n",
    )
    .expect("home");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let (_db, bundle) = build_content_database_and_prepare_bundle(&root, &output).expect("stage1");

    fs::create_dir_all(bundle.public_dir.join("media")).expect("create bundle media");
    let remote_path = bundle.public_dir.join("media/remote-4feef726406519c3.png");
    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(32, 32, Rgba([0, 255, 0, 255]))
        .save(&remote_path)
        .expect("write cached remote png");
    fs::write(
        &bundle.home_path,
        "name: Test\npresentation: Hi\nshortDescription: SD\nusefulLinks:\n  - name: Remote\n    url: https://example.test\n    imgUrl: /public/media/remote-4feef726406519c3.png\n",
    )
    .expect("rewrite staged home");

    process_home_yaml_media_for_bundle(&root, &bundle).expect("process home media");

    assert!(
        remote_path.exists(),
        "cached remote image should remain available in bundle"
    );

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn rewrites_useful_links_img_url_to_40x40_thumbnail_in_bundle() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\nusefulLinks:\n  - name: Remote\n    url: https://example.test\n    imgUrl: https://example.test/logo.png\n",
    )
    .expect("home");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\n---\nHome",
    )
    .expect("doc");

    let (_db, bundle) = build_content_database_and_prepare_bundle(&root, &output).expect("stage1");

    fs::create_dir_all(bundle.public_dir.join("media")).expect("create bundle media");
    let remote_path = bundle.public_dir.join("media/remote-4feef726406519c3.png");
    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(160, 80, Rgba([0, 255, 0, 255]))
        .save(&remote_path)
        .expect("write cached remote png");
    fs::write(
        &bundle.home_path,
        "name: Test\npresentation: Hi\nshortDescription: SD\nusefulLinks:\n  - name: Remote\n    url: https://example.test\n    imgUrl: /public/media/remote-4feef726406519c3.png\n",
    )
    .expect("rewrite staged home");

    process_home_yaml_media_for_bundle(&root, &bundle).expect("process home media");

    let rewritten_home = fs::read_to_string(&bundle.home_path).expect("read rewritten home");
    let home_yaml: serde_yaml::Value = serde_yaml::from_str(&rewritten_home).expect("parse home");
    let img_url = home_yaml
        .get("usefulLinks")
        .and_then(|v| v.as_sequence())
        .and_then(|v| v.first())
        .and_then(|v| v.get("imgUrl"))
        .and_then(|v| v.as_str())
        .expect("useful link imgUrl");

    assert!(
        img_url.starts_with("media#useful-links-40x40-"),
        "usefulLinks imgUrl should be rewritten to bundled 40x40 useful-links asset"
    );

    let relative = img_url.trim_start_matches("media#");
    let thumb_path = bundle.public_dir.join("media").join(relative);
    assert!(
        thumb_path.exists(),
        "useful-links thumbnail should exist in bundle"
    );

    let thumb = image::ImageReader::open(&thumb_path)
        .expect("open thumb")
        .decode()
        .expect("decode thumb");
    assert!(thumb.width() <= 40, "thumbnail width should be <= 40");
    assert!(thumb.height() <= 40, "thumbnail height should be <= 40");

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn full_workflow_helper_builds_and_finalizes_bundle() {
    let root = temporary_directory();
    let output = temporary_directory();

    fs::create_dir_all(root.join("blogs")).expect("create blogs");
    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::create_dir_all(root.join("media")).expect("create media");
    fs::write(root.join("config.yaml"), "port: 3000\nenv: test\n").expect("config");
    fs::write(
        root.join("home.yaml"),
        "name: Test\npresentation: Hi\nshortDescription: SD\ncoverTitle:\n  - T\nhistory:\n  - title: H\n    lieux: L\n    date: D\n    weight: 1\n    imgUrl: media#home-icon.svg\n",
    )
    .expect("home");
    fs::write(root.join("media/icon.svg"), "<svg></svg>").expect("media file");
    fs::write(root.join("media/home-icon.svg"), "<svg></svg>").expect("home media file");
    fs::write(
        root.join("docs/index.md"),
        "---\ntitle: Docs\nspec:\n  doc: true\nimage: media#icon.svg\n---\n![Alt](media#icon.svg)",
    )
    .expect("doc");
    fs::write(
        root.join("blogs/first.md"),
        "---\ntitle: First post\ndescription: First blog entry\ndate: 2025-01-01T00:00:00Z\ntags:\n  - rust\nspec:\n  blog: true\n---\nHello",
    )
    .expect("blog");

    let (db, bundle) = build_content_database_and_bundle(&root, &output).expect("full workflow");

    assert!(bundle.db_path.exists());
    assert!(bundle.config_path.exists());
    assert!(bundle.home_path.exists());
    assert!(output.join("rss.xml").exists());
    assert!(bundle.public_dir.join("media/icon.svg").exists());
    assert!(bundle.public_dir.join("media/home-icon.svg").exists());
    let docs_entry = db
        .entries
        .iter()
        .find(|entry| entry.handle == "docs")
        .expect("docs entry");
    assert_eq!(docs_entry.image, "/public/media/icon.svg");

    let rss = fs::read_to_string(output.join("rss.xml")).expect("read rss");
    assert!(rss.contains("<title>First post</title>"));
    assert!(rss.contains("<link>/blog/first</link>"));
    assert!(rss.contains("<category>rust</category>"));

    fs::remove_dir_all(root).expect("cleanup root");
    fs::remove_dir_all(output).expect("cleanup output");
}

#[test]
fn rewrites_mermaid_codeblocks_to_svg_images() {
    let output = temporary_directory();
    let bundle = ContentOutputBundle {
        root_dir: output.clone(),
        db_path: output.join("db.json"),
        config_path: output.join("config.yaml"),
        home_path: output.join("home.yaml"),
        public_dir: output.join("public"),
    };
    fs::create_dir_all(&bundle.public_dir).expect("create public");

    let mut database = ContentDatabase {
        generated_at_unix: 0,
        entries: vec![ContentEntry {
            title: "Docs".to_string(),
            description: String::new(),
            handle: "docs/test".to_string(),
            source_path: "docs/test.md".to_string(),
            kind: ContentKind {
                blog: false,
                project: false,
                doc: true,
            },
            dates: ContentDates::default(),
            draft: false,
            tags: vec![],
            techno: vec![],
            minia: None,
            image: String::new(),
            reading_time_minutes: 1,
            toc: vec![],
            content: MarkdownContent {
                format: "markdown_ast".to_string(),
                nodes: vec![MarkdownNode {
                    kind: "code_block".to_string(),
                    text: String::new(),
                    attrs: BTreeMap::from([
                        ("style".to_string(), "fenced".to_string()),
                        ("language".to_string(), "mermaid".to_string()),
                    ]),
                    children: vec![MarkdownNode {
                        kind: "text".to_string(),
                        text: "graph TD; A-->B;".to_string(),
                        attrs: BTreeMap::new(),
                        children: vec![],
                    }],
                }],
            },
        }],
        sidebar: vec![],
        blog_timeline: vec![],
    };

    process_mermaid_codeblocks_for_bundle_with(&bundle, &mut database, |_, output_path| {
        fs::write(output_path, "<svg></svg>").map_err(|source| ContentDatabaseError::WriteFile {
            path: output_path.display().to_string(),
            source,
        })
    })
    .expect("process mermaid");

    let node = &database.entries[0].content.nodes[0];
    assert_eq!(node.kind, "image");
    let url = node.attrs.get("url").expect("url attr");
    assert!(url.starts_with("/public/mermaid/docs-test-"));
    assert!(url.ends_with(".svg"));
    assert!(bundle.public_dir.join("mermaid").is_dir());

    fs::remove_dir_all(output).expect("cleanup");
}
