# Running a delta-sharing server

=== "cargo"

    ```sh
    cargo run --bin delta-sharing rest
    ```

=== "Docker"

    ```sh
    docker run -it --rm \
      -p 8080:8080 \
      -v "$(pwd)/config/empty.yaml:/config.yaml" \
      -e RUST_LOG=debug \
      delta-sharing:dev rest
    ```
