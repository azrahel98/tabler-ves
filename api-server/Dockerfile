FROM debian:bookworm-slim

# Instalamos las librerías mínimas para que el binario de Rust corra
# (OpenSSL y certificados para que Actix pueda salir a internet)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiamos el binario que acabas de compilar localmente
COPY target/release/api-server .


# Exponemos el puerto
EXPOSE 4010
# Ejecutamos
CMD ["./api-server"]

#cargo build --release --bin api-server
#docker build -t azrahel9/api-server:latest .
#docker push azrahel9/api-server:latest