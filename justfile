# Build the project in release mode
build:
    cargo build --release

# Run all tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Format code with rustfmt
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Install dependencies
install:
    cargo fetch

# Run the application
run: build
    ./target/release/lazymake

# Build and run tests
test-all: build test
    echo "All tests passed!"

# Development build (faster, unoptimized)
dev:
    cargo build

# Check code without building
check:
    cargo check

# Generate documentation
docs:
    cargo doc --no-deps --open
