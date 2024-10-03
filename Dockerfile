FROM rust:latest as builder
WORKDIR /build
RUN apt-get update && \
    apt-get install -y curl libssl-dev pkg-config build-essential
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /build/target/release/rust_api .
EXPOSE 8080
CMD ["./rust_api"]
