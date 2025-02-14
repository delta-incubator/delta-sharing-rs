set dotenv-load := true

dat_version := "0.0.3"
dat_dir := "dat"
local_config := "config/local.yaml"

# Show available commands
default:
    @just --list --justfile {{ justfile() }}

# Run clippy with fixes
fix:
    @cargo clippy --fix --allow-dirty --allow-staged

# the the documentation (requires mdbook)
docs:
    cd docs && mdbook serve --open

# Run unit tests
test:
    @cargo test --workspace -- --nocapture

# Run integration tests
test-integration: start_pg
    DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres \
        cargo test -p delta-sharing-postgres --features integration-pg -- --nocapture
    @just stop_pg

# Build a docker image for local use
docker-build:
    DOCKER_BUILDKIT=0 docker build . -t delta-sharing:dev -f docker/Dockerfile

# Run a docker image for local use
docker-run:
    docker run -it --rm \
      -p 8080:8080 \
      -v "$(pwd)/config/empty.yaml:/config.yaml" \
      -e RUST_LOG=debug \
      delta-sharing:dev rest

# Run local docker emvironment
docker:
    docker-compose -f docker/compose.yaml up -d

# generate delta-sharing types from proto files
generate:
    buf generate proto
    just delta-sharing/openfga/generate
    npx -y @redocly/cli bundle --remove-unused-components openapi/openapi.yaml > tmp.yaml
    mv tmp.yaml openapi/openapi.yaml
    cargo clippy --fix --allow-dirty --allow-staged

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

rest:
    @RUST_LOG=DEBUG cargo run --bin delta-sharing rest --config {{ local_config }}

grpc:
    @RUST_LOG=DEBUG cargo run --bin delta-sharing grpc --config {{ local_config }}

load-store:
    fga store import --file {{ justfile_directory() }}/delta-sharing/openfga/fga/dev.fga.yaml

# Show unused dependencies
udeps:
    cargo +nightly udeps

sqlx-prepare: start_pg
    # Wait for PostgreSQL to be ready
    sleep 1
    # Run migrations to create tables
    DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres cargo sqlx migrate run --source ./delta-sharing/postgres/migrations
    # Prepare SQLx
    cargo sqlx prepare --workspace -- --tests
    # Clean up
    @just stop_pg

# Start PostgreSQL container to prepare SQLx or to run tests
start_pg:
    docker run -d \
        --name postgres-sharing \
        -e POSTGRES_PASSWORD=postgres \
        -e POSTGRES_USER=postgres \
        -e POSTGRES_DB=postgres \
        -p 5432:5432 \
        postgres:14

# Stop PostgreSQL container
stop_pg:
    docker stop postgres-sharing && docker rm postgres-sharing
