# Build stage
FROM rust:1-slim-bookworm as chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
# If you are using SQLx with compile-time verification, make sure you have run `cargo sqlx prepare`
# and have the `.sqlx` directory committed, or have a running database available.
# otherwise set ENV SQLX_OFFLINE=true
RUN cargo build --release --bin api-server

# Runtime stage
FROM debian:bookworm-slim as runtime
WORKDIR /app
# Install OpenSSL and CA certificates needed for HTTPS/TLS
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

# Expose the application port
EXPOSE 4000

# Run the application
ENTRYPOINT ["/usr/local/bin/api-server"]
