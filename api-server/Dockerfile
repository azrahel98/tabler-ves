FROM rust:1-slim-bookworm as chef
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin api-server

# Runtime stage
FROM debian:bookworm-slim as runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 4000

ENTRYPOINT ["/usr/local/bin/api-server"]