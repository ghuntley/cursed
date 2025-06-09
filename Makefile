.PHONY: build test lint fmt fmt-check fmt-fix fmt-diff clean example jit-test language-benchmark stage2-build stage2-test stage2-status bootstrap-test bootstrap-test-quick bootstrap-test-full bootstrap-test-category bootstrap-test-report bootstrap-test-clean bootstrap-test-help fmt-help cursed-lint cursed-lint-check cursed-lint-fix cursed-lint-stats cursed-lint-help pkg-install pkg-update pkg-check pkg-clean pkg-search pkg-info pkg-init build-with-packages test-with-packages pkg-help docs docs-all docs-markdown docs-json docs-check docs-check-json docs-serve docs-watch docs-clean docs-open docs-config docs-help

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

# CURSED Linter Commands
cursed-lint:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' -print0 | xargs -0 ./target/debug/cursed_lint_new"

# Lint with strict checking (exit on any issues)
cursed-lint-check:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' -print0 | xargs -0 ./target/debug/cursed_lint_new --check --fail-on warning"

# Lint with auto-fix
cursed-lint-fix:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' -print0 | xargs -0 ./target/debug/cursed_lint_new --fix"

# Lint with detailed statistics
cursed-lint-stats:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "find . -name '*.csd' -not -path './target/*' -not -path './.git/*' -print0 | xargs -0 ./target/debug/cursed_lint_new --stats --verbose"

# Lint specific directory recursively
cursed-lint-dir:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "./target/debug/cursed_lint_new --recursive $(DIR)"

# Generate linter configuration
cursed-lint-init:
	devenv shell "cargo build --bin cursed_lint_new"
	devenv shell "./target/debug/cursed_lint_new --generate-config .cursed-lint.toml"

# Show linter help
cursed-lint-help:
	@echo "CURSED Linter Commands:"
	@echo "  make cursed-lint        - Lint all .csd files with default settings"
	@echo "  make cursed-lint-check  - Lint with strict checking (CI mode)"
	@echo "  make cursed-lint-fix    - Lint with auto-fix enabled"
	@echo "  make cursed-lint-stats  - Lint with detailed statistics"
	@echo "  make cursed-lint-init   - Generate default configuration file"
	@echo "  make cursed-lint-dir DIR=path - Lint specific directory"
	@echo ""
	@echo "Configuration:"
	@echo "  Create .cursed-lint.toml for project-specific settings"
	@echo "  Use --disable rule1,rule2 to disable specific rules"
	@echo "  Use --severity warning to set minimum severity level"
	@echo ""
	@echo "Output Formats:"
	@echo "  --format human      - Human-readable output (default)"
	@echo "  --format json       - JSON output for tools"
	@echo "  --format checkstyle - Checkstyle XML for CI"
	@echo "  --format sarif      - SARIF format for security tools"
	@echo ""
	@echo "Examples:"
	@echo "  make cursed-lint-check                    # CI linting"
	@echo "  make cursed-lint-fix                      # Auto-fix issues"
	@echo "  make cursed-lint-dir DIR=examples         # Lint examples/"

# Package Management Targets
# ===========================

# Install packages for current project
pkg-install:
	@echo "Installing package dependencies..."
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg install"

# Update all package dependencies
pkg-update:
	@echo "Updating package dependencies..."
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg update"

# Check for dependency updates and vulnerabilities
pkg-check:
	@echo "Checking package dependencies..."
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg check"

# Clean package cache
pkg-clean:
	@echo "Cleaning package cache..."
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg clean"

# Search for packages in registry
pkg-search:
	@if [ -z "$(PACKAGE)" ]; then \
		echo "Error: Please specify PACKAGE=<package_name>"; \
		exit 1; \
	fi
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg search $(PACKAGE)"

# Show package information
pkg-info:
	@if [ -z "$(PACKAGE)" ]; then \
		echo "Error: Please specify PACKAGE=<package_name>"; \
		exit 1; \
	fi
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg info $(PACKAGE)"

# Initialize package manifest for current project
pkg-init:
	@echo "Initializing package manifest..."
	devenv shell "cargo build --bin cursed-pkg"
	devenv shell "./target/debug/cursed-pkg init"

# Build project with package dependencies
build-with-packages: pkg-install build

# Test project with package dependencies
test-with-packages: pkg-install test

# Package management help
pkg-help:
	@echo "Package Management Commands:"
	@echo "  pkg-install          - Install dependencies from cursed.toml"
	@echo "  pkg-update           - Update all dependencies to latest versions"
	@echo "  pkg-check            - Check dependencies for updates and vulnerabilities"
	@echo "  pkg-clean            - Clean package cache"
	@echo "  pkg-search           - Search for packages (requires PACKAGE=name)"
	@echo "  pkg-info             - Show package information (requires PACKAGE=name)"
	@echo "  pkg-init             - Initialize cursed.toml manifest"
	@echo "  build-with-packages  - Install dependencies and build"
	@echo "  test-with-packages   - Install dependencies and test"
	@echo ""
	@echo "Examples:"
	@echo "  make pkg-search PACKAGE=http"
	@echo "  make pkg-info PACKAGE=serde"
	@echo "  make build-with-packages"

# Documentation Generation Targets
# =================================

# Generate comprehensive HTML documentation
docs:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Generating documentation..."
	devenv shell "./target/debug/cursed-doc --html --source src --source examples --output docs/html --package-name 'CURSED Language' --package-version '$(shell git describe --tags --always)' --description 'A programming language that speaks Gen Z' --clean --stats"
	@echo "Documentation generated successfully!"
	@echo "📖 View at: file://$(PWD)/docs/html/index.html"

# Generate all documentation formats
docs-all:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Generating HTML documentation..."
	devenv shell "./target/debug/cursed-doc --html --source src --source examples --output docs/html --package-name 'CURSED Language' --clean --stats"
	@echo "Generating Markdown documentation..."
	devenv shell "./target/debug/cursed-doc --markdown --source src --source examples --output docs/markdown --package-name 'CURSED Language' --clean --stats"
	@echo "Generating JSON documentation..."
	devenv shell "./target/debug/cursed-doc --json --source src --source examples --output docs/json --package-name 'CURSED Language' --clean --stats"
	@echo "All documentation formats generated successfully!"
	@echo "📖 HTML: file://$(PWD)/docs/html/index.html"
	@echo "📄 Markdown: docs/markdown/"
	@echo "📊 JSON: docs/json/"

# Generate Markdown documentation
docs-markdown:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Generating Markdown documentation..."
	devenv shell "./target/debug/cursed-doc --markdown --source src --source examples --output docs/markdown --package-name 'CURSED Language' --clean --stats"
	@echo "Markdown documentation generated successfully!"
	@echo "📖 View at: docs/markdown/"

# Generate JSON documentation data
docs-json:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Generating JSON documentation..."
	devenv shell "./target/debug/cursed-doc --json --source src --source examples --output docs/json --package-name 'CURSED Language' --clean --stats"
	@echo "JSON documentation generated successfully!"
	@echo "📊 View at: docs/json/"

# Check documentation completeness and validity
docs-check:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Checking documentation completeness..."
	devenv shell "./target/debug/cursed-doc --check --source src --source examples --package-name 'CURSED Language'"
	@echo "Documentation check complete."

# Check documentation completeness with JSON output
docs-check-json:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Checking documentation completeness (JSON output)..."
	devenv shell "./target/debug/cursed-doc --check --source src --source examples --package-name 'CURSED Language' --output-format json" | jq '.'

# Serve documentation locally with auto-reload
docs-serve:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Starting documentation server..."
	devenv shell "./target/debug/cursed-doc --source src --source examples --output docs/html --serve --watch --host 127.0.0.1 --port 8080 --clean"

# Generate documentation and serve with auto-reload (development mode)
docs-watch:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Starting documentation server in watch mode..."
	devenv shell "./target/debug/cursed-doc --source src --source examples --output docs/html --serve --watch --open --clean"

# Clean generated documentation
docs-clean:
	@echo "Cleaning documentation..."
	rm -rf docs/html docs/json docs/markdown
	@echo "Documentation cleaned."

# Open documentation in browser
docs-open:
	@if [ -f "docs/html/index.html" ]; then \
		echo "Opening documentation in browser..."; \
		if command -v xdg-open > /dev/null; then \
			xdg-open "file://$(PWD)/docs/html/index.html"; \
		elif command -v open > /dev/null; then \
			open "file://$(PWD)/docs/html/index.html"; \
		elif command -v start > /dev/null; then \
			start "file://$(PWD)/docs/html/index.html"; \
		else \
			echo "Please open: file://$(PWD)/docs/html/index.html"; \
		fi; \
	else \
		echo "Documentation not found. Run 'make docs' first."; \
	fi

# Generate default configuration file
docs-config:
	@echo "Building documentation generator..."
	devenv shell "cargo build --bin cursed-doc"
	@echo "Generating default configuration..."
	devenv shell "./target/debug/cursed-doc --generate-config .cursed-doc.toml"
	@echo "Configuration file generated: .cursed-doc.toml"
	@echo "Edit this file to customize documentation generation."

# Show documentation help
docs-help:
	@echo ""
	@echo "CURSED Documentation Generation"
	@echo "==============================="
	@echo ""
	@echo "Available documentation targets:"
	@echo "  docs            - Generate HTML documentation (default)"
	@echo "  docs-all        - Generate all formats (HTML, Markdown, JSON)"
	@echo "  docs-markdown   - Generate Markdown documentation only"
	@echo "  docs-json       - Generate JSON documentation data only"
	@echo "  docs-check      - Validate documentation completeness"
	@echo "  docs-check-json - Validate with JSON output"
	@echo "  docs-serve      - Start local server with live reload"
	@echo "  docs-watch      - Generate and serve with auto-reload"
	@echo "  docs-clean      - Clean generated documentation"
	@echo "  docs-open       - Open documentation in browser"
	@echo "  docs-config     - Generate default configuration file"
	@echo "  docs-help       - Show this help"
	@echo ""
	@echo "Example usage:"
	@echo "  make docs                    # Generate HTML docs"
	@echo "  make docs-all               # Generate all formats"
	@echo "  make docs-check             # Validate completeness"
	@echo "  make docs-serve             # Start development server"
	@echo ""
	@echo "CLI usage examples:"
	@echo "  ./target/debug/cursed-doc --html --source src --output docs/html"
	@echo "  ./target/debug/cursed-doc --check --source src"
	@echo "  ./target/debug/cursed-doc --serve --watch --port 8080"
	@echo ""
	@echo "Configuration:"
	@echo "  - Create .cursed-doc.toml for custom settings"
	@echo "  - Use environment variables: CURSED_DOC_*"
	@echo "  - CLI arguments override configuration files"
	@echo ""
	@echo "For advanced options, run:"
	@echo "  ./target/debug/cursed-doc --help"
	@echo ""