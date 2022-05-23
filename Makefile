WEB_IMAGE_NAME ?= ghcr.io/hayashikun/kingsol-web
GRPC_WEB_IMAGE_NAME ?= ghcr.io/hayashikun/kingsol-grpc-web

up/all: up/web up/grpc-web


up/web: build/web push/web

build/web:
	docker build -t $(WEB_IMAGE_NAME) -f docker/web/Dockerfile .

push/web:
	docker push $(WEB_IMAGE_NAME)


up/grpc-web: build/grpc-web push/grpc-web

build/grpc-web:
	docker build -t $(GRPC_WEB_IMAGE_NAME) -f docker/grpc-web/Dockerfile .

push/grpc-web:
	docker push $(GRPC_WEB_IMAGE_NAME)
