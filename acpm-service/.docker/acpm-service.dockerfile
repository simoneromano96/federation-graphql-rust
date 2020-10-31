# FROM rust as builder
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/acpm-service

COPY . .

RUN cargo install --path .

FROM debian:slim as production

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /acpm-service

COPY --from=builder /usr/local/cargo/bin/acpm-service /acpm-service

COPY ./environments/* /acpm-service/environments/

CMD ["/acpm-service/acpm-service"]
