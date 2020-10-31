FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/subscription-service

COPY . .

RUN cargo install --path .

FROM debian:buster-slim as production

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /subscription-service

COPY --from=builder /usr/local/cargo/bin/subscription-service /subscription-service

COPY ./environments/* /subscription-service/environments

CMD ["/subscription-service/subscription-service"]
