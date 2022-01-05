.PHONY: help
help:
	@grep -E '^[a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'


.PHONY: test
test: ## Run all tests
	cargo test --locked

.PHONY: lint
lint: clippy rustfmt ## Run all linters

.PHONY: clippy
clippy: ## Run clippy
	cargo clippy --locked -- -D warnings

.PHONY: rustfmt
rustfmt: ## Run rustfmt
	cargo fmt --all -- --check