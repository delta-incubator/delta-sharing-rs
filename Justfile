set dotenv-load := true

dat_version := "0.0.3"
dat_dir := "dat"
local_config := "config/local.yaml"

# Show available commands
default:
    @just --list --justfile {{ justfile() }}

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
    @just delta-sharing/openfga/generate
    @just clean-openapi

clean-openapi:
    npx @redocly/cli bundle --remove-unused-components openapi/openapi.yaml > tmp.yaml
    mv tmp.yaml openapi/openapi.yaml

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

test-common:
    cargo test -p delta-sharing-common

# run the delta-sharing server with the dev config
rest:
    @RUST_LOG=DEBUG cargo run -p delta-sharing rest --config {{ local_config }}

grpc:
    @RUST_LOG=DEBUG cargo run -p delta-sharing grpc --config {{ local_config }}

load-store:
    fga store import --file {{ justfile_directory() }}/delta-sharing/openfga/fga/dev.fga.yaml

# Show unused dependencies
udeps:
    cargo +nightly udeps

sqlx-prepare:
    cargo sqlx prepare --workspace -- --tests
