FROM node:iron-bookworm-slim as tailwind-builder

RUN USER=root

WORKDIR /tmp

COPY ./static ./static
COPY ./tailwind ./tailwind

WORKDIR /tmp/tailwind

RUN npm install
RUN npm run build

# 

FROM rust:slim-bookworm as builder

RUN USER=root cargo new --bin maeevick

WORKDIR /maeevick

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./static/ ./static/

RUN rm ./target/release/deps/maeevick*
RUN cargo build --release

# 

FROM debian:bookworm-slim

COPY --from=builder /maeevick/target/release/maeevick .
COPY --from=tailwind-builder /tmp/static ./static

CMD ["./maeevick"]