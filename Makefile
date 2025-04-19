.PHONY: build test lint fmt fmt-fix clean example jit-test language-benchmark

build:
	devenv shell cargo build

test:
	devenv shell cargo test

# Run a specific test by name
test-name:
	devenv shell cargo test $(TEST_NAME)

# Run a specific test file
test-file:
	devenv shell cargo test --test $(TEST_FILE)

# Run with warnings silenced
test-quiet:
	devenv shell cargo test --quiet

# Run all tests without warnings
test-no-warn:
	DENY_WARNINGS=0 devenv shell cargo test

# Build with warnings silenced
build-quiet:
	devenv shell cargo build --quiet

lint:
	devenv shell cargo clippy -- -D warnings

# Run clippy with warnings suppressed
lint-allow:
	devenv shell cargo clippy -- -A warnings

fmt:
	devenv shell cargo fmt -- --check

fmt-fix:
	devenv shell cargo fmt

clean:
	devenv shell cargo clean

example:
	devenv shell ./target/debug/cursed examples/$(EXAMPLE).csd

jit-test:
	devenv shell cargo test jit_integration_full

# Test the fixed range clause implementation
range-test:
	devenv shell cargo test range_clause_fixed_test

test_preprocessor:
	devenv shell "cargo build --bin test_preprocessor"
	devenv shell "./target/debug/test_preprocessor"

# Run language comparison benchmarks for all available languages
# Usage: make language-benchmark [FORMAT=console|json|csv|markdown] [OUTPUT=filename]
# Examples:
#   make language-benchmark                         # Runs benchmarks with console output
#   make language-benchmark FORMAT=markdown         # Outputs results in markdown format
#   make language-benchmark OUTPUT=bench_results    # Saves results to bench_results file
language-benchmark:
	$(eval FORMAT ?= console)
	$(eval OUTPUT ?= benchmark_results)
	@echo "Building CURSED in release mode for accurate benchmarking..."
	cargo build --release
	@echo "Running language benchmarks for all available languages..."
	./target/release/language_benchmark $(FORMAT) $(OUTPUT)