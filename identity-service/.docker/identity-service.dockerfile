FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/identity-service

COPY . .

RUN cargo install --path .

FROM debian:stable-slim as production

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /identity-service

COPY --from=builder /usr/local/cargo/bin/identity-service /identity-service

COPY ./environments/* /identity-service/environments/

CMD ["/identity-service/identity-service"]
