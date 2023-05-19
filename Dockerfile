FROM rust:1.69 as build

RUN USER=root cargo new --bin daxx
WORKDIR /daxx

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/daxx*
RUN cargo build --release

FROM debian:buster-slim

# Install OpenSSL dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*


COPY --from=build /daxx/target/release/daxx .

CMD ["./daxx"]