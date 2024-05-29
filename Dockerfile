ARG RUST_VERSION=1.78.0
ARG APP_NAME=todoem

FROM rust:${RUST_VERSION}
ARG APP_NAME

RUN cargo install cargo-watch

WORKDIR /app

