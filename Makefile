export PROJECTNAME=$(shell basename "$(PWD)")
CARGO := cargo
.SILENT: ;               # no need for @

clean: ## Clean build artifacts
	$(CARGO) clean

build: ## Development build
	$(CARGO) build

rebuild: clean ## Development clean and build
	$(CARGO) build

release: ## Release build
	$(CARGO) build --release

run: ## Run the program (development build)
	$(CARGO) run

run-args: ## Run the program with arguments (usage: make run-args ARGS="--your-args-here")
	$(CARGO) run -- $(ARGS)

install: ## Install release binary to system
	$(CARGO) install --path .

outdated: ## Check for outdated dependencies
	$(CARGO) outdated

setup-dev: ## Install development dependencies
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-outdated

.PHONY: help
.DEFAULT_GOAL := help

help: Makefile
	echo
	echo " Choose a command run in "$(PROJECTNAME)":"
	echo
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	echo
