# syntax=docker/dockerfile:1

FROM rust:1.94-slim-bookworm AS builder

WORKDIR /usr/src/sakemas-bot
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    cmake \
    g++ \
    make \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ffmpeg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/sakemas-bot/target/release/sakemas-bot /app/sakemas-bot
COPY --from=builder /usr/src/sakemas-bot/migrations /app/migrations

ENV RUST_LOG=info

CMD ["/app/sakemas-bot"]
