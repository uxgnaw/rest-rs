# syntax = docker/dockerfile:experimental
FROM rust:latest

RUN cargo install sccache

ENV HOME=/home/root
ENV SCCACHE_CACHE_SIZE="1G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

WORKDIR /rest
COPY . .
RUN --mount=type=cache,target=/home/root/.cache/sccache
RUN cargo build --release
ENTRYPOINT ["target/release/rest"]
