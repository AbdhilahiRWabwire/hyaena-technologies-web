# Cargo Build
.PHONY: build
build:
	cargo build
	
# Cargo Build Release Mode
.PHONY: build-release
build-release:
	cargo build --release
