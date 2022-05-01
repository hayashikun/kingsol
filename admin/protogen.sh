#!/usr/bin/env bash

APP_DIR=$(dirname "$0")
TMP_DIR=$APP_DIR/tmp
PROTOC_GEN_GRPC_WEB=$TMP_DIR/protoc-gen-grpc-web

if [ ! -d "$TMP_DIR" ]; then
  mkdir "$TMP_DIR"
fi

if [ ! -e "$PROTOC_GEN_GRPC_WEB" ]; then
  # TODO: support other platform
  curl -sLJ "https://github.com/grpc/grpc-web/releases/download/1.3.1/protoc-gen-grpc-web-1.3.1-darwin-x86_64" -o "$PROTOC_GEN_GRPC_WEB"
  chmod +x "$PROTOC_GEN_GRPC_WEB"
fi

OUT_DIR="$APP_DIR/src/proto"

rm -rf "$OUT_DIR" && mkdir -p "$OUT_DIR"
protoc \
  --plugin="protoc-gen-grpc-web=$PROTOC_GEN_GRPC_WEB" \
  --js_out=import_style=commonjs,binary:"$OUT_DIR" \
  --grpc-web_out=import_style=typescript,mode=grpcweb:"$OUT_DIR" \
  -I "$APP_DIR"/../proto \
  "$(find "$APP_DIR"/../proto/*.proto)"