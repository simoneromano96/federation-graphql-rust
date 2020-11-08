FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/products-service

COPY . .

RUN cargo install --path .

FROM debian:stable-slim as production

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /products-service

COPY --from=builder /usr/local/cargo/bin/products-service /products-service

COPY ./environments/* /products-service/environments/

CMD ["/products-service/products-service"]
