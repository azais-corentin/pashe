# There are 6 stages in this image:
# Stage 1: development      – Base development container with essential tools for development.
# Stage 2: chef             – Base chef container for dependency caching.
# Stage 3: planner          – Prepares dependency recipe for caching.
# Stage 4: cacher           – Caches dependencies for faster builds.
# Stage 5: builder          – Builds the Rust application.
# Stage 6: runtime          – Final container that includes the built application.

ARG DEBIAN_FRONTEND=noninteractive
ARG MOLD_VERSION=2.40.3

FROM mcr.microsoft.com/devcontainers/base:debian AS development

ARG DEBIAN_FRONTEND
ARG MOLD_VERSION

# Install clickhouse repository
RUN curl -fsSL 'https://packages.clickhouse.com/rpm/lts/repodata/repomd.xml.key' | sudo gpg --dearmor -o /usr/share/keyrings/clickhouse-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/clickhouse-keyring.gpg arch=$(dpkg --print-architecture)] https://packages.clickhouse.com/deb stable main" | sudo tee /etc/apt/sources.list.d/clickhouse.list

# Install dependencies
RUN apt-get update && apt-get install -y \ 
    apt-transport-https build-essential ca-certificates clickhouse-client \
    curl file gnupg libayatana-appindicator3-dev librsvg2-dev libssl-dev \
    libwebkit2gtk-4.1-dev libxdo-dev pkg-config wget && \
    rm -rf /var/lib/apt/lists/*

# Install mold
RUN --mount=type=bind,source=.devcontainer/scripts,target=/tmp/scripts \
    /tmp/scripts/install-mold.sh ${MOLD_VERSION}

USER vscode

# Install mise
RUN curl https://mise.run | sh && \
    echo 'eval "$(~/.local/bin/mise activate bash)"' >> ~/.bashrc
# Install bun
RUN ~/.local/bin/mise use -g bun node rust
# Install gemini-cli
RUN ~/.local/bin/mise exec bun -- bun install -g @google/gemini-cli && \
    echo 'PATH=$PATH:~/.bun/bin/' >> ~/.bashrc

# Caching inspired by https://depot.dev/docs/container-builds/how-to-guides/optimal-dockerfiles/rust-dockerfile
FROM clux/muslrust:stable AS base

ARG MOLD_VERSION

# Install dependencies needed for mold installation
RUN apt-get update && apt-get install -y \
    wget tar && \
    rm -rf /var/lib/apt/lists/*

COPY .devcontainer/scripts/install-mold.sh /tmp/install-mold.sh
RUN chmod +x /tmp/install-mold.sh && \
    /tmp/install-mold.sh ${MOLD_VERSION} && \
    rm /tmp/install-mold.sh

    
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef sccache
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare

FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --target x86_64-unknown-linux-musl
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static:nonroot AS runtime

LABEL  \
    org.opencontainers.image.authors='azaiscorentin@gmail.com' \
    org.opencontainers.image.source='https://github.com/azais-corentin/pashe' \
    org.opencontainers.image.vendor='Corentin AZAIS'

COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/pashe-backend /app/pashe-backend
COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/db /app/db
COPY migrations /app/migrations

CMD ["/app/pashe-backend"]