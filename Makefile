.DEFAULT_GOAL := init

p ?= web

init:
	@echo "Initializing the project"
	pnpm i
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css

start: init
	@echo "Starting... $(p)"
	dx serve --platform $(p)

.PHONY: web desktop mobile
web desktop mobile:
	@$(MAKE) start p=$@

bundle:
	@echo "Building...$(p)"
	dx bundle --platform $(p) --release --features bundle
