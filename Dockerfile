FROM rust:1.77-slim as builder
WORKDIR /usr/src/rust-birkana-http
COPY . .
RUN --mount=type=cache,target=/usr/src/rust-birkana-http/target/.fingerprint,rw \
    --mount=type=cache,target=/usr/src/rust-birkana-http/target/build,rw \
    --mount=type=cache,target=/usr/src/rust-birkana-http/target/deps,rw \
    --mount=type=cache,target=/usr/src/rust-birkana-http/target/examples,rw \
    --mount=type=cache,target=/usr/src/rust-birkana-http/target/incremental,rw \
    --mount=type=cache,target=/usr/local/cargo/registry,rw \
    cargo build --release

FROM debian:bookworm-slim
EXPOSE 8080
RUN mkdir /app
COPY --from=builder /usr/src/rust-birkana-http/target/release/rust-birkana-http /app/rust-birkana-http
COPY --from=builder /usr/src/rust-birkana-http/static /app/static
COPY --from=builder /usr/src/rust-birkana-http/templates /app/templates
WORKDIR /app
CMD ["/app/rust-birkana-http"]
