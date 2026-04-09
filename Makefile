.PHONY: up down build migration

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

build:
	docker image prune -f
	docker compose -f docker/docker-compose.yml build

migration:
	cd ./core && sea-orm-cli migrate generate $(NAME)

entities:
	sea-orm-cli generate entity -o ./core/src/data/entity --database-url postgresql://admin:root@localhost:5432/lemcol