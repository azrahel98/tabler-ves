FROM rust:1.86.0 as builder
WORKDIR /app
COPY . .

# Ya no necesitas ENV SQLX_OFFLINE=true porque no hay macros de compilación
RUN cargo build --release --bin api-server

FROM debian:bookworm-slim
# Instalar librerías necesarias para MariaDB/MySQL si usas native-tls
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-server /usr/local/bin/
CMD ["api-server"]