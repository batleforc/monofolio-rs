# Copilot instructions for this repository

## Build, test, and lint commands

- Prefer the repo `task` targets over ad-hoc commands:
  - `task lint` runs `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings`.
  - `task test` runs `cargo test --workspace --exclude frontend`.
  - `task check` runs workspace checks plus `cargo check --package frontend --features ssr` and `cargo check --package frontend --features hydrate --target wasm32-unknown-unknown`.
  - `task build` builds the SSR server with `cargo build --package frontend --features ssr`.
  - `task dev` runs `cargo leptos watch --project frontend`.
  - `task gen` runs both `task gen:shiki` and `task gen:content`.
- Content and asset generation matter for local runs:
  - `task gen:content` runs `cargo run --package content --bin content-build` and `cargo run --package content --bin generate-signature -- --output ./target/content-build/public/minia/signature.png`.
  - `task gen:shiki` runs `pnpm run langs:codeblock && pnpm run build:shiki`.
  - Standalone JS helpers: `pnpm run build:shiki`, `pnpm run watch:shiki`, `pnpm run langs:codeblock`.
- Single-test examples:
  - `cargo test -p content meta_json_controls_sidebar_order`
  - `cargo test -p api get_page_returns_200_when_found`
  - `cargo test -p frontend --features ssr sitemap_contains_static_and_content_urls`
- Current repo caveat: `task check` currently reaches a failing hydrate check on `wasm32-unknown-unknown` because Tokio/Mio pulls in an unsupported WASM net path. Do not assume that failure came from your change unless you touched hydration dependencies or target features.

## High-level architecture

- This is a Rust workspace with four members:
  - `apps/frontend`: Leptos app plus the Actix SSR server entrypoint.
  - `libs/content`: content ingestion/build pipeline.
  - `libs/api`: typed Actix handlers under `/api/v1`.
  - `libs/trace`: tracing/OpenTelemetry setup used by the SSR server.
- `contents/` is the source of truth for site content:
  - `contents/home.yaml` drives the home/about/contact data.
  - `contents/blogs/**/*.md` and `contents/docs/**/*.md` are parsed into the content database.
  - `contents/media/` holds referenced media.
- The content pipeline is the center of the app:
  - `libs/content/src/bin/content-build.rs` builds `target/content-build`.
  - `libs/content::database` parses markdown front matter + body, computes handles/dates, builds the docs sidebar and blog timeline, writes `db.json`, copies/bundles media, renders Mermaid diagrams, and generates `/public/...` asset URLs.
  - Markdown is stored as a JSON AST (`MarkdownContent` / `MarkdownNode`), not as raw markdown strings.
- `apps/frontend/src/main.rs` is the SSR composition point:
  - loads `home.yaml`, `db.json`, and `rss.xml` from `CONTENT_DB_PATH` (default `target/content-build`);
  - serves Leptos routes, `/api/v1`, `/rss.xml`, `/sitemap.xml`, `/api/docs`, `/assets`, `/pkg`, and `/media`;
  - injects `HomeConfig` and `ContentDatabase` into Leptos SSR context.
- `apps/frontend/src/lib.rs` defines the browser routes. Static reference pages (`/blog`, `/docs`, `/projects`) fetch lightweight API payloads, while content pages under `/blogs/*any` and `/docs/*any` render the JSON AST through `apps/frontend/src/components/markdown/*`.
- UI chrome translations live in `apps/frontend/src/i18n/`. Content translations live with the content itself (`presentationEn`, `titleEn`, `descriptionEn`, etc.), not in the translation table.

## Key conventions

- Do not hand-edit generated output under `target/content-build`; update `contents/` or the content builder instead.
- Content kind is inferred from path unless front matter `spec` overrides it:
  - `blogs/...` => blog
  - `docs/...` => doc
  - paths containing `/project/` => project
- Handles are derived from relative markdown paths and slugified to lowercase path segments. `index.md` collapses to the parent handle by default.
- A directory-level `meta.json` changes doc navigation behavior:
  - `order` controls sidebar ordering.
  - `index_is_main_item: false` keeps `index.md` as `.../index` instead of collapsing it into the folder handle.
- Visibility is controlled by front matter, not just file presence:
  - `draft: true` hides content from API/nav output.
  - `release: false` produces an empty `released_at`, which also hides content from page/nav endpoints.
  - If `date` is missing, the content builder falls back to git history for created/updated dates.
- In markdown and YAML content, handle-like prefixes are meaningful:
  - `media#...` becomes a public media path.
  - `blog#...`, `doc#...`, and `project#...` become site routes.
  - Home/useful-link/media assets can also be rewritten into bundled `/public/media/...` files during content generation.
- When adding UI strings, extend `Translations` and fill both `FR` and `EN`. When adding bilingual content fields, follow the existing `*En` naming used by `HomeConfig` and history entries.
