FROM --platform=linux/amd64 rust:alpine

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/build

CMD ["cargo", "build", "--target", "x86_64-unknown-linux-musl", "--release"]
