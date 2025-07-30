# Use the official Rust image as base
FROM rust:1.88 as builder

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src/ ./src/

# Build the application in release mode for better performance
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 julia

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/julia_set /app/julia_set

# Create output directory and set permissions
RUN mkdir -p /app/output && chown julia:julia /app/output

# Switch to non-root user
USER julia

# Expose volume for output files
VOLUME ["/app/output"]

# Set the entry point
ENTRYPOINT ["./julia_set"]
