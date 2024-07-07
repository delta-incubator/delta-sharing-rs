FROM rust:1.79 as build

WORKDIR /home/rust

COPY . .

RUN cargo build -p delta-sharing --release && mv /home/rust/target/release/delta-sharing /usr/bin

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/bin/delta-sharing /usr/bin

ENTRYPOINT ["/usr/bin/delta-sharing"]

CMD ["--help"]
