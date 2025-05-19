FROM rust:1.87.0-alpine3.21 AS base
WORKDIR /app
RUN apk add --no-cache musl-dev \
 && cargo install cargo-chef

FROM base AS src
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY tests/ ./tests/

FROM src AS test
RUN cargo check \
 && cargo test

FROM src AS planner
RUN cargo chef prepare --recipe-path recipe.json

FROM src AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.21
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/github-profile-toolbox /usr/local/bin/
ENTRYPOINT ["github-profile-toolbox"]
