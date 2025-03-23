.PHONY: build test clean docs example

# Default target
all: build

# Build the compiler
build:
	cargo build

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Generate documentation
docs:
	cargo doc --no-deps

# Check code formatting
fmt:
	cargo fmt -- --check

# Fix code formatting
fmt-fix:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Build in release mode
release:
	cargo build --release

# Install the compiler
install: release
	cargo install --path .

# Update dependencies
update:
	cargo update

# Run the compiler with arguments
run:
	cargo run -- $(ARGS)

# Generate the specs documentation
specs:
	@echo "Specs available at ./specs/"

# Run an example
example: build
	@echo "Running example $(EXAMPLE)"
	@./target/debug/cursed examples/$(EXAMPLE).csd

# Help target
help:
	@echo "CURSED Compiler Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build      - Build the compiler"
	@echo "  test       - Run tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  docs       - Generate documentation"
	@echo "  fmt        - Check code formatting"
	@echo "  fmt-fix    - Fix code formatting"
	@echo "  lint       - Run linter"
	@echo "  release    - Build in release mode"
	@echo "  install    - Install the compiler"
	@echo "  update     - Update dependencies"
	@echo "  run        - Run the compiler (use ARGS= to pass arguments)"
	@echo "  specs      - Show specs documentation location"
	@echo "  example    - Run an example"
	@echo "  help       - Show this help message" 