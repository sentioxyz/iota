# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.97.1-trixie AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /iota
RUN apt-get update && apt-get install -y cmake clang lld libpq5 libpq-dev protobuf-compiler

COPY rust-toolchain.toml ./
COPY .cargo .cargo
COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY iota-execution iota-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin iota-trace-server

# Production Image
FROM debian:trixie-slim AS runtime
WORKDIR /iota

# sui-tool needs libpq at runtime and uses git when fetching move dependencies during build
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates git

COPY --from=builder /iota/target/release/iota-trace-server /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION