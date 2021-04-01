# BUILDER IMAGE
FROM messense/rust-musl-cross:x86_64-musl as BUILDER

RUN apt-get update \
 && apt-get update -y \
 && mkdir /src \
 && rustup target add x86_64-unknown-linux-musl

WORKDIR /src
COPY . /src
RUN cargo build \
    --target x86_64-unknown-linux-musl \
    --release \
 && ls -R target

# FINAL IMAGE
FROM scratch
COPY --from=BUILDER \
    /src/target/x86_64-unknown-linux-musl/release/malka-consumer \
    /usr/bin/application

ENTRYPOINT ["/usr/bin/application"]
