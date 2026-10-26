FROM rust:latest AS build

RUN apt-get update -y
RUN apt-get install libclang-dev libssl-dev cmake -y

WORKDIR /app
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update -y
RUN apt-get install openssl ca-certificates -y

RUN groupadd -g 1000 komodo-email-alerter
RUN useradd -s /bin/sh -u 1000 -g komodo-email-alerter komodo-email-alerter

WORKDIR /app
COPY --from=build --chmod=777 /app/target/release/komodo-email-alerter .

COPY templates /templates

ENV TEMPLATE_DIR=/templates
ENV LISTEN_ADDRESS="0.0.0.0:80"
EXPOSE 80
CMD ["/app/komodo-email-alerter"]
