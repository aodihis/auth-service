# Makefile

APP_NAME := myapp

# Load .env if present
ifneq (,$(wildcard .env))
	include .env
	export
endif

.PHONY: run build test fmt lint clean migrate create-migration redo

## Run the app
run:
	cargo run

## Build the app
build:
	cargo build

## Run tests
test:
	cargo test

## Format code
fmt:
	cargo fmt

## Lint code
lint:
	cargo clippy --all-targets --all-features -- -D warnings

## Clean target directory
clean:
	cargo clean

## Run migrations using sqlx
migrate:
	@if [ -z "$$DATABASE_URL" ]; then \
		echo "DATABASE_URL is not set. Set it in your .env file or export it."; \
		exit 1; \
	fi
	sqlx migrate run

## Create a new migration: make create-migration name=create_users
create-migration:
ifndef name
	$(error "You must provide a migration name using name=...")
endif
	@if [ -z "$$DATABASE_URL" ]; then \
		echo "DATABASE_URL is not set. Set it in your .env file or export it."; \
		exit 1; \
	fi
	sqlx migrate add $(name)

## Redo the last migration
redo:
	@if [ -z "$$DATABASE_URL" ]; then \
		echo "DATABASE_URL is not set. Set it in your .env file or export it."; \
		exit 1; \
	fi
	sqlx migrate revert
	sqlx migrate run
