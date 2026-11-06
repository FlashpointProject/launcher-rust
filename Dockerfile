# syntax=docker/dockerfile:1

# Stage 1: client bundle (webpack) -> public/bundle.js
FROM node:24.22.2-trixie-slim@sha256:0820eaa85e07d0490446dd01d14707e17e6108a36988cf374899cd057eb3812d AS client
WORKDIR /src
COPY crates/flashpoint-gotd-webserver/package.json crates/flashpoint-gotd-webserver/package-lock.json ./
RUN npm ci
COPY crates/flashpoint-gotd-webserver/tsconfig.json crates/flashpoint-gotd-webserver/webpack.config.js ./
COPY crates/flashpoint-gotd-webserver/client ./client
COPY crates/flashpoint-gotd-webserver/public ./public
RUN npm run build

# Stage 2: release binary
FROM rust:1.99.1-slim-trixie@sha256:7eab7ce19273ce785e0ca7578bdf7bc1fcbae621775a8197379a04c9f6e03cd5 AS build
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release --locked -p flashpoint-gotd-webserver

# Stage 3: runtime
FROM debian:trixie-slim@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates libssl3t64 \
 && rm -rf /var/lib/apt/lists/*
# The server serves ./public relative to its working directory.
WORKDIR /app/crates/flashpoint-gotd-webserver
COPY --from=build /app/target/release/flashpoint-gotd-webserver /usr/local/bin/flashpoint-gotd-webserver
COPY --from=client /src/public ./public
# Runtime state: /data (flashpoint.sqlite, suggestions.json) and /live (gotd.json) are mounted volumes.
# Settings come from the environment: DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET, DISCORD_REDIRECT_URI, ADMIN_IDS.
EXPOSE 80
CMD ["flashpoint-gotd-webserver"]
