# Start with the Rust base image
FROM rust:1.67 as builder
WORKDIR /usr/src/minos

# Copy over your manifest
COPY Cargo.toml Cargo.lock ./

# This dummy build is to get the dependencies cached
RUN mkdir -p src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm src/main.rs

# Copy sqlx-data.json
COPY sqlx-data.json ./

# Now copy your actual source code
COPY src ./src
RUN touch -a -m ./src/main.rs


# Build for real
RUN cargo build --release --manifest-path ./Cargo.toml

# Create the final image with the built binary
FROM debian:buster-slim
# Install the necessary OpenSSL libraries
RUN apt-get update && apt-get install -y openssl libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/minos/target/release/minos /usr/local/bin/minos
CMD ["minos"]
