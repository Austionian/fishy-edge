FROM lukemathwalker/cargo-chef:latest-rust-1.67.0 AS chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . . 
ENV SQLX_OFFLINE true
RUN cargo build --release --bin fishy_edge

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/fishy_edge fishy_edge
COPY config config
ENV APP_ENVIRONMENT production 
ENTRYPOINT ["./fishy_edge"]
