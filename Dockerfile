FROM golang:1.18.3 as builder
WORKDIR /opt

RUN apt-get update -y && \
  apt-get install -y \
  unzip

RUN go install github.com/skeema/skeema@latest

RUN PROTOC_ZIP=protoc-21.2-linux-$( [[ $(uname -m) == 'aarch64' ]] && echo 'aarch_64' || echo 'x86_64' ).zip && \
  curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.2/$PROTOC_ZIP && \
  unzip -o $PROTOC_ZIP -d /usr/local bin/protoc && \
  unzip -o $PROTOC_ZIP -d /usr/local 'include/*'

FROM rust:latest

COPY --from=builder /go/bin/skeema /usr/local/bin/
COPY --from=builder /usr/local/bin/protoc /usr/local/bin/
COPY --from=builder /usr/local/include/* /usr/local/include/

RUN cargo install cargo-watch

WORKDIR /app
