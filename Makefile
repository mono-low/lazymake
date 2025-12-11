.PHONY: build test clean run help

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Run the application
run: build
	./target/release/lazymake

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy

# Install dependencies
install:
	cargo fetch

# Help command
help:
	@echo "Available targets:"
	@echo "  build   - Build the project"
	@echo "  test    - Run tests"
	@echo "  clean   - Clean build artifacts"
	@echo "  run     - Build and run"
	@echo "  fmt     - Format code"
	@echo "  lint    - Run clippy"
	@echo "  install - Fetch dependencies"
