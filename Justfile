set dotenv-load := true

# list all availabe commands
default:
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
    @cargo test --lib -- --nocapture

# Conduct DB-related unit tests
test-integration:
    @# Be sure run the following command before conducting this:
    @# $ docker compose -f ./devops/local/docker-compose.yaml up
    @cargo test --tests -- --nocapture

alias testdb := test-integration

# Run local docker emvironment
docker:
    @docker compose -f devops/local/docker-compose.yaml up -d

# Run server locally
server:
    @RUST_BACKTRACE=1 cargo run -- server

# Build delta-sharing-rs into a docker image for local use
package:
    DOCKER_BUILDKIT=0 docker build . -t delta-sharing:local -f devops/docker/Dockerfile

# generate delta-sharing types from proto files
generate:
    @buf generate proto

# run the delta-sharing server with the dev config
run:
    @RUST_BACKTRACE=1 cargo run -p delta-sharing-server -- --config ./config/dev.yaml

# the the documentation (requires mdbook)
doc:
    cd docs && mdbook serve --open
