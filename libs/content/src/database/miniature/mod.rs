use std::{
    borrow::Cow,
    fs::{self, File},
    path::{Path, PathBuf},
};

use takumi::{
    layout::{
        node::Node,
        style::{
            AlignItems, ColorInput, Display, Filters, FlexDirection, FromCss, JustifyContent,
            Length::{Percentage, Px},
            ObjectFit, Style, StyleDeclaration,
        },
        Viewport,
    },
    rendering::{render, write_image, ImageOutputFormat, RenderOptions},
    resources::{font::FontResource, image::ImageSource},
    GlobalContext,
};

use crate::{
    database::public_minia_url, home::HomeConfig, ContentDatabase, ContentDatabaseError,
    ContentEntry, ContentOutputBundle,
};

const INTER_FONT: &[u8] = include_bytes!("../../../assets/fonts/Inter-Variable.ttf");

#[derive(Clone)]
pub enum MiniatureImageKind {
    IcomoonSymbol(String, PathBuf),
    Relative(PathBuf),
}

impl MiniatureImageKind {
    pub fn to_string(&self) -> String {
        match self {
            MiniatureImageKind::Relative(path) => path.to_string_lossy().to_string(),
            MiniatureImageKind::IcomoonSymbol(symbol, path) => {
                format!("{}#{}", path.display(), symbol)
            }
        }
    }
}

/// Crée un [`GlobalContext`] Takumi avec la police Inter chargée.
pub fn build_miniature_context() -> GlobalContext {
    let mut context = GlobalContext::default();
    let _ = context
        .font_context
        .load_and_store(FontResource::new(INTER_FONT));
    context
}

fn miniature_file_name(entry: &ContentEntry) -> String {
    format!("{}.webp", entry.handle.replace('/', "_"))
}

fn parse_bg_color(
    input: &str,
    output_path: &Path,
) -> Result<ColorInput<false>, ContentDatabaseError> {
    ColorInput::<false>::from_str(input).map_err(|_| ContentDatabaseError::MiniatureRender {
        path: output_path.display().to_string(),
        message: format!("invalid color: {input}"),
    })
}

fn parse_text_color(
    input: &str,
    output_path: &Path,
) -> Result<ColorInput<true>, ContentDatabaseError> {
    ColorInput::<true>::from_str(input).map_err(|_| ContentDatabaseError::MiniatureRender {
        path: output_path.display().to_string(),
        message: format!("invalid color: {input}"),
    })
}

fn resolve_entry_image_path(entry: &ContentEntry, public_dir: &Path) -> Option<MiniatureImageKind> {
    let raw = entry.image.trim();
    if raw.is_empty() || raw.starts_with("http://") || raw.starts_with("https://") {
        return None;
    }

    if raw.starts_with("icomoon#") {
        let symbol = raw
            .strip_prefix("icomoon#")
            .map(str::trim)
            .filter(|value| !value.is_empty())?
            .to_ascii_lowercase();

        let icomoon_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/frontend/public/icon/symbol-defs.svg");
        if icomoon_path.exists() {
            return Some(MiniatureImageKind::IcomoonSymbol(symbol, icomoon_path));
        } else {
            println!(
                "Warning: icomoon svg not found at {}, miniature will be generated without it",
                icomoon_path.display()
            );
            return None;
        }
    }

    let relative = raw
        .strip_prefix("/public/media/")
        .or_else(|| raw.strip_prefix("media#"))
        .or_else(|| raw.strip_prefix("media/"));

    let relative = relative?;
    let candidate = public_dir.join("media").join(relative);
    if candidate.exists() {
        if is_svg_path(&candidate) {
            // SVG page images are not supported by the miniature rendering pipeline.
            None
        } else {
            Some(MiniatureImageKind::Relative(candidate))
        }
    } else {
        None
    }
}

fn is_svg_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("svg"))
        .unwrap_or(false)
}

fn svg_white_filter(output_path: &Path) -> Result<Filters, ContentDatabaseError> {
    Filters::from_str("brightness(0) invert(1)").map_err(|_| {
        ContentDatabaseError::MiniatureRender {
            path: output_path.display().to_string(),
            message: "invalid svg white filter".to_string(),
        }
    })
}

pub fn process_takumi_page_miniature(
    bundle: &ContentOutputBundle,
    database: &mut ContentDatabase,
    global_context: &mut GlobalContext,
) -> Result<(), ContentDatabaseError> {
    let minia_dir = bundle.public_dir.join("minia");
    fs::create_dir_all(&minia_dir).map_err(|source| ContentDatabaseError::CreateDir {
        path: minia_dir.display().to_string(),
        source,
    })?;

    for entry in &mut database.entries {
        let miniature_name = miniature_file_name(entry);
        let miniature_path = minia_dir.join(&miniature_name);
        generate_miniature_for_entry(entry, &miniature_path, &bundle.public_dir, global_context)?;
        entry.minia = Some(public_minia_url(&miniature_name));
    }
    Ok(())
}

/// Slice de handle pour extraire la section principale (ex: "docs/cicd/..." → "CICD").
fn doc_section_label(entry: &ContentEntry) -> String {
    entry
        .handle
        .split('/')
        .nth(1)
        .unwrap_or("doc")
        .to_uppercase()
}

/// Core miniature renderer shared between entry-based and static-page minias.
fn render_miniature_core(
    portfolio_name: &str,
    section: &str,
    title: &str,
    description: &str,
    cta_text: &str,
    image: Option<MiniatureImageKind>,
    output_path: &Path,
    global_context: &mut GlobalContext,
) -> Result<(), ContentDatabaseError> {
    // Palette issue du theme du site (apps/frontend/style/main.css)
    let bg = parse_bg_color("oklch(0.07 0.018 18)", output_path)?;
    let text = parse_text_color("oklch(0.93 0.018 25)", output_path)?;
    let muted = parse_text_color("oklch(0.58 0.04 30)", output_path)?;
    let primary = parse_text_color("oklch(0.65 0.26 25)", output_path)?;
    let primary_bg = parse_bg_color("oklch(0.65 0.26 25)", output_path)?;
    let dark = parse_text_color("oklch(0.07 0.018 18)", output_path)?;

    // ── Top branding ─────────────────────────────────────────────────────
    let portfolio_name_node = Node::text(portfolio_name.to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(muted.clone()))
            .with(StyleDeclaration::font_size(Px(22.0).into())),
    );

    // ── Middle: section label + title + description ───────────────────────
    let mut middle_children = vec![];
    if !section.trim().is_empty() {
        let section_node = Node::text(section.to_string()).with_style(
            Style::default()
                .with(StyleDeclaration::color(primary))
                .with(StyleDeclaration::font_size(Px(28.0).into())),
        );
        middle_children.push(section_node);
    }

    let title_node = Node::text(title.to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(text.clone()))
            .with(StyleDeclaration::font_size(Px(56.0).into())),
    );
    middle_children.push(title_node);

    if !description.trim().is_empty() {
        let desc_node = Node::text(description.to_string()).with_style(
            Style::default()
                .with(StyleDeclaration::color(muted.clone()))
                .with(StyleDeclaration::font_size(Px(30.0).into())),
        );
        middle_children.push(desc_node);
    }

    let content_node = Node::container(middle_children).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Column))
            .with(StyleDeclaration::align_items(AlignItems::FlexStart)),
    );

    // ── Bottom: CTA button ────────────────────────────────────────────────
    let cta_label = Node::text(cta_text.to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(dark))
            .with(StyleDeclaration::font_size(Px(22.0).into())),
    );

    let cta_button = Node::container([cta_label]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::background_color(primary_bg))
            .with(StyleDeclaration::padding_left(Px(28.0)))
            .with(StyleDeclaration::padding_right(Px(28.0)))
            .with(StyleDeclaration::padding_top(Px(12.0)))
            .with(StyleDeclaration::padding_bottom(Px(12.0)))
            .with(StyleDeclaration::align_items(AlignItems::Center))
            .with(StyleDeclaration::justify_content(JustifyContent::Center)),
    );

    // ── Text panel: SpaceBetween to anchor branding/CTA at edges ─────────
    let text_panel = Node::container([portfolio_name_node, content_node, cta_button]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Column))
            .with(StyleDeclaration::justify_content(
                JustifyContent::SpaceBetween,
            ))
            .with(StyleDeclaration::align_items(AlignItems::FlexStart))
            .with(StyleDeclaration::width(Percentage(62.0)))
            .with(StyleDeclaration::height(Percentage(100.0))),
    );

    let mut root_children = vec![text_panel];
    if let Some(image_path) = image {
        let image_key = match &image_path {
            MiniatureImageKind::Relative(path) => {
                preload_image_resource(path, global_context)?;
                image_path.to_string()
            }
            MiniatureImageKind::IcomoonSymbol(symbol_id, svg_path) => {
                preload_icomoon_symbol(svg_path, symbol_id, global_context)?
            }
        };

        let mut image_style = Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::width(Percentage(100.0)))
            .with(StyleDeclaration::height(Percentage(100.0)));

        if let MiniatureImageKind::IcomoonSymbol(_, _) = image_path {
            // SVG used as logos: force them to white for readability.
            image_style = image_style
                .with(StyleDeclaration::object_fit(ObjectFit::Contain))
                .with(StyleDeclaration::filter(svg_white_filter(output_path)?));
        } else {
            // Most article visuals are logos: `contain` avoids aggressive cropping.
            image_style = image_style.with(StyleDeclaration::object_fit(ObjectFit::Contain));
        }

        let image_node = Node::image(image_key).with_style(image_style);

        let image_panel = Node::container([image_node]).with_style(
            Style::default()
                .with(StyleDeclaration::display(Display::Flex))
                .with(StyleDeclaration::width(Percentage(38.0)))
                .with(StyleDeclaration::height(Percentage(100.0))),
        );
        root_children.push(image_panel);
    }

    let root = Node::container(root_children).with_style(
        Style::default()
            .with(StyleDeclaration::width(Px(1200.0)))
            .with(StyleDeclaration::height(Px(630.0)))
            .with(StyleDeclaration::background_color(bg))
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Row))
            .with(StyleDeclaration::justify_content(JustifyContent::Center))
            .with(StyleDeclaration::align_items(AlignItems::Center))
            .with(StyleDeclaration::padding_left(Px(64.0)))
            .with(StyleDeclaration::padding_right(Px(64.0)))
            .with(StyleDeclaration::padding_top(Px(60.0)))
            .with(StyleDeclaration::padding_bottom(Px(60.0))),
    );

    let options = RenderOptions::builder()
        .viewport(Viewport::new((1200, 630)))
        .node(root)
        .global(global_context)
        .build();

    let image = render(options).map_err(|error| ContentDatabaseError::MiniatureRender {
        path: output_path.display().to_string(),
        message: error.to_string(),
    })?;

    let mut file = File::create(output_path).map_err(|source| ContentDatabaseError::WriteFile {
        path: output_path.display().to_string(),
        source,
    })?;

    write_image(Cow::Owned(image), &mut file, ImageOutputFormat::WebP, None).map_err(|error| {
        ContentDatabaseError::EncodeImage {
            path: output_path.display().to_string(),
            message: error.to_string(),
        }
    })?;
    Ok(())
}

pub fn generate_miniature_for_entry(
    entry: &ContentEntry,
    output_path: impl AsRef<std::path::Path>,
    public_dir: &Path,
    global_context: &mut GlobalContext,
) -> Result<(), ContentDatabaseError> {
    let output_path: &Path = output_path.as_ref();
    let image = resolve_entry_image_path(entry, public_dir).or_else(portfolio_logo_path);
    render_miniature_core(
        "Maxime Leriche",
        &doc_section_label(entry),
        &entry.title,
        &entry.description,
        "Read more →",
        image,
        output_path,
        global_context,
    )
}

/// A static (non-database) page that needs an OpenGraph miniature.
struct StaticPageDef {
    /// File slug used for the output filename (e.g. `"home"` → `home.webp`).
    slug: &'static str,
    /// Section label shown in the primary accent colour.
    section: &'static str,
    /// Main title text.
    title: String,
    /// Subtitle / description text.
    description: String,
    /// CTA button label.
    cta: &'static str,
}

pub fn process_static_pages_miniatures(
    bundle: &ContentOutputBundle,
    global_context: &mut GlobalContext,
) -> Result<(), ContentDatabaseError> {
    // Load home.yaml to get the author name and tagline.
    let home_content =
        fs::read_to_string(&bundle.home_path).map_err(|source| ContentDatabaseError::ReadFile {
            path: bundle.home_path.display().to_string(),
            source,
        })?;
    let home: HomeConfig =
        serde_yaml::from_str(&home_content).map_err(|error| ContentDatabaseError::ParseYaml {
            path: bundle.home_path.display().to_string(),
            message: error.to_string(),
        })?;

    let pages: Vec<StaticPageDef> = vec![
        StaticPageDef {
            slug: "home",
            section: "PORTFOLIO",
            title: home.name.clone(),
            description: home.short_description.clone(),
            cta: "Discover →",
        },
        StaticPageDef {
            slug: "about",
            section: "ABOUT",
            title: home.name.clone(),
            description: home.short_description.clone(),
            cta: "Learn more →",
        },
        StaticPageDef {
            slug: "blog",
            section: "BLOG",
            title: "Articles".to_string(),
            description: String::new(),
            cta: "Read articles →",
        },
        StaticPageDef {
            slug: "docs",
            section: "DOCS",
            title: "Documentation".to_string(),
            description: String::new(),
            cta: "Browse docs →",
        },
        StaticPageDef {
            slug: "projects",
            section: "PROJECTS",
            title: "Projects".to_string(),
            description: String::new(),
            cta: "View projects →",
        },
        StaticPageDef {
            slug: "contact",
            section: "CONTACT",
            title: "Get in touch".to_string(),
            description: home
                .contact_email
                .clone()
                .unwrap_or_else(|| home.name.clone()),
            cta: "Say hello →",
        },
    ];

    let minia_dir = bundle.public_dir.join("minia");
    fs::create_dir_all(&minia_dir).map_err(|source| ContentDatabaseError::CreateDir {
        path: minia_dir.display().to_string(),
        source,
    })?;

    let logo = portfolio_logo_path();

    for page in &pages {
        let file_name = format!("{}.webp", page.slug);
        let output_path = minia_dir.join(&file_name);
        render_miniature_core(
            &home.name,
            page.section,
            &page.title,
            &page.description,
            page.cta,
            logo.clone(),
            &output_path,
            global_context,
        )?;
    }
    Ok(())
}

fn portfolio_logo_path() -> Option<MiniatureImageKind> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../apps/frontend/public/web-app-manifest-512x512.png");
    if path.exists() {
        Some(MiniatureImageKind::Relative(path))
    } else {
        println!(
            "Warning: portfolio logo not found at {}, miniature will be generated without it",
            path.display()
        );
        None
    }
}

/// Extracts a symbol from an icomoon SVG and creates a standalone SVG wrapper.
fn extract_icomoon_symbol(
    svg_path: &Path,
    symbol_id: &str,
) -> Result<Vec<u8>, ContentDatabaseError> {
    let symbol_id = symbol_id.trim().trim_start_matches('#');
    let symbol_id = if symbol_id.starts_with("ico-") {
        symbol_id.to_string()
    } else {
        format!("ico-{symbol_id}")
    };

    let svg_content =
        fs::read_to_string(svg_path).map_err(|source| ContentDatabaseError::ReadFile {
            path: svg_path.display().to_string(),
            source,
        })?;

    // Rust regex does not support JS-style /.../s delimiters.
    // Use inline (?s) so '.' also matches newlines.
    let pattern = format!(
        r#"(?s)<symbol\b[^>]*\bid\s*=\s*"{}"[^>]*>(.*?)</symbol>"#,
        regex::escape(&symbol_id)
    );
    let re = regex::Regex::new(&pattern).map_err(|err| ContentDatabaseError::MiniatureRender {
        path: svg_path.display().to_string(),
        message: format!(
            "failed to parse icomoon symbol: {} with pattern: {}. Error: {}",
            symbol_id, pattern, err
        ),
    })?;

    let symbol_match =
        re.captures(&svg_content)
            .ok_or_else(|| ContentDatabaseError::MiniatureRender {
                path: svg_path.display().to_string(),
                message: format!(
                    "symbol '{}' not found in icomoon SVG with pattern: {}",
                    symbol_id, pattern
                ),
            })?;

    let symbol_content = symbol_match.get(1).unwrap().as_str();

    // Extract viewBox attribute
    let view_box_pattern = format!(
        r#"<symbol\b[^>]*\bid\s*=\s*"{}"[^>]*>"#,
        regex::escape(&symbol_id)
    );
    let view_box_re = regex::Regex::new(&view_box_pattern).map_err(|_| {
        ContentDatabaseError::MiniatureRender {
            path: svg_path.display().to_string(),
            message: "failed to extract viewBox from symbol".to_string(),
        }
    })?;

    let symbol_open_tag = view_box_re
        .captures(&svg_content)
        .and_then(|cap| cap.get(0))
        .map(|m| m.as_str())
        .unwrap_or("");

    let view_box = regex::Regex::new(r#"(?i)\bviewBox\s*=\s*"([^"]+)""#)
        .ok()
        .and_then(|re| re.captures(symbol_open_tag))
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("0 0 24 24");

    // Create standalone SVG
    let standalone_svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">{}</svg>"#,
        view_box, symbol_content
    );

    Ok(standalone_svg.into_bytes())
}

fn preload_image_resource(
    image_path: &Path,
    global_context: &mut GlobalContext,
) -> Result<(), ContentDatabaseError> {
    let key = image_path.to_string_lossy().to_string();
    if global_context.persistent_image_store.get(&key).is_some() {
        return Ok(());
    }

    let bytes = fs::read(image_path).map_err(|source| ContentDatabaseError::ReadFile {
        path: image_path.display().to_string(),
        source,
    })?;

    let image =
        ImageSource::from_bytes(&bytes).map_err(|error| ContentDatabaseError::DecodeImage {
            path: image_path.display().to_string(),
            message: error.to_string(),
        })?;

    global_context.persistent_image_store.insert(key, image);
    Ok(())
}

fn preload_icomoon_symbol(
    svg_path: &Path,
    symbol_id: &str,
    global_context: &mut GlobalContext,
) -> Result<String, ContentDatabaseError> {
    // Create a unique key for this symbol
    let key = format!("{}#{}", svg_path.display(), symbol_id);

    if global_context.persistent_image_store.get(&key).is_some() {
        return Ok(key);
    }

    // Extract the symbol from the icomoon SVG
    let svg_bytes = extract_icomoon_symbol(svg_path, symbol_id)?;

    // Load it as an image
    let image =
        ImageSource::from_bytes(&svg_bytes).map_err(|error| ContentDatabaseError::DecodeImage {
            path: svg_path.display().to_string(),
            message: error.to_string(),
        })?;

    global_context
        .persistent_image_store
        .insert(key.clone(), image);
    Ok(key)
}
