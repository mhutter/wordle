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

.PHONY: clean
clean:
	rm -f words.txt

words.txt: JS=$(shell curl -sS https://www.powerlanguage.co.uk/wordle/ | grep -Po '(?<=<script src=")main\.([^.]+)\.js(?=">)')
words.txt: ## Download latest word list
	curl -sS "https://www.powerlanguage.co.uk/wordle/$(JS)" | grep -Eo '"[a-z]{5}"' | tr -d '"' | sort -u > "$@"
