FROM rust:1.67 as build

WORKDIR /home/rust

COPY . .

RUN cargo build --release && mv /home/rust/target/release/kotosiro-sharing /usr/bin


FROM debian:bullseye-slim

COPY --from=build /usr/bin/kotosiro-sharing /usr/bin

ENTRYPOINT ["/usr/bin/kotosiro-sharing"]

CMD ["--help"]
