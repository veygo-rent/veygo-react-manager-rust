FROM rust:slim

WORKDIR /app

COPY . ./

RUN apt update && apt install curl pkg-config git -y

RUN cargo build --release

ENTRYPOINT ./target/release/veygo-task-manager-rust