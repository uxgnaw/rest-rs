FROM rust:latest
WORKDIR /rest
COPY . .
RUN cargo build --release
ENTRYPOINT ["target/release/rest"]
