# Show this help
help:
    @just --list

# Conduct a rust checking
check:
    @cargo check

# Clean up the pre-build target
clean:
    @cargo clean

# Conduct a full release build
build:
    @# This causes cargo to have to rebuild the binary,
    @# even if no Rust code has changed either.
    @cargo build --release

# Conduct unit tests
test:
    @cargo test -- --nocapture

# Conduct DB-related unit tests
testdb:
    @# Be sure run the following command before conducting this:
    @# $ docker compose -f ./devops/local/docker-compose.yaml up
    @cargo test -- --nocapture --ignored

# Run local docker emvironment
docker:
    @docker compose -f devops/local/docker-compose.yaml up

# Run server locally
server:
    @cargo run -- server

# Build Kotosiro into a docker image for local use
package:
    DOCKER_BUILDKIT=1 docker build . -t kotosiro-sharing:local -f devops/docker/Dockerfile