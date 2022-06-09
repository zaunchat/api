FROM rustlang/rust:nightly-slim AS builder
USER 0:0
WORKDIR /home/rust/src

RUN USER=root cargo new --bin api
WORKDIR /home/rust/src/api
RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY Cargo.toml Cargo.lock ./
COPY assets ./assets
COPY src ./src
RUN cargo install --locked --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/api ./
EXPOSE 8080
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT 8080
CMD ["./api"]