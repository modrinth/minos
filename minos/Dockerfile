# Start with the Rust base image
FROM rust:1.67 as builder
WORKDIR /usr/src/minos

# Copy Cargo.toml and Cargo.lock to leverage Docker's caching capabilities
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build only the dependencies
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the source code and build the application
COPY . .
RUN cargo install --path .

# Create the final image with the built binary
FROM debian:buster-slim
# Install the necessary OpenSSL libraries
RUN apt-get update && apt-get install -y openssl libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/minos /usr/local/bin/minos
CMD ["minos"]
FROM rust:1.67 as builder
WORKDIR /usr/src/minos
COPY . .
RUN cargo install --path .

CMD ["minos"]
