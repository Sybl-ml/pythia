FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release --bin pythia

FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/pythia /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pythia"]
