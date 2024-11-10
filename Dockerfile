FROM rust:1.82.0-alpine3.20 AS base
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .

FROM base AS test
RUN cargo test

FROM base AS builder
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /app/target/release/github-profile-toolbox /usr/local/bin/github-profile-toolbox
ENTRYPOINT ["github-profile-toolbox"]
