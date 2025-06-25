FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update -y
RUN apt-get install musl-dev musl-tools clang libclang-dev cmake -y

WORKDIR /app
COPY Cargo.toml Cargo.lock .
COPY src src

RUN CXX="musl-gcc" RUSTFLAGS="-C link-arg=-s" cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

RUN addgroup -g 1000 komodo-email-alerter
RUN adduser -D -s /bin/sh -u 1000 -G komodo-email-alerter komodo-email-alerter

WORKDIR /app
COPY --from=build --chmod=777 /app/target/x86_64-unknown-linux-musl/release/komodo-email-alerter .

COPY templates /templates

EXPOSE 80
ENV TEMPLATE_DIR=/templates
ENTRYPOINT ["/app/komodo-email-alerter"]
