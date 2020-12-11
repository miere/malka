ARG RUST_VERSION=1.45.2
FROM rust:${RUST_VERSION}

RUN apt-get update && apt-get install -y \
    build-essential pkg-config cmake \
    curl openssl libssl-dev \
    python valgrind zlib1g-dev; \
    mkdir /src

WORKDIR /src
COPY . /src
RUN cargo build