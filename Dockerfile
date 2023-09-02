FROM rust:1.72

WORKDIR /app

RUN curl -sL https://deb.nodesource.com/setup_18.x -o nodesource_setup.sh
RUN apt-get update
RUN apt-get install -y nodejs

COPY . .

WORKDIR "/app/crates/flashpoint-gotd-webserver/"
RUN cargo build
CMD ["/bin/bash", "-c", "cargo run"]
