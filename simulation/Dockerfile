# Build stage
FROM rust:latest AS build

WORKDIR /app

# Copy the source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Set environment variables
ENV RUST_LOG=info

# Install necessary graphical libraries
RUN apt-get update && apt-get install -y \
    libx11-6 \
    libxrandr2 \
    libxcursor1 \
    libwayland-client0 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the build stage
COPY --from=build /app/target/release/simulation /simulation

# Ensure the binary is executable
RUN chmod +x /simulation

# Expose the port
EXPOSE 9007

# Run the binary
CMD ["./simulation"]
