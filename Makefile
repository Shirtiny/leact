.DEFAULT_GOAL := init

p ?= web

init:
	@echo "Initializing the project"
	pnpm i
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css

start:
	@echo "Starting... $(p)"
	dx serve --platform $(p) &
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

.PHONY: web desktop mobile
web desktop mobile:
	@$(MAKE) start p=$@

bundle:
	@echo "Building...$(p)"
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify
	dx bundle --platform $(p) --release --features bundle
