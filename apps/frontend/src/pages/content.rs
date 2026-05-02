use std::collections::HashSet;
#[cfg(feature = "ssr")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "ssr")]
use content::{ContentDatabase, ContentEntry, SidebarItem};
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::markdown::toc::{extract_headings, TableOfContents};
use crate::components::markdown::{MarkdownContent, MarkdownFromValue};
use crate::components::ui::{Card, SectionTitle, TagBadge};
use crate::date_utils::format_display_date;
use crate::i18n::use_language;
use crate::seo::{canonical_url, DEFAULT_OG_IMAGE};
use crate::services::api::fetch_json;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
struct DocSidebarItemData {
    title: String,
    handle: String,
    order: u64,
    kind: String,
    children: Vec<DocSidebarItemData>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
struct PageData {
    title: String,
    description: String,
    handle: String,
    source_path: String,
    created_at: String,
    updated_at: String,
    blog: bool,
    project: bool,
    doc: bool,
    tags: Vec<String>,
    techno: Vec<String>,
    image: String,
    minia: Option<String>,
    reading_time_minutes: usize,
    content: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HeaderImageSource {
    Image(String),
    Icomoon(String),
}

#[allow(dead_code)]
async fn load_page_data(_handle: String) -> Option<PageData> {
    #[cfg(not(feature = "ssr"))]
    {
        let handle = _handle;
        let endpoint = format!("/api/v1/page/{}", handle.trim_start_matches('/'));
        fetch_json(&endpoint).await
    }
    #[cfg(feature = "ssr")]
    {
        let _ = &_handle;
        None
    }
}

#[allow(dead_code)]
async fn load_docs_nav() -> Vec<DocSidebarItemData> {
    fetch_json("/api/v1/nav/doc").await.unwrap_or_default()
}

#[cfg(not(feature = "ssr"))]
async fn load_page_data_send_safe(handle: String) -> Option<PageData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Option<PageData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_page_data(handle).await);
    });
    match rx.await {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("oneshot receiver canceled in send-safe fetch: {err}");
            None
        }
    }
}

#[cfg(not(feature = "ssr"))]
async fn load_docs_nav_send_safe() -> Vec<DocSidebarItemData> {
    let (tx, rx) = futures::channel::oneshot::channel::<Vec<DocSidebarItemData>>();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(load_docs_nav().await);
    });
    match rx.await {
        Ok(data) => data,
        Err(err) => {
            tracing::warn!("oneshot receiver canceled in send-safe fetch: {err}");
            Vec::new()
        }
    }
}

#[cfg(feature = "ssr")]
impl From<&ContentEntry> for PageData {
    fn from(entry: &ContentEntry) -> Self {
        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            handle: entry.handle.clone(),
            source_path: entry.source_path.clone(),
            created_at: entry.dates.created_at.clone(),
            updated_at: entry.dates.updated_at.clone(),
            blog: entry.kind.blog,
            project: entry.kind.project,
            doc: entry.kind.doc,
            tags: entry.tags.clone(),
            techno: entry.techno.clone(),
            image: entry.image.clone(),
            minia: entry.minia.clone(),
            reading_time_minutes: entry.reading_time_minutes,
            content: serde_json::to_value(&entry.content).unwrap_or(serde_json::Value::Null),
        }
    }
}

fn resolve_header_image(raw: &str) -> Option<HeaderImageSource> {
    if raw.trim().is_empty() {
        return None;
    }
    if raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with('/') {
        return Some(HeaderImageSource::Image(raw.to_string()));
    }
    if let Some(file_name) = raw.strip_prefix("media#") {
        return Some(HeaderImageSource::Image(format!(
            "/public/media/{file_name}"
        )));
    }
    if let Some(file_name) = raw.strip_prefix("icomoon#") {
        return Some(HeaderImageSource::Icomoon(format!(
            "/assets/icon/symbol-defs.svg#ico-{file_name}"
        )));
    }
    Some(HeaderImageSource::Image(format!(
        "/public/media/{}",
        raw.trim_start_matches('/')
    )))
}

fn render_header_image(image: HeaderImageSource) -> impl IntoView {
    match image {
        HeaderImageSource::Image(src) => view! {
            <img
                src=src
                alt=""
                class="w-full h-52 md:h-72 object-cover rounded border border-border mb-4"
                loading="lazy"
            />
        }
            .into_any(),
        HeaderImageSource::Icomoon(href) => view! {
            <div class="w-full h-52 md:h-72 rounded border border-border mb-4 bg-muted/30 flex items-center justify-center p-6 md:p-8">
                <svg class="w-full h-full" aria-hidden="true" focusable="false" fill="#fff">
                    <use href=href></use>
                </svg>
            </div>
        }
            .into_any(),
    }
}

fn slugify_like_handle(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut previous_dash = false;

    for ch in value.chars().flat_map(|c| c.to_lowercase()) {
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

fn flatten_doc_index(items: &[DocSidebarItemData], out: &mut Vec<(String, String)>) {
    for item in items {
        if item.kind != "folder" {
            out.push((item.title.clone(), item.handle.clone()));
        }
        flatten_doc_index(&item.children, out);
    }
}

fn find_doc_href_for_techno(techno: &str, docs_index: &[(String, String)]) -> Option<String> {
    let needle = slugify_like_handle(techno);
    if needle.is_empty() {
        return None;
    }

    docs_index.iter().find_map(|(title, handle)| {
        let title_slug = slugify_like_handle(title);
        let handle_slug = handle
            .split('/')
            .next_back()
            .map(slugify_like_handle)
            .unwrap_or_default();

        if needle == title_slug || needle == handle_slug {
            Some(format!("/{}", handle.trim_start_matches('/')))
        } else {
            None
        }
    })
}

fn edit_in_git_url(source_path: &str) -> String {
    format!(
        "https://github.com/batleforc/monofolio-rs/edit/main/contents/{}",
        source_path.trim_start_matches('/')
    )
}

fn current_year_utc() -> i32 {
    #[cfg(not(feature = "ssr"))]
    {
        return js_sys::Date::new_0().get_utc_full_year() as i32;
    }

    #[cfg(feature = "ssr")]
    {
        let unix_days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64 / 86_400)
            .unwrap_or(0);
        // Purement de l'ia, je jure, j'ai galéré a trouver un truc simple et systématiquement il me le remplace par ça
        let z = unix_days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let m = mp + if mp < 10 { 3 } else { -9 };
        (y + if m <= 2 { 1 } else { 0 }) as i32
    }
}

#[cfg(feature = "ssr")]
fn map_sidebar_item(item: &SidebarItem, known_doc_handles: &HashSet<String>) -> DocSidebarItemData {
    let kind = if item.children.is_empty() {
        "markdown".to_string()
    } else if known_doc_handles.contains(&item.handle) {
        "folder_with_index".to_string()
    } else {
        "folder".to_string()
    };
    DocSidebarItemData {
        title: item.title.clone(),
        handle: item.handle.clone(),
        order: item.order as u64,
        kind,
        children: item
            .children
            .iter()
            .map(|child| map_sidebar_item(child, known_doc_handles))
            .collect(),
    }
}

#[cfg(feature = "ssr")]
fn is_nav_visible(entry: &ContentEntry) -> bool {
    !entry.draft && !entry.dates.released_at.trim().is_empty()
}

#[cfg(feature = "ssr")]
fn map_sidebar_item_filtered(
    item: &SidebarItem,
    known_doc_handles: &HashSet<String>,
) -> Option<DocSidebarItemData> {
    let mut mapped = map_sidebar_item(item, known_doc_handles);
    mapped.children = item
        .children
        .iter()
        .filter_map(|child| map_sidebar_item_filtered(child, known_doc_handles))
        .collect();

    if known_doc_handles.contains(&mapped.handle) || !mapped.children.is_empty() {
        Some(mapped)
    } else {
        None
    }
}

fn render_doc_sidebar_tree(
    items: Vec<DocSidebarItemData>,
    active_handle: String,
    depth: usize,
    expanded_folders: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let mut sorted = items;
    sorted.sort_by(|a, b| a.order.cmp(&b.order));

    view! {
        <ul class="flex flex-col gap-1.5">
            {sorted
                .into_iter()
                .map(|item| {
                    let indent = match depth {
                        0 => "",
                        1 => "ms-2",
                        2 => "ms-4",
                        3 => "ms-6",
                        _ => "ms-8",
                    };
                    let href = format!("/{}", item.handle.trim_start_matches('/'));
                    let has_children = !item.children.is_empty();
                    let children = item.children.clone();
                    let is_folder = item.kind == "folder";
                    let is_folder_with_index = item.kind == "folder_with_index";
                    let is_folder_like = is_folder || is_folder_with_index;
                    let is_active = item.handle == active_handle;
                    let item_title = item.title.clone();
                    let handle = item.handle.clone();
                    let handle_for_toggle = handle.clone();
                    let handle_for_open = handle.clone();
                    let active_handle_next = active_handle.clone();

                    view! {
                        <li class=indent>
                            <div class="flex items-center gap-2">
                                {if is_folder_like && has_children {
                                    view! {
                                        <button
                                            type="button"
                                            class="inline-flex h-5 w-5 items-center justify-center rounded border border-border text-muted-foreground hover:text-foreground hover:border-foreground/30 transition-colors"
                                            on:click=move |_| {
                                                if depth == 0 {
                                                    return;
                                                }
                                                expanded_folders
                                                    .update(|open| {
                                                        if open.contains(&handle_for_toggle) {
                                                            open.remove(&handle_for_toggle);
                                                        } else {
                                                            open.insert(handle_for_toggle.clone());
                                                        }
                                                    });
                                            }
                                        >
                                            <span aria-hidden="true">
                                                {move || {
                                                    if depth == 0
                                                        || expanded_folders
                                                            .with(|open| open.contains(&handle_for_open))
                                                    {
                                                        "▾"
                                                    } else {
                                                        "▸"
                                                    }
                                                }}
                                            </span>
                                        </button>
                                    }
                                        .into_any()
                                } else {
                                    view! { <span class="inline-flex h-5 w-5"></span> }.into_any()
                                }}
                                {if is_folder_with_index {
                                    view! {
                                        <a
                                            href=href
                                            class=if is_active {
                                                "text-sm font-semibold text-primary underline"
                                            } else {
                                                "text-sm font-semibold text-foreground hover:text-primary transition-colors"
                                            }
                                        >
                                            {item_title}
                                        </a>
                                    }
                                        .into_any()
                                } else if is_folder {
                                    view! {
                                        <span class="text-xs uppercase tracking-wide text-muted-foreground">
                                            {item_title}
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <a
                                            href=href
                                            class=if is_active {
                                                "text-sm font-semibold text-primary underline"
                                            } else {
                                                "text-sm font-medium text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                            }
                                        >
                                            {item_title}
                                        </a>
                                    }
                                        .into_any()
                                }}
                            </div>

                            {move || {
                                let is_open = depth == 0
                                    || expanded_folders.with(|open| open.contains(&handle));
                                if has_children && is_open {
                                    view! {
                                        <div class="mt-2">
                                            {render_doc_sidebar_tree(
                                                children.clone(),
                                                active_handle_next.clone(),
                                                depth + 1,
                                                expanded_folders,
                                            )}
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }
                            }}
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[allow(dead_code)]
fn collect_active_path_folders(
    items: &[DocSidebarItemData],
    active_handle: &str,
    out: &mut HashSet<String>,
) -> bool {
    for item in items {
        let self_matches = item.handle == active_handle;
        let child_matches = collect_active_path_folders(&item.children, active_handle, out);
        if self_matches || child_matches {
            if !item.children.is_empty() {
                out.insert(item.handle.clone());
            }
            return true;
        }
    }
    false
}

#[component]
pub fn ContentHandlePage() -> impl IntoView {
    let location = use_location();
    let lang = use_language();
    #[cfg(feature = "ssr")]
    let content_db = use_context::<ContentDatabase>();

    let page_data = Resource::new(move || location.pathname.get(), {
        #[cfg(feature = "ssr")]
        let content_db = content_db.clone();
        move |pathname: String| {
            #[cfg(feature = "ssr")]
            let content_db = content_db.clone();
            async move {
                if !pathname.starts_with("/blogs/") && !pathname.starts_with("/docs/") {
                    return None;
                }
                let handle = pathname.trim_start_matches('/').to_string();
                #[cfg(not(feature = "ssr"))]
                {
                    load_page_data_send_safe(handle).await
                }
                #[cfg(feature = "ssr")]
                {
                    content_db.and_then(|db| {
                        db.entries
                            .iter()
                            .find(|entry| entry.handle == handle && is_nav_visible(entry))
                            .map(PageData::from)
                    })
                }
            }
        }
    });
    let docs_nav: Resource<Vec<DocSidebarItemData>> =
        Resource::new(move || location.pathname.get(), {
            #[cfg(feature = "ssr")]
            let content_db = content_db.clone();
            move |pathname: String| {
                #[cfg(feature = "ssr")]
                let content_db = content_db.clone();
                async move {
                    let _ = pathname;
                    #[cfg(not(feature = "ssr"))]
                    {
                        load_docs_nav_send_safe().await
                    }
                    #[cfg(feature = "ssr")]
                    {
                        content_db
                            .map(|db| {
                                let known_doc_handles: HashSet<String> = db
                                    .entries
                                    .iter()
                                    .filter(|entry| entry.kind.doc && is_nav_visible(entry))
                                    .map(|entry| entry.handle.clone())
                                    .collect();
                                db.sidebar
                                    .iter()
                                    .filter_map(|item| {
                                        map_sidebar_item_filtered(item, &known_doc_handles)
                                    })
                                    .collect::<Vec<DocSidebarItemData>>()
                            })
                            .unwrap_or_default()
                    }
                }
            }
        });
    let expanded_folders: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());
    Effect::new(move |_| {
        let handle = location.pathname.get().trim_start_matches('/').to_string();
        let nav_data = docs_nav.get().unwrap_or_default();
        if !handle.starts_with("docs/") && handle != "docs" {
            expanded_folders.set(HashSet::new());
            return;
        }

        let mut open = HashSet::new();
        collect_active_path_folders(&nav_data, &handle, &mut open);
        expanded_folders.set(open);
    });

    view! {
        <section class="min-h-[calc(100svh-3.5rem)] cyber-grid-bg">
            <div class="max-w-7xl mx-auto px-5 py-16">
                <Suspense fallback=move || {
                    view! {
                        <Card class="p-5">
                            <p class="text-sm text-muted-foreground">"Loading page..."</p>
                        </Card>
                    }
                }>
                    <Show when=move || {
                        page_data.get().is_some() && page_data.get().flatten().is_none()
                    }>
                        <Card class="p-5">
                            <p class="text-sm text-muted-foreground">"Unable to load this page."</p>
                            <a
                                href="/projects"
                                class="inline-flex mt-3 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                            >
                                "Back to projects"
                            </a>
                        </Card>
                    </Show>

                    <Show when=move || {
                        page_data.get().flatten().is_some()
                    }>
                        {move || {
                            page_data
                                .get()
                                .flatten()
                                .map(|page| {
                                    let PageData {
                                        title,
                                        description,
                                        handle,
                                        source_path,
                                        created_at,
                                        updated_at,
                                        blog,
                                        project,
                                        doc,
                                        tags,
                                        techno,
                                        image,
                                        reading_time_minutes,
                                        content,
                                        minia,
                                    } = page;
                                    let kind_label = if blog {
                                        "blog"
                                    } else if project {
                                        "project"
                                    } else if doc {
                                        "doc"
                                    } else {
                                        "content"
                                    };
                                    let docs_index = {
                                        let mut out = Vec::new();
                                        let nav = docs_nav.get().unwrap_or_default();
                                        flatten_doc_index(&nav, &mut out);
                                        out
                                    };
                                    let is_blog = blog;
                                    let is_doc = doc;
                                    let back_href = if is_blog { "/blog" } else { "/projects" };
                                    let back_label = if is_blog {
                                        "Back to blog"
                                    } else {
                                        "Back to projects"
                                    };
                                    let header_image = resolve_header_image(&image);
                                    let edit_url = edit_in_git_url(&source_path);
                                    let footer_note = format!(
                                        "Since 2000 - {} with love and coffee",
                                        current_year_utc(),
                                    );
                                    let created_at_raw_doc = created_at.clone();
                                    let updated_at_raw_doc = updated_at.clone();
                                    let created_at_raw_default = created_at.clone();
                                    let updated_at_raw_default = updated_at.clone();
                                    let page_title = format!("{title} | Maxime Leriche");
                                    let page_description = if description.trim().is_empty() {
                                        "Page de contenu de Maxime Leriche".to_string()
                                    } else {
                                        description.clone()
                                    };
                                    let canonical = canonical_url(&handle);
                                    let image_url = if image.trim().is_empty() {
                                        DEFAULT_OG_IMAGE.to_string()
                                    } else if image.starts_with("https://") {
                                        image
                                    } else if image.starts_with("http://") {
                                        format!("https://{}", image.trim_start_matches("http://"))
                                    } else if let Some(file_name) = image.strip_prefix("media#") {
                                        canonical_url(&format!("/media/{file_name}"))
                                    } else {
                                        canonical_url(
                                            &format!("/media/{}", image.trim_start_matches('/')),
                                        )
                                    };

                                    view! {
                                        <Title text=page_title.clone() />
                                        <Meta name="description" content=page_description.clone() />
                                        <Link rel="canonical" href=canonical.clone() />
                                        <Meta
                                            property="og:type"
                                            content=if is_blog { "article" } else { "website" }
                                        />
                                        <Meta property="og:title" content=page_title.clone() />
                                        <Meta
                                            property="og:description"
                                            content=page_description.clone()
                                        />
                                        <Meta property="og:url" content=canonical.clone() />
                                        <Meta
                                            property="og:image"
                                            content=minia.clone().unwrap_or(image_url.clone())
                                        />
                                        <Meta name="twitter:card" content="summary_large_image" />
                                        <Meta name="twitter:title" content=page_title.clone() />
                                        <Meta
                                            name="twitter:description"
                                            content=page_description.clone()
                                        />
                                        <Meta name="twitter:image" content=image_url.clone() />
                                        <SectionTitle>{title.clone()}</SectionTitle>

                                        {if is_doc {
                                            let handle_for_sidebar = handle.clone();
                                            let handle_for_mobile_sidebar = handle.clone();
                                            let toc_entries = serde_json::from_value::<
                                                MarkdownContent,
                                            >(content.clone())
                                                .map(|markdown| extract_headings(&markdown.nodes))
                                                .unwrap_or_default();
                                            let has_toc_entries = !toc_entries.is_empty();
                                            view! {
                                                <div class="lg:hidden sticky top-14 z-40 mb-4">
                                                    <Card class="p-2 border-border/80 bg-background/95 backdrop-blur-md">
                                                        <div class="grid grid-cols-2 gap-2">
                                                            <details>
                                                                <summary class="list-none cursor-pointer select-none rounded border border-border px-3 py-2 text-xs font-mono uppercase tracking-widest text-muted-foreground hover:text-foreground hover:border-primary/40 transition-colors">
                                                                    "Navigation"
                                                                </summary>
                                                                <div class="mt-2 rounded border border-border p-3 max-h-[55svh] overflow-auto">
                                                                    {move || {
                                                                        render_doc_sidebar_tree(
                                                                                docs_nav.get().unwrap_or_default(),
                                                                                handle_for_mobile_sidebar.clone(),
                                                                                0,
                                                                                expanded_folders,
                                                                            )
                                                                            .into_any()
                                                                    }}
                                                                </div>
                                                            </details>

                                                            {if has_toc_entries {
                                                                view! {
                                                                    <details>
                                                                        <summary class="list-none cursor-pointer select-none rounded border border-border px-3 py-2 text-xs font-mono uppercase tracking-widest text-muted-foreground hover:text-foreground hover:border-primary/40 transition-colors">
                                                                            "Sommaire"
                                                                        </summary>
                                                                        <div class="mt-2 rounded border border-border p-3 max-h-[55svh] overflow-auto">
                                                                            <TableOfContents entries=toc_entries.clone() />
                                                                        </div>
                                                                    </details>
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                view! {
                                                                    <div class="rounded border border-dashed border-border/70 px-3 py-2 text-xs text-muted-foreground">
                                                                        "Pas de sommaire"
                                                                    </div>
                                                                }
                                                                    .into_any()
                                                            }}
                                                        </div>
                                                    </Card>
                                                </div>

                                                <div class="grid grid-cols-1 lg:grid-cols-[18rem_1fr] gap-5">
                                                    <aside class="lg:sticky lg:top-16 self-start">
                                                        <Card class="p-4 max-h-[calc(100svh-6rem)] overflow-auto">
                                                            <p class="text-xs uppercase tracking-widest font-mono text-muted-foreground mb-3">
                                                                "Docs navigation"
                                                            </p>
                                                            {move || {
                                                                render_doc_sidebar_tree(
                                                                        docs_nav.get().unwrap_or_default(),
                                                                        handle_for_sidebar.clone(),
                                                                        0,
                                                                        expanded_folders,
                                                                    )
                                                                    .into_any()
                                                            }}
                                                        </Card>
                                                    </aside>

                                                    <Card class="p-5">
                                                        {header_image.clone().map(render_header_image)}
                                                        <p class="text-xs text-muted-foreground uppercase tracking-widest font-mono mb-2">
                                                            {kind_label}
                                                        </p>
                                                        <p class="text-base md:text-lg font-medium text-foreground mb-2">
                                                            {description.clone()}
                                                        </p>
                                                        <div class="flex flex-wrap gap-x-4 gap-y-1 mb-3 text-xs text-muted-foreground">
                                                            {move || {
                                                                let created_at_display = format_display_date(
                                                                    &created_at_raw_doc,
                                                                    lang.get(),
                                                                );
                                                                if !created_at_display.is_empty() {
                                                                    view! {
                                                                        <span>{format!("Published: {}", created_at_display)}</span>
                                                                    }
                                                                        .into_any()
                                                                } else {
                                                                    view! { <></> }.into_any()
                                                                }
                                                            }}
                                                            {move || {
                                                                let updated_at_display = format_display_date(
                                                                    &updated_at_raw_doc,
                                                                    lang.get(),
                                                                );
                                                                if !updated_at_display.is_empty() {
                                                                    view! {
                                                                        <span>{format!("Updated: {}", updated_at_display)}</span>
                                                                    }
                                                                        .into_any()
                                                                } else {
                                                                    view! { <></> }.into_any()
                                                                }
                                                            }}
                                                        </div>
                                                        <div class="flex flex-wrap gap-1.5 mb-3">
                                                            {techno
                                                                .clone()
                                                                .into_iter()
                                                                .map(|item| {
                                                                    let doc_href = find_doc_href_for_techno(&item, &docs_index);
                                                                    view! {
                                                                        {if let Some(href) = doc_href {
                                                                            view! {
                                                                                <a
                                                                                    href=href
                                                                                    class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground hover:text-foreground hover:border-primary/50 transition-colors underline-offset-2 hover:underline"
                                                                                >
                                                                                    {item}
                                                                                </a>
                                                                            }
                                                                                .into_any()
                                                                        } else {
                                                                            view! {
                                                                                <span class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground">
                                                                                    {item}
                                                                                </span>
                                                                            }
                                                                                .into_any()
                                                                        }}
                                                                    }
                                                                })
                                                                .collect_view()}
                                                            {tags
                                                                .clone()
                                                                .into_iter()
                                                                .map(|item| {
                                                                    view! {
                                                                        <TagBadge class="border-primary/30 text-primary">
                                                                            {format!("#{}", item)}
                                                                        </TagBadge>
                                                                    }
                                                                })
                                                                .collect_view()}
                                                        </div>
                                                        <p class="text-xs text-muted-foreground">
                                                            {format!("Reading time: {} min", reading_time_minutes)}
                                                        </p> <div class="mt-5 border-t border-border pt-4">
                                                            <MarkdownFromValue value=content.clone() />
                                                        </div>
                                                        <a
                                                            href=back_href
                                                            class="inline-flex mt-4 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                        >
                                                            {back_label}
                                                        </a>
                                                        <a
                                                            href=edit_url.clone()
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                            class="inline-flex mt-2 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                        >
                                                            "Edit in Git"
                                                        </a>
                                                        <p class="mt-6 text-xs text-muted-foreground">
                                                            {footer_note.clone()}
                                                        </p>
                                                    </Card>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <Card class="p-5">
                                                    {header_image.map(render_header_image)}
                                                    <p class="text-xs text-muted-foreground uppercase tracking-widest font-mono mb-3">
                                                        {kind_label}
                                                    </p>
                                                    <p class="text-base md:text-lg font-medium text-foreground mb-2">
                                                        {description}
                                                    </p>
                                                    <div class="flex flex-wrap gap-x-4 gap-y-1 mb-3 text-xs text-muted-foreground">
                                                        {move || {
                                                            let created_at_display = format_display_date(
                                                                &created_at_raw_default,
                                                                lang.get(),
                                                            );
                                                            if !created_at_display.is_empty() {
                                                                view! {
                                                                    <span>{format!("Published: {}", created_at_display)}</span>
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                view! { <></> }.into_any()
                                                            }
                                                        }}
                                                        {move || {
                                                            let updated_at_display = format_display_date(
                                                                &updated_at_raw_default,
                                                                lang.get(),
                                                            );
                                                            if !updated_at_display.is_empty() {
                                                                view! {
                                                                    <span>{format!("Updated: {}", updated_at_display)}</span>
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                view! { <></> }.into_any()
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="flex flex-wrap gap-1.5 mb-3">
                                                        {techno
                                                            .into_iter()
                                                            .map(|item| {
                                                                let doc_href = find_doc_href_for_techno(&item, &docs_index);
                                                                view! {
                                                                    {if let Some(href) = doc_href {
                                                                        view! {
                                                                            <a
                                                                                href=href
                                                                                class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground hover:text-foreground hover:border-primary/50 transition-colors underline-offset-2 hover:underline"
                                                                            >
                                                                                {item}
                                                                            </a>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! {
                                                                            <span class="inline-flex items-center px-2 py-0.5 rounded border border-border text-[0.7rem] text-muted-foreground">
                                                                                {item}
                                                                            </span>
                                                                        }
                                                                            .into_any()
                                                                    }}
                                                                }
                                                            })
                                                            .collect_view()}
                                                        {tags
                                                            .into_iter()
                                                            .map(|item| {
                                                                view! {
                                                                    <TagBadge class="border-primary/30 text-primary">
                                                                        {format!("#{}", item)}
                                                                    </TagBadge>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>
                                                    <p class="text-xs text-muted-foreground">
                                                        {format!("Reading time: {} min", reading_time_minutes)}
                                                    </p>
                                                    {if is_blog {
                                                        view! {
                                                            <div class="mt-5 border-t border-border pt-4">
                                                                <MarkdownFromValue value=content />
                                                            </div>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }}
                                                    <a
                                                        href=back_href
                                                        class="inline-flex mt-4 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                    >
                                                        {back_label}
                                                    </a>
                                                    <a
                                                        href=edit_url
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="inline-flex mt-2 text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
                                                    >
                                                        "Edit in Git"
                                                    </a>
                                                    <p class="mt-6 text-xs text-muted-foreground">
                                                        {footer_note}
                                                    </p>
                                                </Card>
                                            }
                                                .into_any()
                                        }}
                                    }
                                })
                        }}
                    </Show>
                </Suspense>
            </div>
        </section>
    }
}
