FROM rust:1.96-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY services ./services
COPY shared ./shared

RUN cargo build -p web --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/web /web

EXPOSE 8080
CMD ["/web"]