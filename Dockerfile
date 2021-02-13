FROM rust:1.50-alpine as builder
WORKDIR /usr/src/ludtwig
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ludtwig /usr/local/bin/ludtwig
