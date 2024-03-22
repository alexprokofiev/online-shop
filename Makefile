up:
	IMG=app docker-compose -f docker-compose.yml up -d

down:
	docker-compose down

logs:
	docker-compose logs -f

build:
	docker build . -t app

build_dev:
	docker build . -t app-dev --file=Dockerfile-dev

dev:
	IMG=app-dev docker-compose -f docker-compose-dev.yml up -d
