FROM rust:1.96-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY services/projectx-core ./services/projectx-core
COPY shared ./shared

RUN cargo build -p projectx-core

FROM rust:1.96-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/debug/projectx-core /projectx-core

EXPOSE 8080

CMD ["/projectx-core"]