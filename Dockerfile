# Stage 1: Build the application
FROM rust:1.83-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new directory for the app
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 appuser

# Create app directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/newsletter_subscriber /app/newsletter_subscriber

# Copy configuration files
COPY src/configuration ./configuration

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose the port the app runs on
EXPOSE 8000

# Run the application
CMD ["./newsletter_subscriber"]
