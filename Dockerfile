# Multi-stage build for RaspberryPi (ARM)
FROM --platform=linux/arm64 rust:latest as builder

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install trunk for frontend builds
RUN cargo install trunk

# Install wasm32 target for frontend compilation
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy workspace manifests
COPY Cargo.toml ./
COPY common/Cargo.toml ./common/
COPY backend/Cargo.toml ./backend/
COPY frontend/Cargo.toml ./frontend/

# Copy source code
COPY common/src ./common/src
COPY backend/src ./backend/src
COPY frontend/src ./frontend/src
COPY frontend/index.html ./frontend/
COPY frontend/style.css ./frontend/
COPY frontend/Trunk.toml ./frontend/

# Build backend
RUN cargo build --release --bin backend

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Runtime stage
FROM --platform=linux/arm64 debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Create app directory
WORKDIR /app

# Copy backend binary
COPY --from=builder /app/target/release/backend ./backend

# Copy frontend static files
COPY --from=builder /app/frontend/dist ./static

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose port
EXPOSE 3000

# Set environment variable for static files path
ENV STATIC_FILES_PATH=/app/static

# Start the backend server
CMD ["./backend"] 