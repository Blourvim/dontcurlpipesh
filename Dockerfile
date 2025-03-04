FROM rust:bookworm

WORKDIR /code
COPY Cargo.toml Cargo.toml
COPY ./src/ ./src/

RUN cargo build --release

