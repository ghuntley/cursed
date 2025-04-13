.PHONY: build test lint fmt fmt-fix clean example jit-test

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

test_preprocessor:
	devenv shell "cargo build --bin test_preprocessor"
	devenv shell "./target/debug/test_preprocessor"