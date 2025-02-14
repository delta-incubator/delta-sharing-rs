FROM rust:1.84 as build

WORKDIR /home/rust

COPY . .

RUN cargo build -p delta-sharing-cli --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build /home/rust/target/release/delta-sharing /usr/bin

ENTRYPOINT ["/usr/bin/delta-sharing"]

CMD ["--help"]
