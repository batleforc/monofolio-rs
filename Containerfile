# =============================================================================
# Stage 1 - node-builder: bundle Shiki (syntax highlighting)
# =============================================================================
FROM docker.io/node:lts-bookworm-slim AS node-builder
WORKDIR /build

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN corepack enable && corepack prepare pnpm@latest --activate \
    && pnpm install --frozen-lockfile

COPY apps/frontend/shiki.entry.js apps/frontend/shiki.entry.js
COPY scripts/ scripts/

RUN pnpm run build:shiki

# =============================================================================
# Stage 2 - rust-builder: content-build + cargo-leptos release build
# Node.js is required by cargo-leptos to run the Tailwind CSS compiler.
# =============================================================================
FROM docker.io/rust:1-bookworm AS rust-builder
WORKDIR /app

# ── Install Node.js (LTS) by copying from the official node image ────────────
COPY --from=docker.io/node:lts-bookworm-slim /usr/local/bin/node /usr/local/bin/node
COPY --from=docker.io/node:lts-bookworm-slim /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -sf /usr/local/lib/node_modules/npm/bin/npm-cli.js  /usr/local/bin/npm \
    && ln -sf /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx \
    && npm install -g pnpm \
    && node --version && pnpm --version

# ── Install Rust toolchain extras ────────────────────────────────────────────
RUN rustup target add wasm32-unknown-unknown

# cargo-leptos is the build orchestrator for Leptos SSR projects.
# --locked ensures the version pinned in its Cargo.lock is used.
RUN cargo install cargo-leptos --locked

# ── Node dependencies (Tailwind CSS + dev tools) ─────────────────────────────
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile

# ── Copy workspace source ─────────────────────────────────────────────────────
COPY Cargo.toml Leptos.toml ./
COPY apps/ apps/
COPY libs/ libs/
COPY contents/ contents/

# Inject the pre-built Shiki bundle (built in stage 1)
COPY --from=node-builder /build/apps/frontend/public/js/shiki.bundle.js \
    apps/frontend/public/js/shiki.bundle.js

# Pre-generate the stylesheet expected by cargo-leptos.
# This avoids a build failure when cargo-leptos cannot find target/tmp/tailwind.css.
RUN mkdir -p target/tmp \
    && pnpm exec tailwindcss \
    -i apps/frontend/style/main.css \
    -o target/tmp/tailwind.css \
    --minify

# ── Generate content database (db.json, home.yaml, rss.xml, public/ media) ───
RUN cargo run --release --package content --bin content-build

# ── Build Leptos app: SSR binary + WASM/CSS assets ───────────────────────────
# Outputs:
#   target/release/frontend   - SSR server binary
#   target/site/              - static assets, WASM bundle, compiled CSS
RUN cargo leptos build --release --project frontend

# =============================================================================
# Stage 3 - runtime: minimal image with only what the server needs at runtime
# =============================================================================
FROM docker.io/debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# SSR server binary
COPY --from=rust-builder /app/target/release/frontend ./frontend

# Leptos site assets: WASM bundle, compiled CSS, and static files from
# apps/frontend/public/ (favicons, manifest, shiki.bundle.js, …)
COPY --from=rust-builder /app/target/site ./target/site

# Content database: db.json, home.yaml, rss.xml, config.yaml, public/ (media)
COPY --from=rust-builder /app/target/content-build ./target/content-build

# Raw media files served at /media (MEDIA_PATH default: contents/media)
COPY --from=rust-builder /app/contents/media ./contents/media

# ── Runtime configuration ─────────────────────────────────────────────────────
# LEPTOS_SITE_ROOT must match the path where target/site/ is mounted.
ENV LEPTOS_SITE_ROOT="target/site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
# Override at runtime if you mount content from a volume:
# ENV CONTENT_DB_PATH="target/content-build"
# ENV MEDIA_PATH="contents/media"

EXPOSE 3000

CMD ["./frontend"]
