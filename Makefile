up:
	docker-compose up -d --build

down:
	docker-compose down

logs:
	docker-compose logs -f

dockerfile=Dockerfile

ifeq ($(MODE),dev)
	dockerfile:=$(dockerfile)-dev
endif

build:
	docker build . -t app --file=$(dockerfile)
