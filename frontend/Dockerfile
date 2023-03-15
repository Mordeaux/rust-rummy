# syntax=docker/dockerfile:1.4
FROM node:lts-alpine3.17 AS development

WORKDIR /app
COPY package.json /app
COPY yarn.lock /app
COPY . /app
RUN yarn install
EXPOSE 3000
