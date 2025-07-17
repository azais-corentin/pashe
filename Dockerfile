# There are 2 stages in this image:
# Stage 1: development      – Base development container with essential tools for development.
# Stage 2: production       – Final container that includes the built application.

ARG DEBIAN_FRONTEND noninteractive

FROM mcr.microsoft.com/devcontainers/base:debian AS development

ARG DEBIAN_FRONTEND

USER vscode

# Install mise
RUN curl https://mise.run | sh && \
    echo 'eval "$(~/.local/bin/mise activate bash)"' >> ~/.bashrc
# Install bun
RUN ~/.local/bin/mise use -g bun node
# Install gemini-cli
RUN ~/.local/bin/mise exec bun -- bun install -g @google/gemini-cli && \
    echo 'PATH=$PATH:~/.bun/bin/' >> ~/.bashrc

# Start from a clean, slim container
FROM oven/bun:1 AS production

LABEL  \
    org.opencontainers.image.authors='haellsigh@gmail.com' \
    org.opencontainers.image.source='https://github.com/azais-corentin/PoeIndexer' \
    org.opencontainers.image.vendor='Azais Corentin'

EXPOSE 3000
CMD ["bun", "run", "index..ts"]