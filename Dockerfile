# syntax=docker/dockerfile:1.4
FROM rust:alpine3.17 AS base

RUN apk add musl-dev curl openssl-dev
RUN cargo install cargo-watch

WORKDIR /app
COPY ./rust/Cargo.toml /app
COPY ./rust/Cargo.lock /app
COPY ./rust /app
RUN cargo fetch


FROM base AS rust_wasm
RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


FROM python:3.11.2-alpine3.17 AS rust_python
RUN apk add --no-cache \
        ca-certificates \
        gcc libc-dev
ENV VIRTUAL_ENV=/venv
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$VIRTUAL_ENV/bin:$PATH \
    RUST_VERSION=1.68.0
RUN set -eux; \
    apkArch="$(apk --print-arch)"; \
    case "$apkArch" in \
        x86_64) rustArch='x86_64-unknown-linux-musl'; rustupSha256='241a99ff02accd2e8e0ef3a46aaa59f8d6934b1bb6e4fba158e1806ae028eb25' ;; \
        aarch64) rustArch='aarch64-unknown-linux-musl'; rustupSha256='6a2691ced61ef616ca196bab4b6ba7b0fc5a092923955106a0c8e0afa31dbce4' ;; \
        *) echo >&2 "unsupported architecture: $apkArch"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.25.2/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN pip install cffi maturin[patchelf]
COPY --from=base /app /app
WORKDIR /app
RUN cargo fetch
RUN cargo install cargo-watch
RUN maturin build --features pybindings


FROM python:3.11.2-alpine3.17 AS flask
ENV VIRTUAL_ENV=/venv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

WORKDIR /app

RUN apk add python3-dev libpq-dev gcc musl-dev
COPY ./backend/pyproject.toml /app
COPY ./backend/poetry.lock /app
COPY --from=rust_python /app/target/wheels/ /app/rust-python/
RUN python3 -m venv $VIRTUAL_ENV --copies \
    && pip install -U pip setuptools \
    && pip install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-root
COPY backend /app
EXPOSE 5000
