FROM rust:slim

WORKDIR /app

COPY . ./

RUN apt update && apt install curl pkg-config nodejs npm -y

RUN cargo build --release

ENTRYPOINT ./target/release/veygo-react-manager-rust