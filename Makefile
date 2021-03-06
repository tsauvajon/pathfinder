# By default make command will display help
.DEFAULT_GOAL := help
.PHONY: serve build

build: ## Build the Yew app
	wasm-pack build --target web --out-name wasm --out-dir ./docs

dependencies: ## Get the development dependencies
	cargo install miniserve

serve: ## Serve locally on port 8888
	miniserve ./docs --index index.html -p 8888

help: ## Print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
