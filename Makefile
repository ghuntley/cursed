.PHONY: build test lint fmt fmt-check fmt-fix fmt-diff clean example jit-test language-benchmark stage2-build stage2-test stage2-status bootstrap-test bootstrap-test-quick bootstrap-test-full bootstrap-test-category bootstrap-test-report bootstrap-test-clean bootstrap-test-help fmt-help

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

# Format all CURSED source files
fmt:
	devenv shell "cargo build --bin cursed-fmt"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | xargs ./target/debug/cursed-fmt -w"

# Check if CURSED files are properly formatted (for CI)
fmt-check:
	devenv shell "cargo build --bin cursed-fmt"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | xargs ./target/debug/cursed-fmt --check"

# Show formatting differences without applying changes
fmt-diff:
	devenv shell "cargo build --bin cursed-fmt"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | xargs ./target/debug/cursed-fmt --diff"

# Format Rust source files (existing functionality)
fmt-fix:
	devenv shell cargo fmt

# Check Rust source files formatting (existing functionality)
rust-fmt-check:
	devenv shell cargo fmt -- --check

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

# Stage 2 (Self-Hosting) Compiler Targets
stage2-build:
	@echo "Building Stage 2 CURSED compiler..."
	CURSED_BUILD_STAGE2=1 devenv shell cargo build
	@echo "Stage 2 compiler built successfully"

stage2-test:
	@echo "Testing Stage 2 compiler..."
	devenv shell ./target/debug/cursed stage2 test
	@echo "Testing compilation of example program..."
	devenv shell ./target/debug/cursed stage2 compile examples/stage2_test.csd -o stage2_test_output
	@echo "Stage 2 test completed successfully"

stage2-status:
	@echo "Checking Stage 2 compiler status..."
	devenv shell ./target/debug/cursed stage2 status

stage2-enable:
	@echo "Enabling self-hosting mode..."
	devenv shell ./target/debug/cursed stage2 self-host on
	@echo "Self-hosting mode enabled"

stage2-disable:
	@echo "Disabling self-hosting mode..."
	devenv shell ./target/debug/cursed stage2 self-host off
	@echo "Self-hosting mode disabled"

stage2-compile:
	@echo "Compiling $(FILE) with Stage 2 compiler..."
	devenv shell ./target/debug/cursed stage2 compile $(FILE) -o $(OUTPUT)
	@echo "Stage 2 compilation completed"

# Bootstrap Test Targets

# Run full bootstrap test suite
bootstrap-test: build
	@echo "Running comprehensive bootstrap test suite..."
	./scripts/run_bootstrap_tests.sh --verbose

# Run quick bootstrap tests (subset for faster feedback)
bootstrap-test-quick: build
	@echo "Running quick bootstrap test suite..."
	./scripts/run_bootstrap_tests.sh --quick --verbose

# Run full bootstrap test suite (alias for clarity)
bootstrap-test-full: build
	@echo "Running full bootstrap test suite..."
	./scripts/run_bootstrap_tests.sh --verbose

# Run specific bootstrap test category
bootstrap-test-category: build
	@if [ -z "$(CATEGORY)" ]; then \
		echo "Error: Please specify CATEGORY=<category_name>"; \
		echo "Available categories: minimal_subset, stage2_compiler, self_compilation, performance, regression, ci_integration, memory_usage"; \
		exit 1; \
	fi
	@echo "Running bootstrap tests for category: $(CATEGORY)"
	./scripts/run_bootstrap_tests.sh --category $(CATEGORY) --verbose

# Generate bootstrap test report from existing results
bootstrap-test-report:
	@echo "Generating bootstrap test report..."
	./scripts/run_bootstrap_tests.sh --report-only

# Clean bootstrap test outputs
bootstrap-test-clean:
	@echo "Cleaning bootstrap test outputs..."
	rm -rf test_results/bootstrap/
	@echo "Bootstrap test outputs cleaned"

# Bootstrap test help
bootstrap-test-help:
	@echo "Bootstrap Test Targets:"
	@echo "  bootstrap-test           - Run comprehensive bootstrap test suite"
	@echo "  bootstrap-test-quick     - Run quick bootstrap tests"
	@echo "  bootstrap-test-full      - Run full bootstrap test suite"
	@echo "  bootstrap-test-category  - Run specific test category (requires CATEGORY=<name>)"
	@echo "  bootstrap-test-report    - Generate test report from existing results"
	@echo "  bootstrap-test-clean     - Clean test outputs"
	@echo ""
	@echo "Examples:"
	@echo "  make bootstrap-test-quick"
	@echo "  make bootstrap-test-category CATEGORY=minimal_subset"
	@echo "  make bootstrap-test-category CATEGORY=performance"

# Formatting help
fmt-help:
	@echo "Formatting Targets:"
	@echo "  fmt                      - Format all CURSED (.csd) files"
	@echo "  fmt-check                - Check CURSED file formatting (for CI)"
	@echo "  fmt-diff                 - Show formatting differences without applying"
	@echo "  fmt-fix                  - Format Rust (.rs) files"
	@echo "  rust-fmt-check           - Check Rust file formatting"
	@echo ""
	@echo "Development Workflow:"
	@echo "  ./scripts/install-git-hooks.sh  - Install pre-commit formatting hooks"
	@echo "  make fmt-diff            - Preview formatting changes"
	@echo "  make fmt                 - Apply CURSED formatting"
	@echo "  make fmt-fix             - Apply Rust formatting"
	@echo ""
	@echo "Configuration:"
	@echo "  Edit .cursed_fmt.toml to customize formatting rules"