# syntax=docker/dockerfile:1.4
FROM python:3.11-alpine AS development
ENV VENV_PATH=/.poetry_venv

WORKDIR /app

RUN apk add python3-dev libpq-dev gcc musl-dev
COPY pyproject.toml /app
COPY poetry.lock /app
RUN python3 -m venv $VENV_PATH \
    && $VENV_PATH/bin/pip install -U pip setuptools \
    && $VENV_PATH/bin/pip install poetry
COPY . /app
RUN $VENV_PATH/bin/poetry install
