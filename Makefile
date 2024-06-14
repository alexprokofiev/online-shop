up:
	IMG=app docker-compose -f docker-compose.yml up -d

down:
	IMG=app docker-compose -f docker-compose.yml down 

logs:
	IMG=app docker-compose logs -f

build:
	docker build . -t app

build_dev:
	docker build . -t app-dev --file=Dockerfile-dev

dev:
	IMG=app-dev docker-compose -f docker-compose-dev.yml up -d

sqlx:
	DATABASE_URL=mysql://root:root@127.0.0.1:3306/shop cargo sqlx prepare
