# Multi-stage Dockerfile for CURSED Compiler
# Production-ready container with minimal attack surface

# ============================================================================
# BUILD STAGE
# ============================================================================
FROM alpine:3.19 AS builder

# Build arguments
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG VERSION=1.0.0

# Install build dependencies
RUN apk add --no-cache \
    build-base \
    cmake \
    curl \
    git \
    llvm18-dev \
    llvm18-static \
    musl-dev \
    pkgconfig \
    python3 \
    zig \
    && rm -rf /var/cache/apk/*

# Create build user for security
RUN adduser -D -s /bin/sh builder

# Set working directory
WORKDIR /src

# Copy source code
COPY --chown=builder:builder . .

# Switch to build user
USER builder

# Configure build environment
ENV ZIG_CACHE_DIR=/tmp/zig-cache
ENV ZIG_GLOBAL_CACHE_DIR=/tmp/zig-global-cache

# Build CURSED compiler for the target platform
RUN case "${TARGETPLATFORM}" in \
    "linux/amd64") TARGET="x86_64-unknown-linux-gnu" ;; \
    "linux/arm64") TARGET="aarch64-unknown-linux-gnu" ;; \
    *) TARGET="x86_64-unknown-linux-gnu" ;; \
    esac && \
    echo "Building for target: $TARGET" && \
    zig build -Dtarget=$TARGET -Doptimize=ReleaseFast --prefix /tmp/dist

# Verify build artifacts
RUN ls -la /tmp/dist/bin/ && \
    /tmp/dist/bin/cursed-zig --version

# ============================================================================
# RUNTIME STAGE - Alpine Linux
# ============================================================================
FROM alpine:3.19 AS alpine-runtime

# Runtime dependencies for Alpine
RUN apk add --no-cache \
    libc6-compat \
    libgcc \
    libstdc++ \
    llvm18-libs \
    && rm -rf /var/cache/apk/*

# Create cursed user
RUN adduser -D -s /bin/sh cursed

# Copy built artifacts from builder
COPY --from=builder /tmp/dist/bin/* /usr/local/bin/
COPY --from=builder /src/stdlib /usr/local/share/cursed/stdlib

# Set permissions
RUN chmod +x /usr/local/bin/cursed-* && \
    chown -R cursed:cursed /usr/local/share/cursed

# Create cache and work directories
RUN mkdir -p /var/cache/cursed /home/cursed/work && \
    chown cursed:cursed /var/cache/cursed /home/cursed/work

# Switch to cursed user
USER cursed
WORKDIR /home/cursed/work

# Environment configuration
ENV PATH="/usr/local/bin:$PATH"
ENV CURSED_CACHE_DIR="/var/cache/cursed"
ENV CURSED_STDLIB_PATH="/usr/local/share/cursed/stdlib"

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD cursed-zig --version || exit 1

# Default command
ENTRYPOINT ["cursed-zig"]
CMD ["--help"]

# ============================================================================
# RUNTIME STAGE - Ubuntu (for compatibility)
# ============================================================================
FROM ubuntu:22.04 AS ubuntu-runtime

# Avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Runtime dependencies for Ubuntu
RUN apt-get update && apt-get install -y \
    libc6 \
    libgcc-s1 \
    libstdc++6 \
    llvm-18 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create cursed user
RUN useradd -m -s /bin/bash cursed

# Copy built artifacts from builder
COPY --from=builder /tmp/dist/bin/* /usr/local/bin/
COPY --from=builder /src/stdlib /usr/local/share/cursed/stdlib

# Set permissions
RUN chmod +x /usr/local/bin/cursed-* && \
    chown -R cursed:cursed /usr/local/share/cursed

# Create cache and work directories
RUN mkdir -p /var/cache/cursed /home/cursed/work && \
    chown cursed:cursed /var/cache/cursed /home/cursed/work

# Switch to cursed user
USER cursed
WORKDIR /home/cursed/work

# Environment configuration
ENV PATH="/usr/local/bin:$PATH"
ENV CURSED_CACHE_DIR="/var/cache/cursed"
ENV CURSED_STDLIB_PATH="/usr/local/share/cursed/stdlib"

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD cursed-zig --version || exit 1

# Default command
ENTRYPOINT ["cursed-zig"]
CMD ["--help"]

# ============================================================================
# DEVELOPMENT STAGE
# ============================================================================
FROM alpine-runtime AS development

# Switch back to root for development tools
USER root

# Install development dependencies
RUN apk add --no-cache \
    bash \
    curl \
    git \
    llvm18-dev \
    make \
    python3 \
    vim \
    zig \
    && rm -rf /var/cache/apk/*

# Install additional development tools
RUN apk add --no-cache \
    gdb \
    strace \
    valgrind \
    && rm -rf /var/cache/apk/*

# Create development workspace
RUN mkdir -p /workspace && chown cursed:cursed /workspace

# Switch back to cursed user
USER cursed
WORKDIR /workspace

# Development environment
ENV CURSED_DEV_MODE=1
ENV CURSED_DEBUG=1

# Override entrypoint for development
ENTRYPOINT ["/bin/bash"]

# ============================================================================
# LABELS AND METADATA
# ============================================================================

# Add metadata labels
LABEL maintainer="CURSED Development Team <dev@cursed.dev>"
LABEL description="CURSED Programming Language Compiler"
LABEL version="1.0.0"
LABEL org.opencontainers.image.title="CURSED Compiler"
LABEL org.opencontainers.image.description="A modern, safe, and performant programming language compiler"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.vendor="CURSED Development Team"
LABEL org.opencontainers.image.url="https://cursed.dev"
LABEL org.opencontainers.image.source="https://github.com/ghuntley/cursed"
LABEL org.opencontainers.image.documentation="https://docs.cursed.dev"
LABEL org.opencontainers.image.licenses="MIT"

# Default to Alpine runtime
FROM alpine-runtime
