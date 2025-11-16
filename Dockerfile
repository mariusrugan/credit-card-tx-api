#
# docker buildx build --push -f Dockerfile --platform=linux/amd64 -t ghcr.io/mariusrugan/credit-card-tx-api:0.1.0 .
#
# Notes: `:latest` is because chainguard's `:latest` are free to download
#
# Stage 1: Builder
# Based on: https://images.chainguard.dev/directory/image/rust/sbom
#
FROM cgr.dev/chainguard/rust:latest AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY txapi/Cargo.toml ./txapi/

# Copy source code
COPY txapi/src ./txapi/src

# Build the application in release mode with optimizations
RUN cargo build --release -p txapi

# Stage 2: Runtime image
# Based on: https://images.chainguard.dev/directory/image/glibc-dynamic/sbom
#
FROM cgr.dev/chainguard/glibc-dynamic:latest AS runtime

# OCI Labels - https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.title="Credit Card Transaction Stream API" \
      org.opencontainers.image.description="WebSocket API for streaming mock credit card transactions for fraud detection testing" \
      org.opencontainers.image.version="0.1.0" \
      org.opencontainers.image.authors="" \
      org.opencontainers.image.url="https://github.com/mariusrugan/credit-card-tx-api" \
      org.opencontainers.image.source="https://github.com/mariusrugan/credit-card-tx-api" \
      org.opencontainers.image.vendor="mariusrugan" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.documentation="https://github.com/mariusrugan/credit-card-tx-api/blob/main/README.md" \
      org.opencontainers.image.base.name="cgr.dev/chainguard/glibc-dynamic:latest"

# Application labels
LABEL app.name="txapi" \
      app.component="websocket-api" \
      app.tier="backend" \
      app.language="rust" \
      app.framework="" \
      maintainer="mariusrugan"

WORKDIR /app

# Create a non-root user (Chainguard images already have 'nonroot' user with UID 65532)
USER 65532:65532

# Copy the binary from builder with proper ownership
COPY --from=builder --chown=65532:65532 /app/target/release/txapi /app/txapi

# Expose the application port
EXPOSE 9999

# Set environment variables
ENV LOG_LEVEL=INFO \
    RUST_BACKTRACE=1 \
    PORT=9999

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/txapi", "--health"] || exit 1

# Run the application
ENTRYPOINT ["/app/txapi"]
