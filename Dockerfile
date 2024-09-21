# !!!⚠️ THIS DOCKERFILE IS NOT READY YET ⚠️!!!

FROM rust:1.78.0-bullseye AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo install --path .

FROM ubuntu:22.04

WORKDIR /usr/local/bin

COPY --from=builder /usr/local/cargo/bin/nodura-main-website .

ENV ROCKET_ENV=production

EXPOSE 8080

CMD ["./nodura-main-website"]
