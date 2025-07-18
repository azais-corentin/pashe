# There are 5 stages in this image:
# Stage 1: development      – Base development container with essential tools for development.
# Stage 2: chef             – Base chef container for dependency caching.
# Stage 3: planner          – Prepares dependency recipe for caching.
# Stage 4: builder          – Builds the Rust application.
# Stage 5: runtime          – Final container that includes the built application.

ARG DEBIAN_FRONTEND noninteractive

FROM mcr.microsoft.com/devcontainers/base:debian AS development

ARG DEBIAN_FRONTEND

USER vscode

# Install mise
RUN curl https://mise.run | sh && \
    echo 'eval "$(~/.local/bin/mise activate bash)"' >> ~/.bashrc
# Install bun
RUN ~/.local/bin/mise use -g bun node rust
# Install gemini-cli
RUN ~/.local/bin/mise exec bun -- bun install -g @google/gemini-cli && \
    echo 'PATH=$PATH:~/.bun/bin/' >> ~/.bashrc

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin pashe-backend

FROM debian:bookworm-slim AS runtime

LABEL  \
    org.opencontainers.image.authors='haellsigh@gmail.com' \
    org.opencontainers.image.source='https://github.com/azais-corentin/PoeIndexer' \
    org.opencontainers.image.vendor='Azais Corentin'

WORKDIR /app
COPY --from=builder /app/target/release/pashe-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pashe-backend"]