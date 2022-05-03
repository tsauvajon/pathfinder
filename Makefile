# By default make command will display help
.DEFAULT_GOAL := help
.PHONY: serve build

build: ## Build the app for static serving
	trunk build

dependencies: ## Get the development dependencies
	cargo install miniserve

dev: ## Hot reload the app
	trunk serve

serve: build ## Serve locally on port 8888
	miniserve ./docs --index index.html -p 8888

help: ## Print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
