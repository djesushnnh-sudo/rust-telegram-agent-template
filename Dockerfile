# Multi-stage build for efficient Docker image
FROM rust:1.75-slim as builder

# Install system dependencies needed for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY tests ./tests

# Build the actual application
RUN cargo build --release

# Runtime stage - use minimal base image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false -m -d /app telegram-bot

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/telegram-bot-template /usr/local/bin/telegram-bot-template

# Change ownership to non-root user
RUN chown telegram-bot:telegram-bot /app

# Switch to non-root user
USER telegram-bot

# Expose port for webhook mode (if used)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep telegram-bot-template || exit 1

# Set default environment variables
ENV RUST_LOG=info
ENV AI_ENABLED=false

# Run the application
CMD ["telegram-bot-template"]