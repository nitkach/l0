FROM rust:1.81-bookworm as build

WORKDIR /app

COPY . .

RUN mkdir bin

RUN --mount=type=cache,id=rust-build,target=/app/target \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    cargo build --target-dir /app/target \
    && cp /app/target/debug/l0 /app/bin

FROM debian:bookworm

WORKDIR /app

COPY --from=build /app/bin/l0 /app/bin/l0

CMD ["/app/bin/l0"]
