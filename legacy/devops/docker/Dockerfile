FROM rust:1.67 as build

WORKDIR /home/rust

COPY . .

RUN cargo build --release && mv /home/rust/target/release/delta-sharing /usr/bin


FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/bin/delta-sharing /usr/bin

ENTRYPOINT ["/usr/bin/delta-sharing"]

CMD ["--help"]
