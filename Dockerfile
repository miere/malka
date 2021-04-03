# BUILDER IMAGE
FROM messense/rust-musl-cross:x86_64-musl as BUILDER

RUN apt-get update -y \
 && apt-get install -y ca-certificates \
 && mkdir /src \
 && rustup target add x86_64-unknown-linux-musl

WORKDIR /src
COPY . /src
RUN cargo build \
    --target x86_64-unknown-linux-musl \
    --release

# FINAL IMAGE
FROM scratch
COPY --from=BUILDER \
    /src/target/x86_64-unknown-linux-musl/release/malka-consumer \
    /usr/bin/malka-consumer

COPY --from=BUILDER \
    /etc/ssl/certs/* \
    /etc/ssl/certs/

ENTRYPOINT ["/usr/bin/malka-consumer"]
