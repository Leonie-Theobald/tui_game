FROM mcr.microsoft.com/devcontainers/base:ubuntu

# System deps
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Switch to vscode user FIRST
USER vscode

# Install Rust for vscode user
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add cargo to PATH
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Set default toolchain
RUN rustup default stable