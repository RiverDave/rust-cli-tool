# Simple Makefile for cli-rust project

.PHONY: help build test clean fmt clippy check all

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

fmt: ## Format the code
	cargo fmt

fmt-check: ## Check if code is formatted
	cargo fmt -- --check

clippy: ## Run clippy lints
	cargo clippy

check: ## Run all checks (fmt, clippy, test)
	cargo fmt -- --check
	cargo clippy
	cargo test

all: clean fmt clippy test build ## Run all checks and build