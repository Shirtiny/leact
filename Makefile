.DEFAULT_GOAL := init

p ?= web

init:
	@echo "Initializing the project"
	cargo install cargo-binstall
	cargo binstall dioxus-cli # brew install openssl@3
	pnpm i
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css

watch:
	@echo "Watching... $(p)"
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch	

start:
	@echo "Starting... $(p)"
	dx serve --platform $(p)

.PHONY: web desktop mobile
web desktop mobile:
	@$(MAKE) start p=$@

bundle:
	@echo "Building...$(p)"
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify
	dx bundle --platform $(p) --release --features bundle