.DEFAULT_GOAL := build

init:
	@echo "Initialising the project"
	
start:
	@echo "Starting..."
	dx serve

bundle:
	@echo "👩‍🏭 Building..."
	dx bundle --platform desktop --release --features bundle
