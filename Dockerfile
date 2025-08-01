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

FROM clux/muslrust:stable AS chef

ARG MOLD_VERSION

# Install dependencies needed for mold installation
RUN apt-get update && apt-get install -y \
    wget tar && \
    rm -rf /var/lib/apt/lists/*

# Install mold
COPY .devcontainer/scripts/install-mold.sh /tmp/install-mold.sh
RUN chmod +x /tmp/install-mold.sh && \
    /tmp/install-mold.sh ${MOLD_VERSION} && \
    rm /tmp/install-mold.sh

RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --bin db --bin pashe-backend --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /volume/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin db --bin pashe-backend

FROM gcr.io/distroless/static:nonroot AS runtime

LABEL  \
    org.opencontainers.image.authors='azaiscorentin@gmail.com' \
    org.opencontainers.image.source='https://github.com/azais-corentin/pashe' \
    org.opencontainers.image.vendor='Corentin AZAIS'

COPY --from=builder --chown=nonroot:nonroot /volume/target/x86_64-unknown-linux-musl/release/pashe-backend /app/pashe-backend
COPY --from=builder --chown=nonroot:nonroot /volume/target/x86_64-unknown-linux-musl/release/db /app/db
ENTRYPOINT ["/app/pashe-backend"]