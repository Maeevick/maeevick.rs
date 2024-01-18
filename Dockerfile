FROM rust:1.75.0-bookworm as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN USER=root cargo new --bin maeevick
WORKDIR /maeevick

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./index.html .
COPY ./tailwind.config.js .

RUN rm ./target/release/deps/maeevick*
RUN trunk build --release

FROM nginx:latest

COPY --from=build /maeevick/dist /usr/share/nginx/html