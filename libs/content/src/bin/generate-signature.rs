use std::{borrow::Cow, fs::File, path::PathBuf};

use content::HomeConfig;
use takumi::{
    layout::{
        node::Node,
        style::{
            AlignItems, ColorInput, Display, FlexDirection, FromCss, JustifyContent,
            Length::{Percentage, Px},
            ObjectFit, Style, StyleDeclaration,
        },
        Viewport,
    },
    rendering::{render, write_image, ImageOutputFormat, RenderOptions},
    resources::{font::FontResource, image::ImageSource},
    GlobalContext,
};

const INTER_FONT: &[u8] = include_bytes!("../../assets/fonts/Inter-Variable.ttf");

// ── Palette (apps/frontend/style/main.css) ────────────────────────────────────
const BG: &str = "oklch(0.07 0.018 18)";
const FG: &str = "oklch(0.93 0.018 25)";
const MUTED: &str = "oklch(0.58 0.04 30)";
const PRIMARY: &str = "oklch(0.65 0.26 25)";

fn parse_bg(input: &str) -> ColorInput<false> {
    ColorInput::<false>::from_str(input).expect("invalid bg color")
}

fn parse_fg(input: &str) -> ColorInput<true> {
    ColorInput::<true>::from_str(input).expect("invalid fg color")
}

fn parse_args() -> (PathBuf, PathBuf) {
    let mut args = std::env::args().skip(1);
    let mut home_yaml = PathBuf::from("contents/home.yaml");
    let mut output = PathBuf::from("signature.webp");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--home-yaml" => {
                if let Some(value) = args.next() {
                    home_yaml = PathBuf::from(value);
                }
            }
            "--output" => {
                if let Some(value) = args.next() {
                    output = PathBuf::from(value);
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: generate-signature [--home-yaml <path>] [--output <path>]\n\
                     \nDefaults:\
                     \n  --home-yaml  contents/home.yaml\
                     \n  --output     signature.webp"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {other}. Use --help for usage.");
                std::process::exit(1);
            }
        }
    }

    (home_yaml, output)
}

/// Loads the portfolio logo from the standard frontend public path if it exists.
fn portfolio_logo_path() -> Option<PathBuf> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../apps/frontend/public/web-app-manifest-512x512.png");
    if path.exists() {
        Some(path)
    } else {
        eprintln!(
            "Warning: portfolio logo not found at {}, signature will be rendered without it",
            path.display()
        );
        None
    }
}

fn render_signature(
    home: &HomeConfig,
    output_path: &PathBuf,
    ctx: &mut GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // ── Palette ───────────────────────────────────────────────────────────
    let bg = parse_bg(BG);
    let fg = parse_fg(FG);
    let muted = parse_fg(MUTED);
    let primary_text = parse_fg(PRIMARY);
    let primary_bg = parse_bg(PRIMARY);
    let dark = parse_fg(BG);

    // ── Accent bar (narrow vertical strip — replaces the left padding) ────
    let accent_bar = Node::container([]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::width(Px(8.0)))
            .with(StyleDeclaration::height(Percentage(100.0)))
            .with(StyleDeclaration::background_color(primary_bg.clone())),
    );
    // The content wrapper is 800 - 8 = 792 px so accent_bar + wrapper = 800px.
    const CONTENT_WIDTH: f32 = 792.0;

    // ── Branding: site url (top of text panel, like portfolio_name in miniature) ─
    let portfolio_name_node = Node::text("maxleriche.net".to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(muted.clone()))
            .with(StyleDeclaration::font_size(Px(14.0).into())),
    );

    // ── Middle: section label + name + short description ──────────────────
    let section_node = Node::text("CONTACT".to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(primary_text))
            .with(StyleDeclaration::font_size(Px(14.0).into())),
    );

    let name_node = Node::text(home.name.clone()).with_style(
        Style::default()
            .with(StyleDeclaration::color(fg))
            .with(StyleDeclaration::font_size(Px(30.0).into())),
    );

    let desc_node = Node::text(home.short_description.clone()).with_style(
        Style::default()
            .with(StyleDeclaration::color(muted.clone()))
            .with(StyleDeclaration::font_size(Px(16.0).into())),
    );

    let middle_node = Node::container([section_node, name_node, desc_node]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Column))
            .with(StyleDeclaration::align_items(AlignItems::FlexStart)),
    );

    // ── Bottom: contact info (email · location) + CTA button ─────────────
    // Keep contact_str short: only email + location to avoid text overflow.
    let mut contact_parts: Vec<String> = Vec::new();
    if let Some(ref email) = home.contact_email {
        contact_parts.push(email.clone());
    }
    if let Some(ref location) = home.contact_location {
        contact_parts.push(location.clone());
    }
    let contact_str = contact_parts.join("  ·  ");

    let contact_node = Node::text(contact_str).with_style(
        Style::default()
            .with(StyleDeclaration::color(muted.clone()))
            .with(StyleDeclaration::font_size(Px(13.0).into())),
    );

    // Optional current job on a second line below contact info.
    let job_str = home.current_work.clone().unwrap_or_default();
    let job_node = Node::text(job_str).with_style(
        Style::default()
            .with(StyleDeclaration::color(muted.clone()))
            .with(StyleDeclaration::font_size(Px(12.0).into())),
    );

    let contact_stack = Node::container([contact_node, job_node]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Column))
            .with(StyleDeclaration::align_items(AlignItems::FlexStart)),
    );

    // CTA button — identical style to render_miniature_core.
    let cta_label = Node::text("Discover →".to_string()).with_style(
        Style::default()
            .with(StyleDeclaration::color(dark))
            .with(StyleDeclaration::font_size(Px(14.0).into())),
    );

    let cta_button = Node::container([cta_label]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::background_color(primary_bg))
            .with(StyleDeclaration::padding_left(Px(18.0)))
            .with(StyleDeclaration::padding_right(Px(18.0)))
            .with(StyleDeclaration::padding_top(Px(8.0)))
            .with(StyleDeclaration::padding_bottom(Px(8.0)))
            .with(StyleDeclaration::align_items(AlignItems::Center))
            .with(StyleDeclaration::justify_content(JustifyContent::Center)),
    );

    // CTA sits on its own line below contact info — no side-by-side overflow risk.
    let bottom_section = Node::container([contact_stack, cta_button]).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Column))
            .with(StyleDeclaration::align_items(AlignItems::FlexStart)),
    );

    // ── Text panel (62% of content area, mirroring the miniature ratio) ───
    let text_panel = Node::container([portfolio_name_node, middle_node, bottom_section])
        .with_style(
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

    // ── Image panel (38% of content area) ────────────────────────────────
    let mut content_children = vec![text_panel];

    if let Some(logo_path) = portfolio_logo_path() {
        let image_key = logo_path.to_string_lossy().to_string();
        let bytes = match std::fs::read(&logo_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Warning: could not read logo: {e}");
                return Ok(());
            }
        };
        let source = match ImageSource::from_bytes(&bytes) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Warning: could not decode logo: {e}");
                return Ok(());
            }
        };
        ctx.persistent_image_store.insert(image_key.clone(), source);

        let image_node = Node::image(image_key).with_style(
            Style::default()
                .with(StyleDeclaration::display(Display::Flex))
                .with(StyleDeclaration::width(Percentage(100.0)))
                .with(StyleDeclaration::height(Percentage(100.0)))
                .with(StyleDeclaration::object_fit(ObjectFit::Contain)),
        );

        let image_panel = Node::container([image_node]).with_style(
            Style::default()
                .with(StyleDeclaration::display(Display::Flex))
                .with(StyleDeclaration::width(Percentage(38.0)))
                .with(StyleDeclaration::height(Percentage(100.0))),
        );
        content_children.push(image_panel);
    }

    // Content wrapper: holds text + image, with padding. The accent bar sits
    // outside so it spans the full height without being affected by padding.
    let content_wrapper = Node::container(content_children).with_style(
        Style::default()
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Row))
            .with(StyleDeclaration::justify_content(JustifyContent::FlexStart))
            .with(StyleDeclaration::align_items(AlignItems::Center))
            .with(StyleDeclaration::padding_left(Px(20.0)))
            .with(StyleDeclaration::padding_right(Px(28.0)))
            .with(StyleDeclaration::padding_top(Px(20.0)))
            .with(StyleDeclaration::padding_bottom(Px(20.0)))
            .with(StyleDeclaration::width(Px(CONTENT_WIDTH)))
            .with(StyleDeclaration::height(Percentage(100.0))),
    );

    // ── Root container (700×200, same bg as miniature) ────────────────────
    let root = Node::container([accent_bar, content_wrapper]).with_style(
        Style::default()
            .with(StyleDeclaration::width(Px(800.0)))
            .with(StyleDeclaration::height(Px(220.0)))
            .with(StyleDeclaration::background_color(bg))
            .with(StyleDeclaration::display(Display::Flex))
            .with(StyleDeclaration::flex_direction(FlexDirection::Row))
            .with(StyleDeclaration::justify_content(JustifyContent::FlexStart))
            .with(StyleDeclaration::align_items(AlignItems::Stretch)),
    );

    // ── Render ────────────────────────────────────────────────────────────
    let options = RenderOptions::builder()
        .viewport(Viewport::new((800, 220)))
        .node(root)
        .global(ctx)
        .build();

    let image = render(options)?;

    let mut file = File::create(output_path)?;
    write_image(Cow::Owned(image), &mut file, ImageOutputFormat::WebP, None)?;

    Ok(())
}

fn main() {
    let (home_yaml_path, output_path) = parse_args();

    let home_content = match std::fs::read_to_string(&home_yaml_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {e}", home_yaml_path.display());
            std::process::exit(1);
        }
    };

    let home: HomeConfig = match serde_yaml::from_str(&home_content) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to parse {}: {e}", home_yaml_path.display());
            std::process::exit(1);
        }
    };

    let mut ctx = GlobalContext::default();
    let _ = ctx
        .font_context
        .load_and_store(FontResource::new(INTER_FONT));

    match render_signature(&home, &output_path, &mut ctx) {
        Ok(()) => println!("Signature generated: {}", output_path.display()),
        Err(e) => {
            eprintln!("Render failed: {e}");
            std::process::exit(1);
        }
    }
}
