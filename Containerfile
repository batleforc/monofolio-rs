# syntax=docker/dockerfile:1.7
# Shape from weebo-containerfile: pinned bases and tools, cargo-chef dependency
# layer, distroless runtime. Bases are pinned by digest; updatecli moves them.

# pnpm, exact: lockfiles are law in the image. The lockfile is checked with
# this version (pnpm install --frozen-lockfile passes with it).
ARG PNPM_VERSION=12.23.0

# =============================================================================
# Stage 1 - node-builder: JS toolchain + Shiki bundle
# =============================================================================
FROM docker.io/node:lts-bookworm-slim@sha256:242549cd46785b480c832479a730f4f2a20865d61ea2e404fdb2a5c3d3b73ecf AS node-builder
ARG PNPM_VERSION
WORKDIR /build

# --ignore-scripts: a future pnpm release cannot run an install-time script here.
RUN npm install -g --ignore-scripts pnpm@${PNPM_VERSION}
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN --mount=type=cache,id=pnpm-store,target=/pnpm/store \
    pnpm config set store-dir /pnpm/store \
    && pnpm install --frozen-lockfile

COPY apps/frontend/shiki.entry.js apps/frontend/shiki.entry.js
COPY scripts/ scripts/
RUN pnpm run build:shiki

# =============================================================================
# Stage 2 - chef: Rust toolchain + build tools, shared by planner and builder
# Node.js is required by cargo-leptos to run the Tailwind CSS compiler.
# =============================================================================
FROM docker.io/rust:1.100-bookworm@sha256:75d26c23c24cc6ea1cdabf0714a6bff8fd7bf3248716043fd7ae86dac7f41b94 AS chef
WORKDIR /app

# Node from the pinned node-builder stage (not an unpinned image reference).
COPY --from=node-builder /usr/local/bin/node /usr/local/bin/node
COPY --from=node-builder /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -sf /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
    && ln -sf /usr/local/lib/node_modules/pnpm/pnpm /usr/local/bin/pnpm \
    && node --version && pnpm --version

RUN rustup target add wasm32-unknown-unknown

# Prebuilt, checksum-pinned tools instead of `cargo install`, which compiled
# them on every base image change. cargo-leptos 0.3.11 is the latest release
# with binaries (glibc >= 2.35); an unpinned install broke when 0.3.12 needed a
# newer rustc. Bump both with the Rust image.
ARG CARGO_LEPTOS_VERSION=0.3.11
ARG CARGO_LEPTOS_SHA256=1c9cf0e1e3ffae6c8b58d29281d949dab560cea613b80d361f323f93071df7c8
ARG CARGO_CHEF_VERSION=0.1.78
ARG CARGO_CHEF_SHA256=70ef940ef90d04d122f0176fdb8d6c39069191b484a1eaa29b327370c2e1c3c0
RUN curl -fsSLo /tmp/cargo-leptos.tar.gz \
      "https://github.com/leptos-rs/cargo-leptos/releases/download/v${CARGO_LEPTOS_VERSION}/cargo-leptos-x86_64-unknown-linux-gnu.tar.gz" \
    && echo "${CARGO_LEPTOS_SHA256}  /tmp/cargo-leptos.tar.gz" | sha256sum -c - \
    && tar -xzf /tmp/cargo-leptos.tar.gz -C /tmp \
    && install -m 0755 /tmp/cargo-leptos-x86_64-unknown-linux-gnu/cargo-leptos /usr/local/cargo/bin/ \
    && curl -fsSLo /tmp/cargo-chef.tar.xz \
      "https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CARGO_CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-gnu.tar.xz" \
    && echo "${CARGO_CHEF_SHA256}  /tmp/cargo-chef.tar.xz" | sha256sum -c - \
    && tar -xJf /tmp/cargo-chef.tar.xz -C /tmp \
    && install -m 0755 /tmp/cargo-chef-x86_64-unknown-linux-gnu/cargo-chef /usr/local/cargo/bin/ \
    && rm -rf /tmp/cargo-leptos* /tmp/cargo-chef* \
    && cargo leptos --version && cargo chef --version

# =============================================================================
# Stage 3 - planner: workspace manifests -> recipe.json
# The recipe only changes when dependencies or members change, so a source
# edit keeps the dependency layer below cached.
# =============================================================================
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY apps/ apps/
COPY libs/ libs/
RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Stage 4 - builder: dependencies (cached while recipe.json is unchanged),
# then tests, the Leptos release build and the content build
# =============================================================================
FROM chef AS builder

# .cargo/config.toml changes rustflags (linker), so it must be in place before
# cooking or the final build would not reuse the cooked artefacts.
COPY .cargo .cargo
COPY --from=planner /app/recipe.json recipe.json
# Same profile, targets and features as the builds below:
#   1. workspace, default features: content bins, api, frontend SSR
#      (frontend's default feature is ssr, which is what cargo-leptos builds)
#   2. --tests: dev-dependencies for `cargo test --release --workspace`
#   3. the content bins alone: `cargo run -p content` resolves content's own
#      feature set, different from the workspace-wide one
#   4. the WASM front, in target/front where cargo-leptos builds it
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo chef cook --release --locked --recipe-path recipe.json \
    && cargo chef cook --release --locked --tests --recipe-path recipe.json \
    && cargo chef cook --release --locked --recipe-path recipe.json \
       --manifest-path libs/content/Cargo.toml --bins \
    && cargo chef cook --release --locked --recipe-path recipe.json \
       --manifest-path apps/frontend/Cargo.toml --no-default-features --features hydrate \
       --target wasm32-unknown-unknown --target-dir target/front

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY --from=node-builder /build/node_modules ./node_modules

COPY Cargo.toml Cargo.lock Leptos.toml ./
COPY apps/ apps/
COPY libs/ libs/
# The Shiki bundle built in stage 1.
COPY --from=node-builder /build/apps/frontend/public/js/shiki.bundle.js \
    apps/frontend/public/js/shiki.bundle.js

# A failing test stops the image build (and the deploy).
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo test --release --locked --workspace

# The stylesheet cargo-leptos expects at target/tmp/tailwind.css.
RUN mkdir -p target/tmp \
    && CI=true pnpm exec tailwindcss \
    -i apps/frontend/style/main.css \
    -o target/tmp/tailwind.css \
    --minify

# SSR binary (target/release/frontend) + WASM/CSS assets (target/site/).
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo leptos build --release --project frontend

# The content tools are compiled before the content is copied, so a
# content-only change (a new post) just runs them.
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    cargo build --release --locked --package content --bin content-build --bin generate-signature

# Content last, so content-only updates do not invalidate the builds above.
COPY contents/ contents/
# Git history gives each page its created/updated dates (content-build runs
# `git log`). Copied last so a new commit only invalidates the content step.
COPY .git .git

# Content database (db.json, home.yaml, rss.xml, public/ media) + signature.
RUN ./target/release/content-build \
    && ./target/release/generate-signature --output ./target/content-build/public/minia/signature.png

# =============================================================================
# Stage 5 - runtime: minimal image with only what the server needs at runtime
# =============================================================================
FROM gcr.io/distroless/cc-debian12:nonroot@sha256:bd2899c12b335c827750ccf2359879eab09c09b206023dcebea408947d54127c
WORKDIR /app

# SSR server binary
COPY --from=builder /app/target/release/frontend ./frontend

# Leptos site assets: WASM bundle, compiled CSS, and static files from
# apps/frontend/public/ (favicons, manifest, shiki.bundle.js, …)
COPY --from=builder /app/target/site ./target/site

# Content database: db.json, home.yaml, rss.xml, config.yaml, public/ (media)
COPY --from=builder /app/target/content-build ./target/content-build

# Raw media files served at /media (MEDIA_PATH default: contents/media)
COPY --from=builder /app/contents/media ./contents/media

# ── Runtime configuration ─────────────────────────────────────────────────────
# LEPTOS_SITE_ROOT must match the path where target/site/ is mounted.
ENV LEPTOS_SITE_ROOT="target/site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
# Override at runtime if you mount content from a volume:
# ENV CONTENT_DB_PATH="target/content-build"
# ENV MEDIA_PATH="contents/media"

EXPOSE 3000

# 65532 is distroless's nonroot user, declared numerically: no /etc/passwd
# lookup at runtime, and Kubernetes needs a numeric UID for runAsNonRoot.
USER 65532:65532

CMD ["./frontend"]
