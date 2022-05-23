FROM rust:latest as builder

RUN apt-get update && apt-get install cmake -y

WORKDIR /build
COPY . .
RUN cargo build --release --package kingsol --bin grpc-web

FROM rust:latest
COPY --from=builder /build/target/release/grpc-web /usr/local/bin/

CMD ["/usr/local/bin/grpc-web"]
