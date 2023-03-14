FROM rust:1.67.0-bullseye AS builder

WORKDIR /usr/src/app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release \
    && mv /usr/src/app/target/release/kotosiro-sharing /usr/bin


FROM debian:bullseye-slim

COPY --from=builder /usr/bin/kotosiro-sharing /usr/bin

ENTRYPOINT ["/usr/bin/kotosiro-sharing"]

CMD ["--help"]
