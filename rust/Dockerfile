# syntax=docker/dockerfile:1.4
FROM rust:alpine3.17 AS development

RUN rustup target add wasm32-unknown-unknown
RUN apk add musl-dev
RUN cargo install cargo-watch

WORKDIR /app
COPY Cargo.toml /app
COPY Cargo.lock /app
COPY . /app
RUN cargo fetch
