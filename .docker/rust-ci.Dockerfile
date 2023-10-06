FROM rust:1.73.0

RUN rustup target add x86_64-unknown-linux-musl
