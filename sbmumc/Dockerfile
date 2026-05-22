# ============================================================
# SBMUMC - Universal Meta-Compiler Docker Image
# Supports: AMD64, ARM64, RISC-V, and all major cloud platforms
# ============================================================

# -----------------------------
# Stage 1: Base Build
# -----------------------------
FROM --platform=$BUILDPLATFORM rust:1.75-bookworm AS base

# Install build dependencies for multiple platforms
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    llvm \
    libclang-dev \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    protobuf-c-compiler \
    libprotobuf-dev \
    libz-dev \
    libbz2-dev \
    liblzma-dev \
    git \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Configure Rust for cross-compilation
RUN rustup target add x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu \
    armv7-unknown-linux-gnueabihf \
    riscv64gc-unknown-linux-gnu \
    wasm32-wasi \
    wasm32-unknown-emscripten \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-musl

# -----------------------------
# Stage 2: Dependency Builder
# -----------------------------
FROM base AS deps-builder

WORKDIR /build

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy source to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "[package]\nname=\"sbmumc\"\nversion=\"0.1.0\"\nedition=\"2021\"" > Cargo.toml && \
    cargo build --release --locked 2>/dev/null || true

# Remove dummy source
RUN rm -rf src Cargo.toml

# -----------------------------
# Stage 3: Builder
# -----------------------------
FROM base AS builder

WORKDIR /build

# Copy dependency cache from deps-builder stage
COPY --from=deps-builder /build/target /build/target

# Copy actual source code
COPY src/ ./src/
COPY Cargo.toml Cargo.lock* ./

# Build for native target (x86_64)
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu \
    --features full,safe,optimized \
    --bin sbmumc

# -----------------------------
# Stage 4: Minimal Runtime (Alpine)
# -----------------------------
FROM alpine:3.19 AS runtime-minimal

# Install minimal runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libssl3 \
    zlib \
    musl'stdlib' \
    && update-ca-certificates 2>/dev/null || true

# Create non-root user for security
RUN addgroup -g 1000 -S sbmumc && \
    adduser -u 1000 -G sbmumc -S sbmumc

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/sbmumc /app/sbmumc

# Set ownership
RUN chown -R sbmumc:sbmumc /app

# Switch to non-root user
USER sbmumc

# Environment variables
ENV SBMUMC_HOME=/app \
    SBMUMC_DATA=/app/data \
    RUST_LOG=info \
    RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose default ports
EXPOSE 8080 8081 8082 8083

# Default entrypoint
ENTRYPOINT ["/app/sbmumc"]

# -----------------------------
# Stage 5: Standard Runtime (Debian)
# -----------------------------
FROM debian:bookworm-slim AS runtime-standard

# Install standard runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    zlib1g \
    libbz2-1.0 \
    liblzma5 \
    libzstd1 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 -s /bin/bash sbmumc

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/sbmumc /app/sbmumc

# Copy example configuration
COPY --from=builder /build/src/../configs/docker/ /app/config/ 2>/dev/null || true

# Set ownership
RUN chown -R sbmumc:sbmumc /app

# Switch to non-root user
USER sbmumc

# Environment variables
ENV SBMUMC_HOME=/app \
    SBMUMC_DATA=/app/data \
    RUST_LOG=info \
    RUST_BACKTRACE=1 \
    SERVER_PORT=8080 \
    SERVER_HOST=0.0.0.0

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose ports
EXPOSE 8080 8081 8082 8083

# Entrypoint with default args
ENTRYPOINT ["/app/sbmumc"]

# -----------------------------
# Stage 6: Full Runtime (Ubuntu)
# -----------------------------
FROM ubuntu:24.04 AS runtime-full

# Install full runtime dependencies with development tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    build-essential \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 -s /bin/bash sbmumc

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/sbmumc /app/sbmumc

# Create directories
RUN mkdir -p /app/{config,data,logs,plugins,cache} && \
    chown -R sbmumc:sbmumc /app

# Copy example files
COPY --from=builder /build/src/../configs/docker/ /app/config/ 2>/dev/null || true

# Switch to non-root user
USER sbmumc

# Environment variables
ENV SBMUMC_HOME=/app \
    SBMUMC_DATA=/app/data \
    SBMUMC_LOGS=/app/logs \
    SBMUMC_PLUGINS=/app/plugins \
    RUST_LOG=info \
    RUST_BACKTRACE=1 \
    SERVER_PORT=8080 \
    SERVER_HOST=0.0.0.0 \
    API_PORT=8081 \
    METRICS_PORT=8082 \
    GRPC_PORT=8083

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose all ports
EXPOSE 8080 8081 8082 8083

# Volume mounts for persistence
VOLUME ["/app/data", "/app/logs", "/app/plugins", "/app/cache"]

ENTRYPOINT ["/app/sbmumc"]

# -----------------------------
# Stage 7: Cloud Native (Optimized for AWS/GCP/Azure)
# -----------------------------
FROM gcr.io/distroless/cc-debian12 AS runtime-cloud

WORKDIR /app

# Copy binary (static, from musl build)
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/sbmumc /app/sbmumc

# Set as non-root
USER 1000:1000

# Minimal environment for cloud
ENV SBMUMC_HOME=/app \
    SBMUMC_DATA=/app/data \
    RUST_LOG=info

# Expose cloud-optimized ports
EXPOSE 8080 8081 8082 8083

# No shell access, minimal attack surface
ENTRYPOINT ["/app/sbmumc"]

# -----------------------------
# Stage 8: Development Image
# -----------------------------
FROM base AS runtime-dev

# Install development tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    vim \
    less \
    jq \
    curl \
    tree \
    htop \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy debug build
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/debug/sbmumc /app/sbmumc

# Create development directories
RUN mkdir -p /app/{src,config,data,logs,plugins,cache} && \
    chown -R $(id -u):$(id -g) /app

# Environment for development
ENV RUST_LOG=debug \
    RUST_BACKTRACE=full \
    SBMUMC_DEV=1

# Volume for source code mounting
VOLUME ["/app/src", "/app/config"]

EXPOSE 8080 8081 8082 8083 9090

ENTRYPOINT ["/app/sbmumc"]

# -----------------------------
# Default target: Standard runtime
# -----------------------------
FROM runtime-standard AS default

# Labels
LABEL maintainer="Samuel Benwellonedge Mukandara <sbmumc@example.com>" \
      version="0.1.0" \
      description="SBMUMC - Universal Meta-Compiler for sovereign AGI" \
      org.opencontainers.image.title="SBMUMC Universal Meta-Compiler" \
      org.opencontainers.image.description="A sovereign AGI system for universal code transformation and generation" \
      org.opencontainers.image.source="https://github.com/Benwellonedge28/sbmumc" \
      org.opencontainers.image.documentation="https://sbmumc.com/docs" \
      org.opencontainers.image.vendor="SBMUMC" \
      org.opencontainers.artifact.created="$SOURCE_DATE" \
      org.opencontainers.artifact.version="$VERSION"

# Build arguments
ARG BUILD_DATE
ARG VERSION=0.1.0
ARG VCS_REF

LABEL org.opencontainers.image.created=$BUILD_DATE \
      org.opencontainers.artifact.version=$VERSION \
      org.opencontainers.artifact.revision=$VCS_REF