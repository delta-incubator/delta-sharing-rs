set dotenv-load := true

dat_version := "0.0.3"
dat_dir := "dat"
local_config := "config/local.yaml"

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
    docker-compose -f docker/compose.yaml up -d

# Run server locally
server:
    @RUST_BACKTRACE=1 cargo run -- server

# Build delta-sharing-rs into a docker image for local use
package:
    DOCKER_BUILDKIT=0 docker build . -t delta-sharing:dev -f docker/Dockerfile

# generate delta-sharing types from proto files
generate:
    @buf generate proto

# run the delta-sharing server with the dev config
do-it:
    @RUST_BACKTRACE=1 cargo run -p delta-sharing server --config {{ local_config }}

# the the documentation (requires mdbook)
doc:
    cd docs && mdbook serve --open

# load delta acceptance testing (dat) data from the release
load-dat:
    rm -rf {{ dat_dir }}
    curl -OL https://github.com/delta-incubator/dat/releases/download/v{{ dat_version }}/deltalake-dat-v{{ dat_version }}.tar.gz
    mkdir -p {{ dat_dir }}
    tar  --no-same-permissions -xzf deltalake-dat-v{{ dat_version }}.tar.gz --directory {{ dat_dir }}
    rm deltalake-dat-v{{ dat_version }}.tar.gz

render-config:
    DIRECTORY={{ justfile_directory() }} DAT={{ dat_dir }} envsubst < config/local.yaml.tpl > {{ local_config }}

# local setup
local-setup: load-dat render-config

profile:
    @cargo run -p delta-sharing -- profile \
        -e https://localhost:8080 \
        --subject someone@email.com \
        --validity 90 \
        --shares asdf \
        --secret secret

# run the delta-sharing server with the dev config
rest:
    @RUST_LOG=DEBUG cargo run -p delta-sharing rest --config {{ local_config }}

grpc:
    @RUST_LOG=DEBUG cargo run -p delta-sharing grpc --config {{ local_config }}

test-common:
    cargo test -p delta-sharing-common
