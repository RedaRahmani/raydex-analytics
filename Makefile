DOCKER_COMPOSE := docker-compose -f infra/docker-compose.yml

.PHONY: up down logs-kafka logs-clickhouse ps

up:
	$(DOCKER_COMPOSE) up -d

down:
	$(DOCKER_COMPOSE) down

ps:
	$(DOCKER_COMPOSE) ps

logs-kafka:
	$(DOCKER_COMPOSE) logs -f kafka

logs-clickhouse:
	$(DOCKER_COMPOSE) logs -f clickhouse
