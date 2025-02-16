# Cargo Build Package
.PHONY: build
build:
	cargo build
	
# Cargo Build Package Release Mode
.PHONY: build-release
build-release:
	cargo build --release

# Cargo Check Package
.PHONY: check
check:
	cargo check

# Cargo Document Package
.PHONY: doc
doc:
	cargo doc

# Cargo Test Package
.PHONY: test
test:
	cargo test

# Cargo Update Dependencies
.PHONY: update
update:
	cargo update

# Docker Build Package
.PHONY: docker-build
docker-build:
	docker build ./ --file ./hyaena-technologies.Dockerfile --tag hyaena-technologies-server:latest
