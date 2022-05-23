FROM rust:latest as builder

RUN apt-get update && apt-get install cmake -y

WORKDIR /build
COPY . .
RUN cargo build --release --package kingsol --bin web

FROM rust:latest
COPY --from=builder /build/target/release/web /usr/local/bin/

CMD ["/usr/local/bin/web"]
