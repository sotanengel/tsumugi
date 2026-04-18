FROM node:22-bookworm-slim

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

# System dependencies for Tauri development
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    pkg-config \
    wget \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}

USER ${USERNAME}
WORKDIR /workspace

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    && rustup component add clippy rustfmt

# Install pnpm
RUN corepack enable && corepack prepare pnpm@10 --activate

ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"
