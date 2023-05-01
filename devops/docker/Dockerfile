FROM rust:1.67 as build

WORKDIR /home/rust

COPY . .

RUN cargo build --release && mv /home/rust/target/release/kotosiro-sharing /usr/bin


FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/bin/kotosiro-sharing /usr/bin

ENTRYPOINT ["/usr/bin/kotosiro-sharing"]

CMD ["--help"]
