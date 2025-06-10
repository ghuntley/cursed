.PHONY: build test lint fmt fmt-check fmt-fix fmt-diff clean example jit-test language-benchmark stage2-build stage2-test stage2-status bootstrap-test bootstrap-test-quick bootstrap-test-full bootstrap-test-category bootstrap-test-report bootstrap-test-clean bootstrap-test-help fmt-help cursed-lint cursed-lint-check cursed-lint-fix cursed-lint-stats cursed-lint-help pkg-install pkg-update pkg-check pkg-clean pkg-search pkg-info pkg-init build-with-packages test-with-packages pkg-help docs docs-all docs-markdown docs-json docs-check docs-check-json docs-serve docs-watch docs-clean docs-open docs-config docs-help cursed-build cursed-build-init cursed-build-clean cursed-build-run cursed-build-test cursed-build-templates cursed-build-help debug-build debug-test debug-ir debug-dwarf debug-gdb debug-lldb debug-vscode debug-report debug-validate debug-help crypto-test crypto-test-integration crypto-test-stress crypto-test-security crypto-test-interop crypto-test-all crypto-example crypto-benchmark crypto-help enhanced-gc-test enhanced-gc-test-unit enhanced-gc-test-integration enhanced-gc-test-performance enhanced-gc-test-stress enhanced-gc-test-memory-safety enhanced-gc-test-all enhanced-gc-test-quick enhanced-gc-test-ignored enhanced-gc-test-coverage enhanced-gc-test-report enhanced-gc-help type-system-test type-system-test-integration type-system-test-parser type-system-test-comprehensive type-system-test-all type-system-test-quick type-system-help

build:
	./fix_linking.sh devenv shell cargo build

test:
	./fix_linking.sh devenv shell cargo test

# Run a specific test by name
test-name:
	./fix_linking.sh devenv shell cargo test $(TEST_NAME)

# Run a specific test file
test-file:
	./fix_linking.sh devenv shell cargo test --test $(TEST_FILE)

# Run with warnings silenced
test-quiet:
	./fix_linking.sh devenv shell cargo test --quiet

# Run all tests without warnings
test-no-warn:
	DENY_WARNINGS=0 ./fix_linking.sh devenv shell cargo test

# Build with warnings silenced
build-quiet:
	./fix_linking.sh devenv shell cargo build --quiet

lint:
	./fix_linking.sh devenv shell cargo clippy -- -D warnings

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

# CURSED Build System Integration
# ===============================

# Build the cursed-build tool
cursed-build:
	@echo "Building CURSED build system..."
	devenv shell "cargo build --bin cursed-build"
	@echo "CURSED build system ready: ./target/debug/cursed-build"

# Build using the comprehensive build system
cursed-build-comprehensive: cursed-build
	@echo "Running comprehensive build with pipeline..."
	devenv shell "./target/debug/cursed-build build --verbose"

# Quick build (skip formatting and linting)
cursed-build-quick: cursed-build
	@echo "Running quick build..."
	devenv shell "./target/debug/cursed-build build --quick --verbose"

# Force rebuild (ignore cache)
cursed-build-force: cursed-build
	@echo "Running force rebuild..."
	devenv shell "./target/debug/cursed-build build --force --verbose"

# Parallel build with specific job count
cursed-build-parallel: cursed-build
	@echo "Running parallel build with $(JOBS) jobs..."
	$(eval JOBS ?= 4)
	devenv shell "./target/debug/cursed-build build --jobs $(JOBS) --verbose"

# Release build with all optimizations
cursed-build-release: cursed-build
	@echo "Running release build with full pipeline..."
	devenv shell "./target/debug/cursed-build build --release --verbose"

# Initialize a new CURSED project
cursed-build-init: cursed-build
	@if [ -z "$(PROJECT)" ]; then \
		echo "Error: Please specify PROJECT=<project_name>"; \
		echo "Usage: make cursed-build-init PROJECT=my-project [TEMPLATE=cli|lib|web|api|game]"; \
		exit 1; \
	fi
	$(eval TEMPLATE ?= cli)
	@echo "Initializing CURSED project: $(PROJECT) with template: $(TEMPLATE)"
	devenv shell "./target/debug/cursed-build init $(PROJECT) --template $(TEMPLATE)"

# Build project with CURSED build system
cursed-build-build: cursed-build
	@echo "Building project with CURSED build system..."
	devenv shell "./target/debug/cursed-build build"

# Run project with CURSED build system
cursed-build-run: cursed-build
	@echo "Running project with CURSED build system..."
	devenv shell "./target/debug/cursed-build run"

# Test project with CURSED build system
cursed-build-test: cursed-build
	@echo "Testing project with CURSED build system..."
	devenv shell "./target/debug/cursed-build test"

# Clean project with CURSED build system
cursed-build-clean: cursed-build
	@echo "Cleaning project with CURSED build system..."
	devenv shell "./target/debug/cursed-build clean"

# Show available project templates
cursed-build-templates: cursed-build
	@echo "Available CURSED project templates:"
	devenv shell "./target/debug/cursed-build templates --detailed"

# Format code with build system integration
cursed-build-fmt: cursed-build
	@echo "Formatting code with CURSED build system..."
	devenv shell "./target/debug/cursed-build format"

# Lint code with build system integration
cursed-build-lint: cursed-build
	@echo "Linting code with CURSED build system..."
	devenv shell "./target/debug/cursed-build lint"

# Generate docs with build system integration
cursed-build-docs: cursed-build
	@echo "Generating documentation with CURSED build system..."
	devenv shell "./target/debug/cursed-build docs"

# Show project information
cursed-build-info: cursed-build
	@echo "Project information:"
	devenv shell "./target/debug/cursed-build info --deps --config"

# Package management through build system
cursed-build-pkg-install: cursed-build
	@echo "Installing dependencies through build system..."
	devenv shell "./target/debug/cursed-build package install"

cursed-build-pkg-add: cursed-build
	@if [ -z "$(PACKAGE)" ]; then \
		echo "Error: Please specify PACKAGE=<package_name>"; \
		echo "Usage: make cursed-build-pkg-add PACKAGE=package-name [VERSION=1.0.0]"; \
		exit 1; \
	fi
	$(eval VERSION_ARG := $(if $(VERSION),--version $(VERSION),))
	devenv shell "./target/debug/cursed-build package add $(PACKAGE) $(VERSION_ARG)"

# Build system help
cursed-build-help: cursed-build
	@echo ""
	@echo "CURSED Build System Commands"
	@echo "============================"
	@echo ""
	@echo "Project Management:"
	@echo "  cursed-build-init           - Initialize new project (requires PROJECT=name)"
	@echo "  cursed-build-templates      - Show available templates"
	@echo "  cursed-build-info           - Show project information"
	@echo ""
	@echo "Building and Testing:"
	@echo "  cursed-build-build          - Build the current project"
	@echo "  cursed-build-run            - Build and run the current project"
	@echo "  cursed-build-test           - Run project tests"
	@echo "  cursed-build-clean          - Clean build artifacts"
	@echo ""
	@echo "Code Quality:"
	@echo "  cursed-build-fmt            - Format source code"
	@echo "  cursed-build-lint           - Lint source code"
	@echo "  cursed-build-docs           - Generate documentation"
	@echo ""
	@echo "Package Management:"
	@echo "  cursed-build-pkg-install    - Install dependencies"
	@echo "  cursed-build-pkg-add        - Add dependency (requires PACKAGE=name)"
	@echo ""
	@echo "Examples:"
	@echo "  make cursed-build-init PROJECT=my-web-app TEMPLATE=web"
	@echo "  make cursed-build-pkg-add PACKAGE=cursed-http VERSION=1.0.0"
	@echo "  make cursed-build-build"
	@echo "  make cursed-build-run"
	@echo ""
	@echo "CLI Usage:"
	@echo "  ./target/debug/cursed-build --help         # Show all options"
	@echo "  ./target/debug/cursed-build init --help    # Help for specific command"
	@echo ""
	@echo "Enhanced Build Commands:"
	@echo "  make cursed-build-comprehensive  # Full pipeline build"
	@echo "  make cursed-build-quick          # Quick build (skip fmt/lint)"
	@echo "  make cursed-build-force          # Force rebuild (ignore cache)"
	@echo "  make cursed-build-parallel JOBS=8 # Parallel build with job limit"
	@echo "  make cursed-build-release        # Release build with optimizations"
	@echo ""

# =============================================================================
# Debug Information Generation
# =============================================================================

# Build debug tool
debug-build:
	@echo "Building CURSED debug tool..."
	LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib" RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd" devenv shell cargo build --bin cursed-debug

# Test debug functionality
debug-test: debug-build
	@echo "Testing debug information generation..."
	LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib" RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd" devenv shell cargo test --test debug_integration_test

# Generate LLVM IR with debug information
debug-ir: debug-build
	@echo "Generating LLVM IR with debug information..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-ir FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),debug)
	LIBRARY_PATH="/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib" RUSTFLAGS="-C linker=gcc -C link-arg=-fuse-ld=bfd" devenv shell "cargo run --bin cursed-debug $(FILE) --format llvm-ir --output $(or $(OUTPUT),debug) $(ARGS)"

# Generate DWARF debug information
debug-dwarf: debug-build
	@echo "Generating DWARF debug information..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-dwarf FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),debug)
	devenv shell "./target/debug/cursed-debug $(FILE) --format dwarf --output $(or $(OUTPUT),debug) $(ARGS)"

# Generate GDB debugging script
debug-gdb: debug-build
	@echo "Generating GDB debugging script..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-gdb FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),debug)
	devenv shell "./target/debug/cursed-debug $(FILE) --format gdb-script --output $(or $(OUTPUT),debug) $(ARGS)"

# Generate LLDB debugging script
debug-lldb: debug-build
	@echo "Generating LLDB debugging script..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-lldb FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),debug)
	devenv shell "./target/debug/cursed-debug $(FILE) --format lldb-script --output $(or $(OUTPUT),debug) $(ARGS)"

# Generate VS Code debugging configuration
debug-vscode: debug-build
	@echo "Generating VS Code debugging configuration..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-vscode FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),.vscode)
	devenv shell "./target/debug/cursed-debug $(FILE) --format vscode-config --output $(or $(OUTPUT),.vscode) $(ARGS)"

# Generate comprehensive debug report
debug-report: debug-build
	@echo "Generating comprehensive debug report..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-report FILE=path/to/file.csd [OUTPUT=output/dir]"; \
		exit 1; \
	fi
	@mkdir -p $(or $(OUTPUT),debug)
	devenv shell "./target/debug/cursed-debug $(FILE) --format report --output $(or $(OUTPUT),debug) --validate --stats $(ARGS)"

# Validate debug information
debug-validate: debug-build
	@echo "Validating debug information..."
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make debug-validate FILE=path/to/file.csd"; \
		exit 1; \
	fi
	devenv shell "./target/debug/cursed-debug $(FILE) --validate --stats --verbose $(ARGS)"

# Debug help
debug-help:
	@echo "CURSED Debug Information Generation"
	@echo "=================================="
	@echo ""
	@echo "Available debug targets:"
	@echo "  debug-build       Build the debug tool"
	@echo "  debug-test        Test debug functionality"
	@echo "  debug-ir          Generate LLVM IR with debug info"
	@echo "  debug-dwarf       Generate DWARF debug information"
	@echo "  debug-gdb         Generate GDB debugging script"
	@echo "  debug-lldb        Generate LLDB debugging script"
	@echo "  debug-vscode      Generate VS Code debug configuration"
	@echo "  debug-report      Generate comprehensive debug report"
	@echo "  debug-validate    Validate debug information"
	@echo ""
	@echo "Usage examples:"
	@echo "  make debug-ir FILE=examples/hello.csd"
	@echo "  make debug-gdb FILE=examples/fibonacci.csd OUTPUT=debug/"
	@echo "  make debug-report FILE=myprogram.csd"
	@echo "  make debug-validate FILE=myprogram.csd"
	@echo ""
	@echo "Additional arguments can be passed via ARGS variable:"
	@echo "  make debug-ir FILE=test.csd ARGS='--debug-level 3 --include-source'"
	@echo "  make debug-dwarf FILE=test.csd ARGS='--dwarf-version 5 --compress'"
	@echo "  make debug-report FILE=test.csd ARGS='--verbose'"
	@echo ""
	@echo "Debug levels:"
	@echo "  0 - No debug information"
	@echo "  1 - Line tables only (minimal)"
	@echo "  2 - Full debug information (default)"
	@echo "  3 - Enhanced debug with additional metadata"
	@echo ""
	@echo "For complete usage information:"
	@echo "  ./target/debug/cursed-debug --help"
	@echo ""

# =============================================================================
# CRYPTO TESTING COMMANDS
# =============================================================================

# Run crypto integration tests
crypto-test-integration:
	@echo "🔐 Running crypto integration tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_integration_test

# Run crypto stress tests
crypto-test-stress:
	@echo "🚀 Running crypto stress tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_stress_test

# Run crypto security validation tests
crypto-test-security:
	@echo "🛡️ Running crypto security tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_security_test

# Run crypto interoperability tests
crypto-test-interop:
	@echo "🔗 Running crypto interoperability tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_interop_test

# Run all crypto tests
crypto-test-all:
	@echo "💎 Running comprehensive crypto test suite..."
	@make crypto-test-integration
	@make crypto-test-stress
	@make crypto-test-security
	@make crypto-test-interop
	@echo "✅ All crypto tests completed!"

# Run specific crypto test suites
crypto-test:
	@echo "🔒 Running standard crypto tests..."
	./fix_linking.sh devenv shell cargo test crypto

# Run crypto examples
crypto-example:
	@echo "🎭 Running crypto examples..."
	@echo "Building crypto showcase..."
	./target/debug/cursed examples/crypto_showcase.csd
	@echo "Building secure messaging demo..."
	./target/debug/cursed examples/secure_messaging.csd
	@echo "Building file encryption utility..."
	./target/debug/cursed examples/file_encryption.csd demo
	@echo "Building digital signatures demo..."
	./target/debug/cursed examples/digital_signatures.csd
	@echo "Building web security demo..."
	./target/debug/cursed examples/web_security.csd

# Run crypto performance benchmarks
crypto-benchmark:
	@echo "⚡ Running crypto performance benchmarks..."
	./fix_linking.sh devenv shell cargo test --test crypto_stress_test test_performance_benchmarks --release
	./fix_linking.sh devenv shell cargo test --test crypto_integration_test test_performance_benchmarks --release

# Build crypto examples
crypto-build-examples:
	@echo "🔧 Building crypto examples..."
	@make build
	@echo "✅ Examples ready to run!"

# Quick crypto test (essential tests only)
crypto-test-quick:
	@echo "⚡ Running quick crypto tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_integration_test test_end_to_end_encryption_workflow
	./fix_linking.sh devenv shell cargo test --test crypto_security_test test_randomness_quality
	@echo "✅ Quick crypto tests completed!"

# Crypto test with coverage
crypto-test-coverage:
	@echo "📊 Running crypto tests with coverage..."
	./fix_linking.sh devenv shell cargo tarpaulin --tests crypto_integration_test crypto_stress_test crypto_security_test crypto_interop_test --out html --output-dir target/coverage/crypto

# Validate crypto implementations
crypto-validate:
	@echo "✅ Validating crypto implementations..."
	./fix_linking.sh devenv shell cargo test --test crypto_interop_test test_standard_test_vectors
	./fix_linking.sh devenv shell cargo test --test crypto_security_test test_constant_time_operations
	@echo "✅ Crypto validation completed!"

# Clean crypto test artifacts
crypto-clean:
	@echo "🧹 Cleaning crypto test artifacts..."
	rm -rf target/coverage/crypto
	rm -f examples/*.encrypted
	rm -f *.encrypted
	rm -f demo_secret.txt
	@echo "✅ Crypto artifacts cleaned!"

# Crypto help
crypto-help:
	@echo "🔐 CURSED Crypto Testing Commands"
	@echo "================================="
	@echo ""
	@echo "Test Commands:"
	@echo "  crypto-test               - Run standard crypto tests"
	@echo "  crypto-test-integration   - Run comprehensive integration tests"
	@echo "  crypto-test-stress        - Run performance and stress tests"
	@echo "  crypto-test-security      - Run security validation tests"
	@echo "  crypto-test-interop       - Run interoperability tests"
	@echo "  crypto-test-all           - Run complete crypto test suite"
	@echo "  crypto-test-quick         - Run essential tests only"
	@echo "  crypto-test-coverage      - Run tests with coverage analysis"
	@echo ""
	@echo "Example Commands:"
	@echo "  crypto-example            - Run all crypto examples"
	@echo "  crypto-build-examples     - Build crypto examples"
	@echo ""
	@echo "Performance Commands:"
	@echo "  crypto-benchmark          - Run crypto performance benchmarks"
	@echo ""
	@echo "Validation Commands:"
	@echo "  crypto-validate           - Validate crypto implementations"
	@echo ""
	@echo "Utility Commands:"
	@echo "  crypto-clean              - Clean crypto test artifacts"
	@echo "  crypto-help               - Show this help message"
	@echo ""
	@echo "Example Usage:"
	@echo "  make crypto-test-all                    # Run complete test suite"
	@echo "  make crypto-test-quick                  # Quick validation"
	@echo "  make crypto-example                     # Try examples"
	@echo "  make crypto-benchmark                   # Performance testing"
	@echo ""
	@echo "Test Suites Include:"
	@echo "  • Symmetric encryption (AES-GCM, ChaCha20-Poly1305)"
	@echo "  • Asymmetric cryptography (RSA, ECC, Ed25519)"
	@echo "  • Digital signatures and verification"
	@echo "  • Cryptographic hashing (SHA-2, SHA-3, BLAKE3)"
	@echo "  • Key derivation functions (PBKDF2, Argon2, scrypt)"
	@echo "  • Secure random number generation"
	@echo "  • Zero-knowledge proofs"
	@echo "  • Post-quantum cryptography assessment"
	@echo "  • PKI and certificate handling"
	@echo "  • Cryptographic protocols"
	@echo "  • Security validation and attack resistance"
	@echo "  • Cross-platform compatibility"
	@echo "  • Standard compliance verification"
	@echo ""
	@echo "For more details, see the AGENT.md file!"

# Enhanced GC Test Suite Targets
# These targets provide comprehensive testing for the enhanced garbage collection implementation

# Run all enhanced GC tests (default)
enhanced-gc-test:
	@echo "Running comprehensive enhanced GC test suite..."
	tests/run_enhanced_gc_tests.sh

# Run unit tests for enhanced GC features
enhanced-gc-test-unit:
	@echo "Running enhanced GC unit tests..."
	tests/run_enhanced_gc_tests.sh --test unit

# Run integration tests for enhanced GC workflows
enhanced-gc-test-integration:
	@echo "Running enhanced GC integration tests..."
	tests/run_enhanced_gc_tests.sh --test integration

# Run performance tests for enhanced GC
enhanced-gc-test-performance:
	@echo "Running enhanced GC performance tests..."
	tests/run_enhanced_gc_tests.sh --test performance --ignored

# Run stress tests for enhanced GC
enhanced-gc-test-stress:
	@echo "Running enhanced GC stress tests..."
	tests/run_enhanced_gc_tests.sh --test stress --ignored

# Run memory safety tests for enhanced GC
enhanced-gc-test-memory-safety:
	@echo "Running enhanced GC memory safety tests..."
	tests/run_enhanced_gc_tests.sh --test memory-safety

# Run all enhanced GC tests including long-running ones
enhanced-gc-test-all:
	@echo "Running all enhanced GC tests (including stress and performance)..."
	tests/run_enhanced_gc_tests.sh --ignored

# Run quick enhanced GC tests (excluding long-running tests)
enhanced-gc-test-quick:
	@echo "Running quick enhanced GC tests..."
	tests/run_enhanced_gc_tests.sh --quick

# Run ignored enhanced GC tests (stress and performance)
enhanced-gc-test-ignored:
	@echo "Running ignored enhanced GC tests (stress and performance)..."
	tests/run_enhanced_gc_tests.sh --ignored

# Generate enhanced GC test coverage report
enhanced-gc-test-coverage:
	@echo "Generating enhanced GC test coverage report..."
	tests/run_enhanced_gc_tests.sh --coverage

# Generate enhanced GC test report
enhanced-gc-test-report:
	@echo "Generating enhanced GC test report..."
	tests/run_enhanced_gc_tests.sh --report enhanced_gc_test_report.md

# LLVM GC Integration Test Suite Targets

# Run LLVM GC integration tests
llvm-gc-test:
	@echo "Running LLVM GC integration tests..."
	./fix_linking.sh cargo test --test llvm_gc_integration_test

# Run LLVM GC integration tests with verbose output
llvm-gc-test-verbose:
	@echo "Running LLVM GC integration tests (verbose)..."
	./fix_linking.sh cargo test --test llvm_gc_integration_test -- --nocapture

# Run specific LLVM GC integration test
llvm-gc-test-single:
	@echo "Running specific LLVM GC integration test: $(TEST_NAME)"
	./fix_linking.sh cargo test --test llvm_gc_integration_test $(TEST_NAME)

# Run all GC-related tests (enhanced + LLVM integration)
gc-test-all:
	@echo "Running all GC-related tests..."
	@$(MAKE) enhanced-gc-test-quick
	@$(MAKE) llvm-gc-test
	@echo "All GC tests completed successfully!"

# Enhanced GC test help
enhanced-gc-help:
	@echo "Enhanced GC Test Suite Commands:"
	@echo ""
	@echo "Basic Testing:"
	@echo "  enhanced-gc-test               - Run all standard enhanced GC tests"
	@echo "  enhanced-gc-test-quick         - Run quick tests only (exclude long-running)"
	@echo "  enhanced-gc-test-all           - Run all tests including stress and performance"
	@echo ""
	@echo "Test Categories:"
	@echo "  enhanced-gc-test-unit          - Unit tests for heap management features"
	@echo "  enhanced-gc-test-integration   - Integration tests for generational collection"
	@echo "  enhanced-gc-test-performance   - Performance tests for incremental collection"
	@echo "  enhanced-gc-test-stress        - Stress tests for memory pressure scenarios"
	@echo "  enhanced-gc-test-memory-safety - Memory safety guarantee validation"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  enhanced-gc-test-coverage      - Generate code coverage report"
	@echo "  enhanced-gc-test-report        - Generate detailed test report"
	@echo "  enhanced-gc-test-ignored       - Run long-running tests only"
	@echo ""
	@echo "LLVM GC Integration Test Commands:"
	@echo "  llvm-gc-test                   - Run LLVM GC integration tests"
	@echo "  llvm-gc-test-verbose           - Run LLVM GC integration tests (verbose)"
	@echo "  llvm-gc-test-single TEST_NAME  - Run specific LLVM GC integration test"
	@echo "  gc-test-all                    - Run all GC-related tests"
	@echo ""
	@echo "Test Features:"
	@echo "  • Comprehensive unit testing for new heap management features"
	@echo "  • End-to-end integration testing for generational collection"
	@echo "  • Performance validation for incremental collection algorithms"
	@echo "  • Stress testing under extreme memory pressure conditions"
	@echo "  • Memory safety guarantee validation with concurrent scenarios"
	@echo "  • Thread safety testing for concurrent GC operations"
	@echo "  • Memory corruption detection and prevention testing"
	@echo "  • Adaptive algorithm selection validation"
	@echo "  • Cross-generational reference safety testing"
	@echo "  • Complex object graph handling validation"
	@echo ""

# ===================================================================
# String Manipulation Utilities
# ===================================================================

# String utilities help
string-help:
	@echo ""
	@echo "🔤 CURSED String Manipulation Utilities"
	@echo "======================================"
	@echo ""
	@echo "Test Commands:"
	@echo "  string-test                - Run comprehensive string manipulation tests"
	@echo "  string-test-verbose        - Run tests with verbose output"
	@echo ""
	@echo "Documentation:"
	@echo "  string-doc                 - Generate string utilities documentation"
	@echo "  string-examples            - Show string manipulation examples"
	@echo ""
	@echo "Features:"
	@echo "  • Core operations (length, concat, reverse, repeat)"
	@echo "  • Search and replace (contains, find, replace variants)"
	@echo "  • Transformations (case conversion, trimming, substrings)"
	@echo "  • Splitting and joining (split by delimiter, whitespace, etc.)"
	@echo "  • Validation (numeric, email, URL, phone, palindrome)"
	@echo "  • Formatting (padding, centering, wrapping, escaping)"
	@echo "  • Full Unicode support with proper character handling"
	@echo "  • Error handling with detailed error messages"
	@echo ""

# Run string manipulation tests
string-test:
	@echo "🔤 Running string manipulation tests..."
	$(LINK_FIX) cargo test --test string_manipulation_test

# Run string tests with verbose output
string-test-verbose:
	@echo "🔤 Running string manipulation tests (verbose)..."
	$(LINK_FIX) cargo test --test string_manipulation_test -- --nocapture

# Generate string utilities documentation
string-doc:
	@echo "📚 Generating string utilities documentation..."
	$(LINK_FIX) cargo doc --no-deps --document-private-items --package cursed

# Show string manipulation examples
string-examples:
	@echo "📝 String Manipulation Examples:"
	@echo ""
	@echo "Core Operations:"
	@echo "  length(\"hello\")           -> 5"
	@echo "  reverse(\"hello\")         -> \"olleh\""
	@echo "  repeat(\"abc\", 3)         -> \"abcabcabc\""
	@echo ""
	@echo "Search & Replace:"
	@echo "  contains(\"hello\", \"ell\") -> true"
	@echo "  find(\"hello\", \"ell\")     -> Some(1)"
	@echo "  replace(\"hello\", \"l\", \"x\") -> \"hexxo\""
	@echo ""
	@echo "Transformations:"
	@echo "  to_uppercase(\"hello\")    -> \"HELLO\""
	@echo "  trim(\"  hello  \")        -> \"hello\""
	@echo "  to_camel_case(\"hello world\") -> \"helloWorld\""
	@echo ""
	@echo "Validation:"
	@echo "  is_numeric(\"123\")        -> true"
	@echo "  is_email(\"user@example.com\") -> true"
	@echo "  is_palindrome(\"racecar\") -> true"
	@echo ""
	@echo "For the complete demo, see: examples/string_manipulation_demo.csd"
	@echo ""

# Type System Integration Testing
# ================================

# Run basic type system integration tests
type-system-test:
	@echo "Running type system integration tests..."
	./fix_linking.sh cargo test --test type_system_llvm_integration_test

# Run type system LLVM integration tests
type-system-test-integration:
	@echo "Running type system LLVM integration tests..."
	./fix_linking.sh cargo test --test type_system_llvm_integration_test

# Run parser constraint integration tests
type-system-test-parser:
	@echo "Running parser constraint integration tests..."
	./fix_linking.sh cargo test --test parser_constraint_integration_test

# Run comprehensive generic integration tests
type-system-test-comprehensive:
	@echo "Running comprehensive generic integration tests..."
	./fix_linking.sh cargo test --test comprehensive_generic_integration_test

# Run all type system integration tests
type-system-test-all:
	@echo "Running all type system integration tests..."
	./fix_linking.sh cargo test --test type_system_llvm_integration_test
	./fix_linking.sh cargo test --test parser_constraint_integration_test
	./fix_linking.sh cargo test --test comprehensive_generic_integration_test

# Run quick type system validation
type-system-test-quick:
	@echo "Running quick type system validation..."
	./fix_linking.sh cargo test test_generic_struct_compilation
	./fix_linking.sh cargo test test_generic_instantiation
	./fix_linking.sh cargo test test_constraint_validation

# Type system test help
type-system-help:
	@echo "Type System Integration Test Targets:"
	@echo "  type-system-test            - Run basic type system integration tests"
	@echo "  type-system-test-integration - Run type system LLVM integration tests"
	@echo "  type-system-test-parser     - Run parser constraint integration tests"
	@echo "  type-system-test-comprehensive - Run comprehensive generic integration tests"
	@echo "  type-system-test-all        - Run all type system integration tests"
	@echo "  type-system-test-quick      - Run quick type system validation"
	@echo "  type-system-help            - Show this help message"
	@echo "All tests automatically use the linking fix infrastructure for Nix compatibility."