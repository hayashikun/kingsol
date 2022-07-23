WEB_IMAGE_NAME ?= ghcr.io/hayashikun/kingsol-web
GRPC_WEB_IMAGE_NAME ?= ghcr.io/hayashikun/kingsol-grpc-web

up:
	docker-compose up

down:
	docker-compose down

push/all: push/grpc-web push/web

build/web:
	docker build -t $(WEB_IMAGE_NAME) -f docker/web/Dockerfile .

push/web: build/web
	docker push $(WEB_IMAGE_NAME)

build/grpc-web:
	docker build -t $(GRPC_WEB_IMAGE_NAME) -f docker/grpc-web/Dockerfile .

push/grpc-web: build/web
	docker push $(GRPC_WEB_IMAGE_NAME)

skeema/push:
	docker-compose exec web sh -c 'cd mysql && skeema push dev --allow-unsafe'

skeema/lint:
	docker-compose exec web sh -c 'cd mysql && skeema lint dev'

.PHONY: mysql
mysql:
	docker-compose exec mysql mysql -u root
