# syntax=docker/dockerfile:1.7
# =============================================================================
# Stage 1 - node-builder: bundle Shiki (syntax highlighting)
# =============================================================================
FROM docker.io/node:lts-bookworm-slim AS node-builder
WORKDIR /build

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN --mount=type=cache,id=pnpm-store,target=/pnpm/store \
    corepack enable && corepack prepare pnpm@latest --activate \
    && pnpm config set store-dir /pnpm/store \
    && pnpm install --frozen-lockfile

COPY apps/frontend/shiki.entry.js apps/frontend/shiki.entry.js
COPY scripts/ scripts/

RUN pnpm run build:shiki

# =============================================================================
# Stage 2 - rust-base: shared Rust + Node.js toolchain for build stages
# Node.js is required by cargo-leptos to run the Tailwind CSS compiler.
# =============================================================================
FROM docker.io/rust:1-bookworm AS rust-base
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
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo install cargo-leptos --locked

# Reuse the node_modules tree already resolved in the Node stage so the Rust
# stages do not spend time reinstalling the same JS toolchain.
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY --from=node-builder /build/node_modules ./node_modules

# =============================================================================
# Stage 3 - rust-deps: precompile dependency-heavy Rust targets from manifests
# =============================================================================
FROM rust-base AS rust-deps

COPY Cargo.toml Cargo.lock Leptos.toml ./
COPY .cargo .cargo
COPY apps/frontend/Cargo.toml apps/frontend/Cargo.toml
COPY libs/api/Cargo.toml libs/api/Cargo.toml
COPY libs/content/Cargo.toml libs/content/Cargo.toml
COPY libs/trace/Cargo.toml libs/trace/Cargo.toml

# Build placeholder crates so Docker can cache compiled dependencies separately
# from the real workspace sources. When code changes but manifests do not, the
# final build can reuse most of this stage.
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    mkdir -p apps/frontend/src libs/api/src libs/content/src/bin libs/trace/src \
    && printf 'pub fn __placeholder() {}\n' > apps/frontend/src/lib.rs \
    && printf 'fn main() {}\n' > apps/frontend/src/main.rs \
    && printf 'pub fn __placeholder() {}\n' > libs/api/src/lib.rs \
    && printf 'pub fn __placeholder() {}\n' > libs/content/src/lib.rs \
    && printf 'fn main() {}\n' > libs/content/src/bin/content-build.rs \
    && printf 'fn main() {}\n' > libs/content/src/bin/generate-signature.rs \
    && printf 'pub fn __placeholder() {}\n' > libs/trace/src/lib.rs \
    && cargo build --release --locked --package content --bins \
    && cargo build --release --locked --package frontend --bin frontend --features ssr \
    && cargo build --release --locked --package frontend --lib --no-default-features --features hydrate --target wasm32-unknown-unknown

# =============================================================================
# Stage 4 - rust-builder: final content-build + cargo-leptos release build
# =============================================================================
FROM rust-base AS rust-builder

COPY Cargo.toml Cargo.lock Leptos.toml ./
COPY .cargo .cargo
COPY apps/frontend/Cargo.toml apps/frontend/Cargo.toml
COPY libs/api/Cargo.toml libs/api/Cargo.toml
COPY libs/content/Cargo.toml libs/content/Cargo.toml
COPY libs/trace/Cargo.toml libs/trace/Cargo.toml
COPY --from=rust-deps /app/target ./target

# ── Copy workspace source ─────────────────────────────────────────────────────
COPY apps/ apps/
COPY libs/ libs/

# Inject the pre-built Shiki bundle (built in stage 1)
COPY --from=node-builder /build/apps/frontend/public/js/shiki.bundle.js \
    apps/frontend/public/js/shiki.bundle.js

# Keep third-party dependencies from rust-deps, but force workspace crates to be
# rebuilt from the real sources instead of reusing placeholder artefacts.
RUN cargo clean --release --package frontend --package content --package api --package trace \
    && cargo clean --release --target wasm32-unknown-unknown --package frontend

# Pre-generate the stylesheet expected by cargo-leptos.
# This avoids a build failure when cargo-leptos cannot find target/tmp/tailwind.css.
RUN mkdir -p target/tmp \
    && CI=true pnpm exec tailwindcss \
    -i apps/frontend/style/main.css \
    -o target/tmp/tailwind.css \
    --minify

# ── Build Leptos app: SSR binary + WASM/CSS assets ───────────────────────────
# Outputs:
#   target/release/frontend   - SSR server binary
#   target/site/              - static assets, WASM bundle, compiled CSS
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo leptos build --release --project frontend

# Copy content last so content-only updates do not invalidate the expensive
# Leptos SSR/WASM compilation step above.
COPY contents/ contents/

# ── Generate content database (db.json, home.yaml, rss.xml, public/ media) ───
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo run --release --locked --package content --bin content-build && \
    cargo run --release --locked --package content --bin generate-signature -- --output \
    ./target/content-build/public/minia/signature.png

# =============================================================================
# Stage 5 - runtime: minimal image with only what the server needs at runtime
# =============================================================================
FROM gcr.io/distroless/cc-debian12:nonroot@sha256:e2d29aec8061843706b7e484c444f78fafb05bfe47745505252b1769a05d14f1
WORKDIR /app

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
