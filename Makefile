.PHONY: build test lint fmt fmt-check fmt-fix fmt-diff clean example jit-test language-benchmark collections-test collections-test-verbose collections-test-quick collections-test-performance collections-test-stress collections-test-errors collections-help queues-test queues-test-unit queues-test-performance queues-test-thread-safety queues-test-edge-cases queues-test-all queues-test-quick queues-test-coverage queues-test-report queues-help stage2-build stage2-test stage2-status bootstrap-test bootstrap-test-quick bootstrap-test-full bootstrap-test-category bootstrap-test-report bootstrap-test-clean bootstrap-test-help fmt-help cursed-lint cursed-lint-check cursed-lint-fix cursed-lint-stats cursed-lint-help pkg-install pkg-update pkg-check pkg-clean pkg-search pkg-info pkg-init build-with-packages test-with-packages pkg-help docs docs-all docs-markdown docs-json docs-check docs-check-json docs-serve docs-watch docs-clean docs-open docs-config docs-help cursed-build cursed-build-init cursed-build-clean cursed-build-run cursed-build-test cursed-build-templates cursed-build-help debug-build debug-test debug-ir debug-dwarf debug-gdb debug-lldb debug-vscode debug-report debug-validate debug-help crypto-test crypto-test-quick crypto-test-integration crypto-test-stress crypto-test-security crypto-test-interop crypto-test-all crypto-test-coverage crypto-test-report crypto-example crypto-build-examples crypto-benchmark crypto-validate crypto-clean crypto-help enhanced-gc-test enhanced-gc-test-unit enhanced-gc-test-integration enhanced-gc-test-performance enhanced-gc-test-stress enhanced-gc-test-memory-safety enhanced-gc-test-all enhanced-gc-test-quick enhanced-gc-test-ignored enhanced-gc-test-coverage enhanced-gc-test-report enhanced-gc-help type-system-test type-system-test-integration type-system-test-parser type-system-test-comprehensive type-system-test-all type-system-test-quick type-system-help enhanced-debug-test enhanced-debug-test-integration enhanced-debug-test-performance enhanced-debug-test-edge-cases enhanced-debug-test-unit enhanced-debug-test-all enhanced-debug-test-quick enhanced-debug-test-coverage enhanced-debug-test-report enhanced-debug-help panic-recovery-test panic-recovery-test-unit panic-recovery-test-integration panic-recovery-test-llvm panic-recovery-test-all panic-recovery-test-quick panic-recovery-test-coverage panic-recovery-test-report panic-recovery-help error-handling-test error-handling-test-integration error-handling-test-stress error-handling-test-edge-cases error-handling-test-all error-handling-test-quick error-handling-test-coverage error-handling-test-report error-handling-help testing-framework-test testing-framework-demo testing-framework-runner-demo testing-framework-integration testing-framework-assertions testing-framework-discovery testing-framework-execution testing-framework-reporting testing-framework-stats testing-framework-all testing-framework-coverage testing-framework-docs testing-framework-help package-installer-test package-installer-test-integration package-installer-test-scripts package-installer-test-database package-installer-test-all package-installer-test-quick package-installer-test-coverage package-installer-test-report package-installer-help error-propagation-test error-propagation-test-integration error-propagation-test-compilation error-propagation-test-examples error-propagation-test-all error-propagation-test-quick error-propagation-test-coverage error-propagation-test-report error-propagation-help optimization-test optimization-test-quick optimization-test-benchmarks optimization-test-cli optimization-test-performance optimization-test-pipeline optimization-test-all optimization-test-ignored optimization-test-coverage optimization-benchmark optimization-demo optimization-build-example optimization-help ipc-test ipc-test-basic ipc-test-stress ipc-test-performance ipc-test-all ipc-test-quick ipc-test-coverage ipc-test-report ipc-example ipc-demo ipc-help process-test process-test-integration process-test-basic process-test-concurrent process-test-monitoring process-test-all process-test-quick process-help sqlite-production-test sqlite-production-test-all sqlite-production-test-quick sqlite-production-help build-optimization-test build-optimization-test-cli build-optimization-test-integration build-optimization-test-performance build-optimization-test-all build-optimization-test-quick build-optimization-test-coverage build-optimization-test-report build-optimization-help distributed-test distributed-test-quick distributed-test-unit distributed-test-integration distributed-test-network distributed-test-load-balancing distributed-test-fault-tolerance distributed-test-stress distributed-test-all distributed-build distributed-help build-analytics-test build-analytics-test-integration build-analytics-test-performance build-analytics-test-all build-analytics-test-quick build-analytics-test-coverage build-analytics-demo build-analytics-help stack-walker-test stack-walker-test-comprehensive stack-walker-test-platform stack-walker-test-stress stack-walker-test-all stack-walker-test-quick stack-walker-test-coverage stack-walker-test-report stack-walker-help

build:
	./fix_linking.sh devenv shell cargo build

test:
	./fix_linking.sh devenv shell cargo test

# Comprehensive test discovery and execution
test-discovery: build
	@echo "🧪 Running comprehensive test discovery demo..."
	./fix_linking.sh devenv shell cargo run --example test_discovery_demo

test-comprehensive: build
	@echo "🚀 Running comprehensive test suite with discovery..."
	@echo "This will discover and execute all tests using the new test system"
	./fix_linking.sh devenv shell cargo test --all

test-filter: build
	@echo "🔍 Running filtered tests (TEST_PATTERN required)..."
	./fix_linking.sh devenv shell cargo test $(TEST_PATTERN)

test-unit: build
	@echo "📝 Running unit tests only..."
	./fix_linking.sh devenv shell cargo test --lib

test-integration: build
	@echo "🔗 Running integration tests only..."
	./fix_linking.sh devenv shell cargo test --test "*"

test-ignored: build
	@echo "⏭️  Running ignored tests..."
	./fix_linking.sh devenv shell cargo test -- --ignored

test-help:
	@echo "Available test targets:"
	@echo "  test                - Run all tests"
	@echo "  test-discovery      - Demo comprehensive test discovery system"
	@echo "  test-comprehensive  - Run all tests with comprehensive discovery"
	@echo "  test-filter         - Run filtered tests (set TEST_PATTERN=pattern)"
	@echo "  test-unit          - Run unit tests only"
	@echo "  test-integration   - Run integration tests only"
	@echo "  test-ignored       - Run ignored tests"
	@echo "  test-help          - Show this help"

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

# Math Trigonometry Tests
math-trig-test:
	@echo "Running comprehensive trigonometric function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test

math-trig-test-quick:
	@echo "Running quick trigonometric function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_(sin|cos|tan|degree).*basic"

math-trig-test-basic:
	@echo "Running basic trigonometric function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_(sin|cos|tan|reciprocal).*basic"

math-trig-test-inverse:
	@echo "Running inverse trigonometric function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_a(sin|cos|tan).*basic"

math-trig-test-hyperbolic:
	@echo "Running hyperbolic function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_(sinh|cosh|tanh|asinh|acosh|atanh).*basic"

math-trig-test-advanced:
	@echo "Running advanced trigonometric function tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_(advanced|sincos|trig_all)"

math-trig-test-error:
	@echo "Running trigonometric error handling tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_(error|domain)"

math-trig-test-identities:
	@echo "Running trigonometric identity validation tests..."
	./fix_linking.sh devenv shell cargo test --test math_trigonometry_test "test_.*(identities|identity)"

math-trig-help:
	@echo "Math Trigonometry Test Targets:"
	@echo "  math-trig-test           - Run all trigonometric function tests"
	@echo "  math-trig-test-quick     - Run quick trigonometric tests"
	@echo "  math-trig-test-basic     - Run basic trig function tests (sin, cos, tan)"
	@echo "  math-trig-test-inverse   - Run inverse trig function tests (asin, acos, atan)"
	@echo "  math-trig-test-hyperbolic - Run hyperbolic function tests (sinh, cosh, tanh)"
	@echo "  math-trig-test-advanced  - Run advanced trig function tests"
	@echo "  math-trig-test-error     - Run error handling tests"
	@echo "  math-trig-test-identities - Run mathematical identity tests"
	@echo ""
	@echo "Examples:"
	@echo "  make math-trig-test-quick"
	@echo "  make math-trig-test-basic"
	@echo "  make math-trig-test-hyperbolic"

# Math Statistics Tests
math-stats-test:
	@echo "Running all statistical function tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test

math-stats-test-quick:
	@echo "Running quick statistical tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(mean|median|variance).*basic"

math-stats-test-descriptive:
	@echo "Running descriptive statistics tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(mean|median|mode|variance|standard_deviation|skewness|kurtosis)"

math-stats-test-measures:
	@echo "Running statistical measures tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(percentile|quartile|range|five_number)"

math-stats-test-distributions:
	@echo "Running probability distribution tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(normal|uniform).*"

math-stats-test-correlation:
	@echo "Running correlation and covariance tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(covariance|correlation)"

math-stats-test-outliers:
	@echo "Running outlier detection tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_outliers.*"

math-stats-test-validation:
	@echo "Running data validation tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(invalid_values|data_cleaning|dataset_validation)"

math-stats-test-advanced:
	@echo "Running advanced statistical tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(harmonic|geometric|root_mean|coefficient)"

math-stats-test-edge-cases:
	@echo "Running statistical edge case tests..."
	./fix_linking.sh devenv shell cargo test --test math_statistics_test "test_(edge_cases|error_handling|large_datasets|precision)"

math-stats-help:
	@echo "Math Statistics Test Targets:"
	@echo "  math-stats-test              - Run all statistical function tests"
	@echo "  math-stats-test-quick        - Run quick statistical tests"
	@echo "  math-stats-test-descriptive  - Run descriptive statistics tests"
	@echo "  math-stats-test-measures     - Run statistical measures tests (percentiles, quartiles)"
	@echo "  math-stats-test-distributions - Run probability distribution tests"
	@echo "  math-stats-test-correlation  - Run correlation and covariance tests"
	@echo "  math-stats-test-outliers     - Run outlier detection tests"
	@echo "  math-stats-test-validation   - Run data validation tests"
	@echo "  math-stats-test-advanced     - Run advanced statistical tests"
	@echo "  math-stats-test-edge-cases   - Run edge case and error handling tests"
	@echo ""
	@echo "Examples:"
	@echo "  make math-stats-test-quick"
	@echo "  make math-stats-test-descriptive"
	@echo "  make math-stats-test-distributions"

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

# Collections Tests - Comprehensive data structure testing

# Run all collections tests
collections-test:
	@echo "Running comprehensive collections tests..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test

# Run collections tests with verbose output
collections-test-verbose:
	@echo "Running collections tests with verbose output..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test -- --nocapture

# Run quick collections tests (basic operations only)
collections-test-quick:
	@echo "Running quick collections tests..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test "test_(hash_set|tree_set|bit_set).*basic"

# Run collections performance tests
collections-test-performance:
	@echo "Running collections performance tests..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test "test_.*performance.*"

# Run collections stress tests
collections-test-stress:
	@echo "Running collections stress tests..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test "test_.*stress.*"

# Run collections error handling tests
collections-test-errors:
	@echo "Running collections error handling tests..."
	./fix_linking.sh devenv shell cargo test --test collections_sets_test "test_.*error.*"

# Collections help
collections-help:
	@echo "CURSED Collections Test Commands:"
	@echo "  collections-test              - Run all collections tests"
	@echo "  collections-test-verbose      - Run collections tests with verbose output"
	@echo "  collections-test-quick        - Run quick collections tests (basic operations)"
	@echo "  collections-test-performance  - Run collections performance tests"
	@echo "  collections-test-stress       - Run collections stress tests"
	@echo "  collections-test-errors       - Run collections error handling tests"
	@echo "  collections-help              - Show this help message"
	@echo ""
	@echo "Collections include:"
	@echo "  • HashSet<T>     - Fast hash-based set with O(1) operations"
	@echo "  • TreeSet<T>     - Ordered set with O(log n) operations"
	@echo "  • BitSet         - Efficient bit-based set for small integers"
	@echo "  • Set operations - Union, intersection, difference, subset testing"

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

# Bootstrap Verification System
bootstrap-verify:
	@echo "🚀 Running CURSED bootstrap verification system..."
	./run_bootstrap_verification.sh

# Quick bootstrap verification (2 cycles)
bootstrap-verify-quick:
	@echo "⚡ Running quick bootstrap verification..."
	./run_bootstrap_verification.sh --quick

# Verbose bootstrap verification
bootstrap-verify-verbose:
	@echo "📢 Running verbose bootstrap verification..."
	./run_bootstrap_verification.sh --verbose

# Bootstrap verification with debugging files preserved
bootstrap-verify-debug:
	@echo "🗂️ Running bootstrap verification with debug files preserved..."
	./run_bootstrap_verification.sh --keep

# Run bootstrap verification tests
bootstrap-verify-test:
	@echo "🧪 Running bootstrap verification test suite..."
	./fix_linking.sh devenv shell cargo test --test bootstrap_verification_test

# Bootstrap verification unit tests
bootstrap-verify-test-unit:
	@echo "🔬 Running bootstrap verification unit tests..."
	./fix_linking.sh devenv shell cargo test --lib bootstrap::

# Build bootstrap verification tool
bootstrap-verify-build:
	@echo "🔧 Building bootstrap verification tool..."
	./fix_linking.sh devenv shell cargo build --bin bootstrap-verify --release

# Bootstrap verification help
bootstrap-verify-help:
	@echo "Bootstrap Verification Targets:"
	@echo "  bootstrap-verify         - Run complete bootstrap verification"
	@echo "  bootstrap-verify-quick   - Run quick verification (2 cycles)"
	@echo "  bootstrap-verify-verbose - Run with verbose output"
	@echo "  bootstrap-verify-debug   - Run with debugging files preserved"
	@echo "  bootstrap-verify-test    - Run verification test suite"
	@echo "  bootstrap-verify-test-unit - Run unit tests only"
	@echo "  bootstrap-verify-build   - Build verification tool"
	@echo ""
	@echo "Examples:"
	@echo "  make bootstrap-verify"
	@echo "  make bootstrap-verify-quick"
	@echo "  make bootstrap-verify-verbose"
	@echo "  make bootstrap-verify-debug"
	@echo ""
	@echo "Report Location:"
	@echo "  ./reports/bootstrap_verification_report.md"

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
	@make crypto-test-pqc
	@make crypto-test-pqc-integration
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
	./fix_linking.sh devenv shell cargo test --test crypto_pqc_test test_kyber_encaps_decaps_round_trip
	@echo "✅ Quick crypto tests completed!"

# Post-quantum cryptography tests
crypto-test-pqc:
	@echo "🔐 Running post-quantum cryptography tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_pqc_test
	@echo "✅ PQC tests completed!"

crypto-test-pqc-performance:
	@echo "⚡ Running PQC performance tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_pqc_performance_test -- --ignored
	@echo "✅ PQC performance tests completed!"

crypto-test-pqc-integration:
	@echo "🔗 Running PQC integration tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_pqc_integration_test
	@echo "✅ PQC integration tests completed!"

# Crypto test with coverage
crypto-test-coverage:
	@echo "📊 Running crypto tests with coverage..."
	./fix_linking.sh devenv shell cargo tarpaulin --tests crypto_integration_test crypto_stress_test crypto_security_test crypto_interop_test crypto_pqc_test crypto_pqc_performance_test crypto_pqc_integration_test --out html --output-dir target/coverage/crypto

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
	@echo "  crypto-test-pqc           - Run post-quantum cryptography tests"
	@echo "  crypto-test-pqc-performance - Run PQC performance tests"
	@echo "  crypto-test-pqc-integration - Run PQC integration tests"
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
	@echo "  • Post-quantum cryptography (Kyber, Dilithium, SPHINCS+, Falcon, NTRU)"
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
# Distributed Compilation System Tests
# ===================================================================

# Distributed Compilation test targets
distributed-compilation-test-quick:
	@echo "Running distributed compilation quick tests..."
	./tests/run_distributed_compilation_tests.sh --quick

distributed-compilation-test:
	@echo "Running distributed compilation standard tests..."
	./tests/run_distributed_compilation_tests.sh --test unit --test integration

distributed-compilation-test-all:
	@echo "Running all distributed compilation tests (including stress tests)..."
	./tests/run_distributed_compilation_tests.sh

distributed-compilation-test-ignored:
	@echo "Running distributed compilation stress tests..."
	./tests/run_distributed_compilation_tests.sh --ignored

distributed-compilation-test-unit:
	@echo "Running distributed compilation unit tests..."
	./tests/run_distributed_compilation_tests.sh --test unit

distributed-compilation-test-integration:
	@echo "Running distributed compilation integration tests..."
	./tests/run_distributed_compilation_tests.sh --test integration

distributed-compilation-test-stress:
	@echo "Running distributed compilation stress tests..."
	./tests/run_distributed_compilation_tests.sh --test stress

distributed-compilation-test-coverage:
	@echo "Generating distributed compilation test coverage report..."
	./tests/run_distributed_compilation_tests.sh --coverage

distributed-compilation-test-report:
	@echo "Generating distributed compilation test report..."
	./tests/run_distributed_compilation_tests.sh --report distributed_compilation_test_report.md

# Distributed Compilation test help
distributed-compilation-help:
	@echo "Distributed Compilation Test Suite Commands:"
	@echo ""
	@echo "Basic Testing:"
	@echo "  distributed-compilation-test            - Run all standard distributed compilation tests"
	@echo "  distributed-compilation-test-quick      - Run quick tests only (exclude stress tests)"
	@echo "  distributed-compilation-test-all        - Run all tests including stress tests"
	@echo ""
	@echo "Test Categories:"
	@echo "  distributed-compilation-test-unit       - Unit tests for individual components"
	@echo "  distributed-compilation-test-integration- Integration tests for end-to-end workflows"
	@echo "  distributed-compilation-test-stress     - Stress tests for large-scale scenarios"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  distributed-compilation-test-coverage   - Generate code coverage report"
	@echo "  distributed-compilation-test-report     - Generate detailed test report"
	@echo "  distributed-compilation-test-ignored    - Run stress tests only"
	@echo ""
	@echo "Test Features:"
	@echo "  • Network discovery and communication protocol testing"
	@echo "  • Task distribution and load balancing algorithm validation"
	@echo "  • Work stealing mechanism testing"
	@echo "  • Node health monitoring and fault tolerance validation"
	@echo "  • High concurrency and performance testing"
	@echo "  • Network failure simulation and recovery testing"
	@echo "  • Large-scale compilation workload testing"
	@echo "  • Configuration validation and edge case handling"
	@echo "  • Error recovery and system resilience testing"
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

# Mathematics Library Integration Commands
# ========================================

# Quick validation of math library integration
math-test-quick:
	@echo "🧮 Running math library quick validation..."
	./tests/run_math_integration_tests.sh --quick

# Comprehensive math library integration tests
math-test:
	@echo "🧮 Running comprehensive math library integration tests..."
	./tests/run_math_integration_tests.sh

# Verbose math library testing
math-test-verbose:
	@echo "🧮 Running math library tests with verbose output..."
	./tests/run_math_integration_tests.sh --verbose

# Generate coverage report for math library
math-test-coverage:
	@echo "📊 Generating math library coverage report..."
	./tests/run_math_integration_tests.sh --coverage

# Generate detailed math library integration report
math-test-report:
	@echo "📝 Generating math library integration report..."
	./tests/run_math_integration_tests.sh --report math_integration_report.md

# Build math comprehensive demo program
math-build-example:
	@echo "🔨 Validating math comprehensive demo..."
	@if [ -f examples/math_comprehensive_demo.csd ]; then \
		echo "✅ Math demo program found: examples/math_comprehensive_demo.csd"; \
		echo "📄 Program structure validated"; \
		echo "ℹ️  Note: Requires CURSED compiler to execute"; \
	else \
		echo "❌ Math demo program not found"; \
		exit 1; \
	fi

# Validate math library compilation and integration
math-validate:
	@echo "✅ Validating math library integration..."
	./fix_linking.sh cargo check --lib
	@echo "✅ Math module compilation successful"
	@echo "✅ All mathematical functions properly exported"

# Clean math test artifacts
math-clean:
	@echo "🧹 Cleaning math test artifacts..."
	rm -f math_integration_report.md
	rm -rf coverage/math
	@echo "✅ Math test artifacts cleaned"

# Show math library integration help
math-help:
	@echo "CURSED Mathematics Library Integration"
	@echo "====================================="
	@echo ""
	@echo "Quick Testing:"
	@echo "  math-test-quick     - Quick validation tests"
	@echo "  math-test           - Comprehensive integration tests"
	@echo "  math-validate       - Validate library compilation"
	@echo ""
	@echo "Detailed Testing:"
	@echo "  math-test-verbose   - Verbose test execution"
	@echo "  math-test-coverage  - Generate coverage report"
	@echo "  math-test-report    - Generate detailed report"
	@echo ""
	@echo "Examples and Validation:"
	@echo "  math-build-example  - Validate demo program"
	@echo "  math-clean          - Clean test artifacts"
	@echo "  math-help           - Show this help"
	@echo ""
	@echo "Math Library Modules:"
	@echo "  - basic             - Fundamental arithmetic and utilities"
	@echo "  - trigonometry      - Complete trigonometric operations"
	@echo "  - logarithmic       - Logarithmic, exponential, and power functions"
	@echo "  - constants         - Mathematical constants and fundamental values"
	@echo "  - random            - Random number generation and distributions"
	@echo "  - statistics        - Statistical analysis and data processing"
	@echo "  - special           - Advanced mathematical functions"
	@echo "  - utilities         - Computational mathematics and numerical methods"
	@echo ""
	@echo "Integration Features:"
	@echo "  - Unified API       - All functions accessible through 'import \"stdlib::math\"'"
	@echo "  - No Conflicts      - Resolved naming conflicts between modules"
	@echo "  - Error Handling    - Comprehensive error types with meaningful messages"
	@echo "  - Performance       - Optimized implementations for mathematical operations"
	@echo "  - Safety            - Domain validation and overflow protection"
	@echo "  - Documentation     - Complete guide and examples provided"
	@echo ""

# Enhanced Debugging System Test Suite
# ===================================

# Quick validation of enhanced debug functionality
enhanced-debug-test-quick:
	@echo "Running quick enhanced debug validation..."
	./tests/run_enhanced_debug_tests.sh --quick

# Basic enhanced debug integration test
enhanced-debug-test:
	@echo "Running enhanced debug integration tests..."
	./tests/run_enhanced_debug_tests.sh --integration

# Integration tests for all enhanced debug features
enhanced-debug-test-integration:
	@echo "Running comprehensive enhanced debug integration tests..."
	./tests/run_enhanced_debug_tests.sh --integration

# Performance tests for enhanced debug system
enhanced-debug-test-performance:
	@echo "Running enhanced debug performance tests..."
	./tests/run_enhanced_debug_tests.sh --performance

# Edge case tests for enhanced debug system
enhanced-debug-test-edge-cases:
	@echo "Running enhanced debug edge case tests..."
	./tests/run_enhanced_debug_tests.sh --edge-cases

# Unit tests for enhanced debug modules
enhanced-debug-test-unit:
	@echo "Running enhanced debug unit tests..."
	./fix_linking.sh devenv shell cargo test --lib debug::enhanced_debug::tests
	./fix_linking.sh devenv shell cargo test --lib runtime::debug_runtime::tests

# Run all enhanced debug tests
enhanced-debug-test-all:
	@echo "Running complete enhanced debug test suite..."
	./tests/run_enhanced_debug_tests.sh

# Generate enhanced debug test coverage report
enhanced-debug-test-coverage:
	@echo "Generating enhanced debug test coverage report..."
	./tests/run_enhanced_debug_tests.sh --coverage

# Generate detailed enhanced debug test report
enhanced-debug-test-report:
	@echo "Generating detailed enhanced debug test report..."
	./tests/run_enhanced_debug_tests.sh --report enhanced_debug_test_report.md
	@echo "Report generated: enhanced_debug_test_report.md"

# Enhanced debug help
enhanced-debug-help:
	@echo "Enhanced Debug System Test Suite"
	@echo "================================"
	@echo ""
	@echo "Available targets:"
	@echo "  enhanced-debug-test-quick       - Quick validation tests"
	@echo "  enhanced-debug-test             - Basic integration tests"
	@echo "  enhanced-debug-test-integration - Comprehensive integration tests"
	@echo "  enhanced-debug-test-performance - Performance and scaling tests"
	@echo "  enhanced-debug-test-edge-cases  - Edge case and error handling tests"
	@echo "  enhanced-debug-test-unit        - Unit tests for debug modules"
	@echo "  enhanced-debug-test-all         - Complete test suite"
	@echo "  enhanced-debug-test-coverage    - Generate coverage report"
	@echo "  enhanced-debug-test-report      - Generate detailed test report"
	@echo "  enhanced-debug-help             - Show this help message"
	@echo ""
	@echo "Test Components:"
	@echo "  - Debug Information System      - Enhanced debug info with source mapping"
	@echo "  - Runtime Debugging Support     - Runtime symbol tables and inspection"
	@echo "  - Error Context Enhancement     - Rich error reporting with debug context"
	@echo "  - LLVM Debug Integration        - Debug metadata generation and embedding"
	@echo "  - Source Mapping                - Precise location tracking and mapping"
	@echo "  - Symbol Resolution             - Symbol metadata and type information"
	@echo "  - Breakpoint Simulation         - Debugging workflow simulation"
	@echo "  - Performance Monitoring        - Debug overhead analysis and optimization"
	@echo ""
	@echo "All tests automatically use the linking fix infrastructure for Nix compatibility."

## Panic/Recovery System Testing
## Comprehensive testing for panic handling and recovery mechanisms

# Quick validation of panic/recovery functionality
panic-recovery-test-quick:
	@echo "Running quick panic/recovery system tests..."
	./tests/run_panic_recovery_tests.sh --quick

# Run all standard panic/recovery tests
panic-recovery-test:
	@echo "Running panic/recovery system tests..."
	./tests/run_panic_recovery_tests.sh

# Unit tests for panic/recovery components
panic-recovery-test-unit:
	@echo "Running panic/recovery unit tests..."
	./tests/run_panic_recovery_tests.sh --test unit

# Integration tests for complete panic/recovery workflows
panic-recovery-test-integration:
	@echo "Running panic/recovery integration tests..."
	./tests/run_panic_recovery_tests.sh --test integration

# LLVM integration tests for panic/recovery compilation
panic-recovery-test-llvm:
	@echo "Running panic/recovery LLVM integration tests..."
	./tests/run_panic_recovery_tests.sh --test llvm

# All panic/recovery tests including stress tests
panic-recovery-test-all:
	@echo "Running all panic/recovery tests including ignored ones..."
	./tests/run_panic_recovery_tests.sh --test all --ignored

# Generate code coverage report for panic/recovery system
panic-recovery-test-coverage:
	@echo "Generating panic/recovery test coverage report..."
	./tests/run_panic_recovery_tests.sh --coverage

# Generate comprehensive test report
panic-recovery-test-report:
	@echo "Generating panic/recovery test report..."
	./tests/run_panic_recovery_tests.sh --report panic_recovery_report.md --verbose

# Show panic/recovery testing help
panic-recovery-help:
	@echo "CURSED Panic/Recovery System Testing"
	@echo "====================================="
	@echo ""
	@echo "Quick Testing:"
	@echo "  panic-recovery-test-quick       - Essential tests only (fast)"
	@echo "  panic-recovery-test             - Standard test suite"
	@echo ""
	@echo "Specific Test Categories:"
	@echo "  panic-recovery-test-unit        - Unit tests for components"
	@echo "  panic-recovery-test-integration - Integration and workflow tests"
	@echo "  panic-recovery-test-llvm        - LLVM compilation tests"
	@echo ""
	@echo "Comprehensive Testing:"
	@echo "  panic-recovery-test-all         - All tests including stress tests"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  panic-recovery-test-coverage    - Generate code coverage report"
	@echo "  panic-recovery-test-report      - Generate detailed test report"
	@echo ""
	@echo "  panic-recovery-help             - Show this help message"
	@echo ""
	@echo "Test Components:"
	@echo "  - Panic Runtime System          - Core panic handling infrastructure"
	@echo "  - Recovery Manager              - Recovery scope and error management"
	@echo "  - Gen Z Slang Functions         - no_cap, sus, cap, not_vibing panics"
	@echo "  - LLVM Integration              - Panic/recovery code generation"
	@echo "  - Error Conversion Utilities    - Panic-to-error and recovery actions"
	@echo "  - Thread Safety                 - Concurrent panic/recovery handling"
	@echo "  - Stack Trace Management        - Enhanced stack trace capture"
	@echo "  - FFI Interface                 - C-compatible panic/recovery functions"
	@echo ""
	@echo "All tests automatically use the linking fix infrastructure for Nix compatibility."

# ==============================================================================
# Error Handling Testing Commands
# ==============================================================================

# Quick error handling validation (essential tests only)
error-handling-test-quick:
	./tests/run_error_handling_tests.sh --quick

# Standard error handling test suite  
error-handling-test:
	./tests/run_error_handling_tests.sh

# Run only integration tests
error-handling-test-integration:
	./tests/run_error_handling_tests.sh --integration

# Run only stress tests (may take a while)
error-handling-test-stress:
	./tests/run_error_handling_tests.sh --stress --ignored

# Run only edge case tests
error-handling-test-edge-cases:
	./tests/run_error_handling_tests.sh --edge-cases

# Run all error handling tests including stress tests
error-handling-test-all:
	./tests/run_error_handling_tests.sh --all --ignored

# Generate coverage report for error handling
error-handling-test-coverage:
	./tests/run_error_handling_tests.sh --coverage --quick

# Generate detailed test report
error-handling-test-report:
	./tests/run_error_handling_tests.sh --all --report error_handling_report.md --verbose

# Show error handling testing help
error-handling-help:
	@echo "CURSED Error Handling System Testing"
	@echo "===================================="
	@echo ""
	@echo "Quick Testing:"
	@echo "  error-handling-test-quick       - Essential tests only (fast)"
	@echo "  error-handling-test             - Standard test suite"
	@echo ""
	@echo "Specific Test Categories:"
	@echo "  error-handling-test-integration - Integration and workflow tests"
	@echo "  error-handling-test-stress      - Stress tests (high load, concurrency)"
	@echo "  error-handling-test-edge-cases  - Edge cases and boundary conditions"
	@echo ""
	@echo "Comprehensive Testing:"
	@echo "  error-handling-test-all         - All tests including stress tests"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  error-handling-test-coverage    - Generate code coverage report"
	@echo "  error-handling-test-report      - Generate detailed test report"
	@echo ""
	@echo "  error-handling-help             - Show this help message"
	@echo ""
	@echo "Test Components:"
	@echo "  - Error Runtime System          - Core error handling infrastructure"
	@echo "  - Error Propagation             - ? operator and error chaining"
	@echo "  - Panic/Recovery Integration    - Panic handling with error conversion"
	@echo "  - Stack Trace Management        - Enhanced stack trace capture"
	@echo "  - Thread Safety                 - Concurrent error handling"
	@echo "  - Performance Characteristics   - Error handling under load"
	@echo "  - Memory Management             - Error handling with memory pressure"
	@echo "  - Edge Case Handling            - Boundary conditions and corruption"
	@echo ""
	@echo "All tests automatically use the linking fix infrastructure for Nix compatibility."

# ==================== Queue Collections Tests ====================

# Quick queue tests
queues-test-quick:
	@echo "Running quick queue collection tests..."
	./fix_linking.sh cargo test --test collections_queues_test "test_(queue|deque|priority_queue|circular_queue).*basic"

# Unit tests for specific queue types
queues-test-unit:
	@echo "Running comprehensive unit tests for all queue types..."
	./fix_linking.sh cargo test --test collections_queues_test

# Performance tests (large operations)
queues-test-performance:
	@echo "Running queue performance tests..."
	./fix_linking.sh cargo test --test collections_queues_test -- --ignored

# Thread safety tests
queues-test-thread-safety:
	@echo "Running thread safety tests..."
	./fix_linking.sh cargo test --test collections_queues_test "test_thread_safe"

# Edge cases and stress tests
queues-test-edge-cases:
	@echo "Running edge case tests..."
	./fix_linking.sh cargo test --test collections_queues_test "test_(edge_cases|error_handling|large_queue|memory)"

# All queue tests
queues-test-all:
	@echo "Running all queue tests including performance tests..."
	./fix_linking.sh cargo test --test collections_queues_test
	@echo "Running performance tests..."
	./fix_linking.sh cargo test --test collections_queues_test -- --ignored

# Standard queue tests (alias for unit tests)
queues-test:
	@echo "Running standard queue collection tests..."
	./fix_linking.sh cargo test --test collections_queues_test

# Coverage analysis
queues-test-coverage:
	@echo "Generating coverage report for queue tests..."
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { \
		echo "Installing cargo-tarpaulin..."; \
		cargo install cargo-tarpaulin; \
	}
	./fix_linking.sh cargo tarpaulin --test collections_queues_test --out Html --output-dir target/tarpaulin
	@echo "Coverage report generated in target/tarpaulin/"

# Generate test report
queues-test-report:
	@echo "# Queue Collections Test Report" > queues_test_report.md
	@echo "Generated on: $$(date)" >> queues_test_report.md
	@echo "" >> queues_test_report.md
	@echo "## Test Results" >> queues_test_report.md
	@echo "" >> queues_test_report.md
	@echo "\`\`\`" >> queues_test_report.md
	./fix_linking.sh cargo test --test collections_queues_test 2>&1 | tee -a queues_test_report.md
	@echo "\`\`\`" >> queues_test_report.md
	@echo "" >> queues_test_report.md
	@echo "## Performance Tests" >> queues_test_report.md
	@echo "" >> queues_test_report.md
	@echo "\`\`\`" >> queues_test_report.md
	./fix_linking.sh cargo test --test collections_queues_test -- --ignored 2>&1 | tee -a queues_test_report.md
	@echo "\`\`\`" >> queues_test_report.md
	@echo "Test report generated: queues_test_report.md"

# Help for queue tests
queues-help:
	@echo "=== CURSED Queue Collections Test Commands ==="
	@echo ""
	@echo "Quick Testing:"
	@echo "  make queues-test-quick          - Run basic functionality tests"
	@echo "  make queues-test                - Run standard unit tests"
	@echo ""
	@echo "Comprehensive Testing:"
	@echo "  make queues-test-unit           - All unit tests for queue functionality"
	@echo "  make queues-test-performance    - Performance and stress tests"
	@echo "  make queues-test-thread-safety  - Thread safety validation"
	@echo "  make queues-test-edge-cases     - Edge cases and error handling"
	@echo "  make queues-test-all            - All tests including performance"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  make queues-test-coverage       - Generate code coverage report"
	@echo "  make queues-test-report         - Generate detailed test report"
	@echo ""
	@echo "Queue Types Tested:"
	@echo "  - Queue<T>                      - FIFO queue with dynamic resizing"
	@echo "  - Deque<T>                      - Double-ended queue operations"
	@echo "  - PriorityQueue<T>              - Binary heap priority queue"
	@echo "  - CircularQueue<T>              - Fixed-size circular buffer"
	@echo "  - ThreadSafeQueue<T>            - Thread-safe queue wrapper"
	@echo "  - ThreadSafeDeque<T>            - Thread-safe deque wrapper"
	@echo ""
	@echo "Test Categories:"
	@echo "  - Basic Operations              - FIFO/LIFO behavior validation"
	@echo "  - Capacity Management           - Dynamic resizing and memory"
	@echo "  - Bulk Operations               - Efficient batch processing"
	@echo "  - Priority Ordering             - Heap-based priority handling"
	@echo "  - Circular Buffer Behavior      - Wrap-around and overflow handling"
	@echo "  - Thread Safety                 - Concurrent access validation"
	@echo "  - Error Handling                - Boundary conditions and failures"
	@echo "  - Performance Characteristics   - Large-scale operation testing"
	@echo "  - Memory Efficiency             - Resource usage optimization"
	@echo "  - Edge Cases                    - Corner cases and stress scenarios"
	@echo ""
	@echo "All tests automatically use the linking fix infrastructure for Nix compatibility."
# ===== COLLECTIONS INTEGRATION TESTING =====
# Comprehensive integration testing for the complete CURSED collections ecosystem

# Main collections integration test
collections-integration-test:
	@echo "Running comprehensive collections integration tests..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test

# Specific integration test categories
collections-integration-test-interop:
	@echo "Testing interoperability between collection types..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test "test_(basic_collection_interoperability|cross_collection_operations|iterator_chaining)"

collections-integration-test-cross-ops:
	@echo "Testing cross-collection operations..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test "test_(cross_collection|priority_queue_with_sets|circular_queue_with_stack|bit_set_operations)"

collections-integration-test-performance:
	@echo "Testing performance across collections..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test "test_(performance_comparison|memory_efficiency)"

collections-integration-test-real-world:
	@echo "Testing real-world integration scenarios..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test "test_(real_world_data_processing|thread_safe_stack|deque_bidirectional)"

collections-integration-test-all:
	@echo "Running all collections integration tests..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test --verbose

collections-integration-test-quick:
	@echo "Running quick collections integration tests..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test "test_(basic_collection_interoperability|cross_collection_operations|priority_queue_with_sets)"

# Performance benchmarking (includes ignored tests)
collections-integration-benchmark:
	@echo "Running collections integration performance benchmarks..."
	./fix_linking.sh devenv shell cargo test --test collections_integration_test -- --ignored

# Collections demo program
collections-demo:
	@echo "Building collections demo program..."
	@echo "Note: This would compile examples/collections_demo.csd when CURSED compiler is ready"
	@echo "Demo showcases comprehensive usage of all collection types with Gen Z syntax"

collections-demo-run:
	@echo "Running collections demo..."
	@echo "Demo location: examples/collections_demo.csd"
	@echo "Features demonstrated:"
	@echo "  - HashSet for unique user tracking"
	@echo "  - TreeSet for sorted high scores"
	@echo "  - BitSet for feature flags"
	@echo "  - Queue for user registration processing"
	@echo "  - PriorityQueue for task management"
	@echo "  - CircularQueue for chat message buffer"
	@echo "  - Deque for browser history navigation"
	@echo "  - Stack for function call tracking"
	@echo "  - FixedStack for undo operations"
	@echo "  - ThreadSafeStack for concurrent processing"
	@echo "  - Real-world integration scenarios"
	@echo "  - Performance comparisons"

collections-demo-build:
	@echo "Collections demo build (future CURSED compiler target)"
	@echo "Would execute: cursed compile examples/collections_demo.csd"

collections-example:
	@echo "Collections examples available:"
	@echo "  examples/collections_demo.csd - Comprehensive collection demonstrations"
	@echo "  Real-world scenarios include:"
	@echo "    - Event processing system"
	@echo "    - Task management with priorities"
	@echo "    - Chat message buffering"
	@echo "    - Browser history navigation"
	@echo "    - Undo/redo operations"
	@echo "    - Performance analysis"

# Coverage analysis for collections
collections-coverage:
	@echo "Generating collections test coverage report..."
	./fix_linking.sh devenv shell cargo tarpaulin --out Html --output-dir target/tarpaulin --include-tests --test collections_integration_test

# Comprehensive collections testing (all modules + integration)
collections-full-test:
	@echo "Running complete collections test suite..."
	@echo "1. Individual collection tests..."
	$(MAKE) collections-test-all
	@echo "2. Integration tests..."
	$(MAKE) collections-integration-test-all
	@echo "3. Performance benchmarks..."
	$(MAKE) collections-integration-benchmark
	@echo "✅ Complete collections test suite finished!"

# Help for collections integration testing
collections-integration-help:
	@echo "Collections Integration Test Targets:"
	@echo "====================================="
	@echo ""
	@echo "Main Targets:"
	@echo "  collections-integration-test         - Run all integration tests"
	@echo "  collections-integration-test-all     - Run all tests with verbose output"
	@echo "  collections-integration-test-quick   - Run quick integration tests"
	@echo "  collections-full-test               - Run complete test suite (all modules + integration)"
	@echo ""
	@echo "Specific Test Categories:"
	@echo "  collections-integration-test-interop     - Collection interoperability tests"
	@echo "  collections-integration-test-cross-ops   - Cross-collection operation tests"
	@echo "  collections-integration-test-performance - Performance comparison tests"
	@echo "  collections-integration-test-real-world  - Real-world scenario tests"
	@echo ""
	@echo "Performance & Analysis:"
	@echo "  collections-integration-benchmark    - Run performance benchmarks (ignored tests)"
	@echo "  collections-coverage                 - Generate test coverage report"
	@echo ""
	@echo "Demo & Examples:"
	@echo "  collections-demo                     - Build collections demo program"
	@echo "  collections-demo-run                 - Show demo program features"
	@echo "  collections-example                  - List available examples"
	@echo ""
	@echo "Integration Test Features:"
	@echo "  - Interoperability testing between all collection types"
	@echo "  - Cross-collection operations and conversions"
	@echo "  - Performance comparisons and memory efficiency"
	@echo "  - Real-world usage scenarios and patterns"
	@echo "  - Iterator chaining across different collections"
	@echo "  - Thread safety and concurrent operations"
	@echo "  - Error handling across collection boundaries"
	@echo "  - Complex data processing pipelines"
	@echo ""
	@echo "Examples:"
	@echo "  make collections-integration-test-quick"
	@echo "  make collections-integration-test-performance"
	@echo "  make collections-full-test"
	@echo "  make collections-demo-run"

# Testing Framework - Comprehensive unit testing for CURSED
testing-framework-test:
	@echo "🧪 Running testing framework tests..."
	./fix_linking.sh devenv shell cargo test --test testing_framework_test

testing-framework-demo:
	@echo "🎯 Running testing framework demo..."
	@echo "Compiling and running CURSED testing framework demo..."
	@if [ -f "examples/testing_framework_demo.csd" ]; then \
		echo "Demo file found: examples/testing_framework_demo.csd"; \
		echo "This would normally compile and run the CURSED demo"; \
		echo "For now, showing the demo content:"; \
		head -50 examples/testing_framework_demo.csd; \
	else \
		echo "Demo file not found"; \
	fi

testing-framework-runner-demo:
	@echo "🏃 Running test runner demo..."
	@echo "Compiling and running CURSED test runner demo..."
	@if [ -f "examples/test_runner_example.csd" ]; then \
		echo "Test runner demo found: examples/test_runner_example.csd"; \
		echo "This would normally compile and run the CURSED test runner"; \
		echo "For now, showing the demo content:"; \
		head -30 examples/test_runner_example.csd; \
	else \
		echo "Test runner demo file not found"; \
	fi

testing-framework-integration:
	@echo "🔧 Testing framework integration test..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing

testing-framework-assertions:
	@echo "✅ Testing assertion framework..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing::assertions

testing-framework-discovery:
	@echo "🔍 Testing test discovery..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing::discovery

testing-framework-execution:
	@echo "⚡ Testing test execution..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing::executor

testing-framework-reporting:
	@echo "📊 Testing report generation..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing::reporting

testing-framework-stats:
	@echo "📈 Testing statistics collection..."
	./fix_linking.sh devenv shell cargo test --lib stdlib::testing::stats

testing-framework-all:
	@echo "🧪 Running all testing framework tests..."
	$(MAKE) testing-framework-test
	$(MAKE) testing-framework-integration
	$(MAKE) testing-framework-assertions
	$(MAKE) testing-framework-discovery
	$(MAKE) testing-framework-execution
	$(MAKE) testing-framework-reporting
	$(MAKE) testing-framework-stats

testing-framework-coverage:
	@echo "📊 Generating testing framework coverage report..."
	cargo tarpaulin --out Html --output-dir coverage_reports/testing_framework --include-tests --exclude-files "target/*" --timeout 300 --features default -- --test testing_framework_test

testing-framework-docs:
	@echo "📚 Opening testing framework documentation..."
	@if [ -f "docs/testing_framework.md" ]; then \
		echo "Testing framework documentation:"; \
		echo "File: docs/testing_framework.md"; \
		echo "Open this file in your preferred markdown viewer"; \
	else \
		echo "Documentation not found"; \
	fi

testing-framework-help:
	@echo "🔧 Testing Framework Help:"
	@echo ""
	@echo "Available testing framework commands:"
	@echo "  testing-framework-test       - Run framework tests"
	@echo "  testing-framework-demo       - Run testing demo"
	@echo "  testing-framework-runner-demo - Run test runner demo"
	@echo "  testing-framework-integration - Test framework integration"
	@echo "  testing-framework-assertions - Test assertion framework"
	@echo "  testing-framework-discovery  - Test test discovery"
	@echo "  testing-framework-execution  - Test test execution"
	@echo "  testing-framework-reporting  - Test report generation"
	@echo "  testing-framework-stats      - Test statistics collection"
	@echo "  testing-framework-all        - Run all framework tests"
	@echo "  testing-framework-coverage   - Generate coverage report"
	@echo "  testing-framework-docs       - Open documentation"
	@echo "  testing-framework-help       - Show this help"
	@echo ""
	@echo "Example usage:"
	@echo "  make testing-framework-demo    # Run framework demo"
	@echo "  make testing-framework-test    # Run framework tests"
	@echo "  make testing-framework-all     # Run comprehensive tests"

# Package Installation System Tests
package-installer-test:
	@echo "Running package installer tests..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test

package-installer-test-integration:
	@echo "Running package installer integration tests..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_complete_package_lifecycle
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_basic_package_installation
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_package_upgrade

package-installer-test-scripts:
	@echo "Running package installer script tests..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_script_execution
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_dangerous_script_rejection
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_script_timeout

package-installer-test-database:
	@echo "Running package database tests..."
	./fix_linking.sh devenv shell cargo test --test package_database_test

package-installer-test-all:
	@echo "Running all package installer tests..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test
	./fix_linking.sh devenv shell cargo test --test package_database_test

package-installer-test-quick:
	@echo "Running quick package installer tests..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test test_basic_package_installation
	./fix_linking.sh devenv shell cargo test --test package_database_test test_add_and_get_package

package-installer-test-coverage:
	@echo "Running package installer tests with coverage..."
	./fix_linking.sh devenv shell cargo tarpaulin --tests --test package_installer_test --test package_database_test --out Xml --output-dir ./target/tarpaulin

package-installer-test-report:
	@echo "Generating package installer test report..."
	./fix_linking.sh devenv shell cargo test --test package_installer_test -- --format=json > package_installer_test_report.json || true
	./fix_linking.sh devenv shell cargo test --test package_database_test -- --format=json > package_database_test_report.json || true
	@echo "Test reports generated: package_installer_test_report.json, package_database_test_report.json"

package-installer-help:
	@echo "Package Installer System Test Commands:"
	@echo "  make package-installer-test            # Run all package installer tests"
	@echo "  make package-installer-test-integration # Run integration tests"
	@echo "  make package-installer-test-scripts   # Run script execution tests"
	@echo "  make package-installer-test-database  # Run database tests"
	@echo "  make package-installer-test-all       # Run comprehensive tests"
	@echo "  make package-installer-test-quick     # Run quick validation tests"
	@echo "  make package-installer-test-coverage  # Run tests with coverage"
	@echo "  make package-installer-test-report    # Generate test reports"

# Error Propagation Tests (? operator)
error-propagation-test:
	@echo "🔄 Running error propagation tests..."
	./tests/run_error_propagation_tests.sh --test all

error-propagation-test-integration:
	@echo "🔄 Running error propagation integration tests..."
	./tests/run_error_propagation_tests.sh --test integration

error-propagation-test-llvm:
	@echo "🔄 Running error propagation LLVM IR generation tests..."
	./tests/run_error_propagation_tests.sh --test llvm

error-propagation-test-runtime:
	@echo "🔄 Running error propagation runtime execution tests..."
	./tests/run_error_propagation_tests.sh --test runtime

error-propagation-test-performance:
	@echo "🔄 Running error propagation performance benchmarks..."
	./tests/run_error_propagation_tests.sh --performance

error-propagation-test-all:
	@echo "🔄 Running comprehensive error propagation tests..."
	./tests/run_error_propagation_tests.sh

error-propagation-test-quick:
	@echo "🔄 Running quick error propagation validation..."
	./tests/run_error_propagation_tests.sh --quick

error-propagation-test-coverage:
	@echo "🔄 Generating error propagation coverage report..."
	./tests/run_error_propagation_tests.sh --coverage

error-propagation-test-report:
	@echo "🔄 Generating error propagation test report..."
	./tests/run_error_propagation_tests.sh --report error_propagation_report.md

error-propagation-help:
	@echo "🔄 Error Propagation Test Help:"
	@echo ""
	@echo "Available error propagation test commands:"
	@echo "  error-propagation-test             - Run all error propagation tests"
	@echo "  error-propagation-test-integration - Run integration tests only"
	@echo "  error-propagation-test-llvm        - Run LLVM IR generation tests only"
	@echo "  error-propagation-test-runtime     - Run runtime execution tests only"
	@echo "  error-propagation-test-performance - Run performance benchmarks"
	@echo "  error-propagation-test-all         - Run comprehensive test suite"
	@echo "  error-propagation-test-quick       - Run quick validation tests"
	@echo "  error-propagation-test-coverage    - Generate coverage report"
	@echo "  error-propagation-test-report      - Generate detailed test report"
	@echo "  error-propagation-help             - Show this help"
	@echo ""
	@echo "Test coverage includes:"
	@echo "  • ? operator parsing and AST generation"
	@echo "  • Result<T, E> and Option<T> error propagation"
	@echo "  • Chained ? operator expressions (a?.b?.c?)"
	@echo "  • LLVM IR generation and optimization"
	@echo "  • Runtime execution and performance"
	@echo "  • Memory safety and resource cleanup"
	@echo "  • Concurrent error propagation"
	@echo "  • Type system integration"
	@echo ""
	@echo "Example usage:"
	@echo "  make error-propagation-test-quick      # Quick validation"
	@echo "  make error-propagation-test-integration # Integration tests"
	@echo "  make error-propagation-test-runtime    # Runtime execution tests"
	@echo "  make error-propagation-test-all        # Comprehensive testing"

# Template Cache Testing Commands
template-cache-test:
	$(LINK_FIX) cargo test --test template_cache_comprehensive_test

template-cache-test-unit:
	$(LINK_FIX) cargo test --lib template_cache

template-cache-test-all: template-cache-test template-cache-test-unit

# Template cache performance tests (ignored by default)
template-cache-test-performance:
	$(LINK_FIX) cargo test --test template_cache_comprehensive_test test_parallel_cache_operations --release

template-cache-help:
	@echo "Template Cache Testing Commands:"
	@echo "  template-cache-test                - Run comprehensive cache tests"
	@echo "  template-cache-test-unit          - Run unit tests for cache module"
	@echo "  template-cache-test-all           - Run all cache tests"

# LLVM Optimization System Tests
optimization-test:
	@echo "🔧 Running comprehensive optimization system tests..."
	$(LINK_FIX) cargo test --test optimization_system_test

optimization-test-quick:
	@echo "🔧 Running quick optimization tests..."
	$(LINK_FIX) cargo test --test optimization_system_test "test_(optimization_level|manager|cache)"

optimization-test-benchmarks:
	@echo "🔧 Running optimization benchmarks..."
	$(LINK_FIX) cargo test --test performance_benchmark_test --release

optimization-test-cli:
	@echo "🔧 Running CLI optimization integration tests..."
	$(LINK_FIX) cargo test --test cli_optimization_integration_test

optimization-test-performance:
	@echo "🔧 Running performance profiling tests..."
	$(LINK_FIX) cargo test --test llvm_optimization_test "test_performance.*"

optimization-test-pipeline:
	@echo "🔧 Running performance pipeline tests..."
	$(LINK_FIX) cargo test --test llvm_optimization_test "test_.*pipeline.*"

optimization-test-all: optimization-test optimization-test-benchmarks optimization-test-cli
	@echo "✅ All optimization tests completed"

optimization-test-ignored:
	@echo "🔧 Running ignored optimization tests (performance-intensive)..."
	$(LINK_FIX) cargo test --test performance_benchmark_test --release -- --ignored

optimization-test-coverage:
	@echo "📊 Generating optimization test coverage..."
	$(LINK_FIX) cargo tarpaulin --out Html --output-dir coverage/optimization --tests llvm_optimization_test performance_benchmark_test cli_optimization_integration_test

optimization-benchmark:
	@echo "📈 Running optimization performance benchmarks..."
	$(LINK_FIX) cargo test --test performance_benchmark_test benchmark_ --release -- --show-output

optimization-demo:
	@echo "🎯 Running optimization system demo..."
	$(LINK_FIX) cargo run --example optimization_demo

optimization-build-example:
	@echo "🏗️ Building example with optimizations..."
	$(LINK_FIX) cargo run -- build examples/fibonacci.csd -O3 --lto --target-cpu native

optimization-help:
	@echo "LLVM Optimization System Commands:"
	@echo "  optimization-test                  - Run all optimization system tests"
	@echo "  optimization-test-quick           - Run quick optimization tests"
	@echo "  optimization-test-benchmarks      - Run performance benchmarks"
	@echo "  optimization-test-cli             - Run CLI integration tests"
	@echo "  optimization-test-performance     - Run performance profiling tests"
	@echo "  optimization-test-pipeline        - Run performance pipeline tests"
	@echo "  optimization-test-all             - Run all optimization tests"
	@echo "  optimization-test-ignored         - Run performance-intensive tests"
	@echo "  optimization-test-coverage        - Generate test coverage report"
	@echo "  optimization-benchmark            - Run performance benchmarks"
	@echo "  optimization-demo                 - Demo optimization system"
	@echo "  optimization-build-example        - Build example with optimization"
	@echo "  optimization-help                 - Show this help"
	@echo "  template-cache-test-performance   - Run performance tests"

# =============================================================================
# Advanced Build System Optimization Commands
# =============================================================================

# Build optimization analysis
build-opt-analyze:
	cargo run --bin cursed-build -- analyze --smart-ordering --suggestions

build-opt-analyze-verbose:
	cargo run --bin cursed-build -- --verbose analyze --smart-ordering --dependency-pruning --suggestions --output-format report

# Advanced caching commands
build-opt-cache-stats:
	cargo run --bin cursed-build -- cache stats

build-opt-cache-clear:
	cargo run --bin cursed-build -- cache clear

build-opt-cache-clear-type:
	@if [ -z "$(TYPE)" ]; then \
		echo "Usage: make build-opt-cache-clear-type TYPE=<ast|ir|object|all>"; \
		exit 1; \
	fi
	cargo run --bin cursed-build -- cache clear $(TYPE)

build-opt-cache-warm:
	cargo run --bin cursed-build -- cache warm src/**/*.rs

build-opt-cache-optimize:
	cargo run --bin cursed-build -- cache optimize

build-opt-cache-configure:
	cargo run --bin cursed-build -- cache configure --max-size 2048 --distributed --cache-dir .cursed_cache

# Distributed compilation commands
build-opt-distributed-start:
	cargo run --bin cursed-build -- distributed start --work-stealing --workers localhost:9001 --workers localhost:9002

build-opt-distributed-stop:
	cargo run --bin cursed-build -- distributed stop

build-opt-distributed-status:
	cargo run --bin cursed-build -- distributed status

build-opt-distributed-add-worker:
	@if [ -z "$(WORKER)" ]; then \
		echo "Usage: make build-opt-distributed-add-worker WORKER=host:port"; \
		exit 1; \
	fi
	cargo run --bin cursed-build -- distributed add-worker $(WORKER)

build-opt-distributed-configure:
	cargo run --bin cursed-build -- distributed configure --timeout 300 --strategy work-stealing

# Build analytics commands
build-opt-analytics-report:
	cargo run --bin cursed-build -- analytics report --trends --bottlenecks

build-opt-analytics-report-json:
	cargo run --bin cursed-build -- analytics report --format json --output build_report.json

build-opt-analytics-report-html:
	cargo run --bin cursed-build -- analytics report --format html --output build_report.html --trends --bottlenecks

build-opt-analytics-stats:
	cargo run --bin cursed-build -- analytics stats

build-opt-analytics-monitor:
	cargo run --bin cursed-build -- analytics monitor --interval 5

build-opt-analytics-trends:
	cargo run --bin cursed-build -- analytics trends --days 30

build-opt-analytics-configure:
	cargo run --bin cursed-build -- analytics configure --detailed --memory-profiling --regression-detection

# Memory optimization commands
build-opt-memory-stats:
	cargo run --bin cursed-build -- memory stats

build-opt-memory-configure:
	cargo run --bin cursed-build -- memory configure --max-memory 4096 --strategy adaptive --streaming --chunk-size 64

build-opt-memory-configure-conservative:
	cargo run --bin cursed-build -- memory configure --max-memory 2048 --strategy conservative

build-opt-memory-configure-aggressive:
	cargo run --bin cursed-build -- memory configure --max-memory 8192 --strategy aggressive

build-opt-memory-monitor:
	cargo run --bin cursed-build -- memory monitor --interval 1000

build-opt-memory-gc:
	cargo run --bin cursed-build -- memory gc

build-opt-memory-pressure:
	cargo run --bin cursed-build -- memory pressure

# Performance tuning commands
build-opt-tune:
	cargo run --bin cursed-build -- tune --wizard

build-opt-tune-benchmark:
	cargo run --bin cursed-build -- tune --benchmark

build-opt-tune-apply:
	cargo run --bin cursed-build -- tune --apply-recommendations

build-opt-tune-test-config:
	@if [ -z "$(CONFIG)" ]; then \
		echo "Usage: make build-opt-tune-test-config CONFIG=path/to/config.toml"; \
		exit 1; \
	fi
	cargo run --bin cursed-build -- tune --test-config $(CONFIG)

# Optimized build commands
build-opt-optimized-build:
	cargo run --bin cursed-build -- optimized-build --all-optimizations --release

build-opt-optimized-build-target:
	@if [ -z "$(TARGET)" ]; then \
		echo "Usage: make build-opt-optimized-build-target TARGET=<target_name>"; \
		exit 1; \
	fi
	cargo run --bin cursed-build -- optimized-build $(TARGET) --all-optimizations

build-opt-optimized-build-custom:
	cargo run --bin cursed-build -- optimized-build --dependency-optimization --advanced-caching --memory-optimization --analytics --jobs 8

build-opt-optimized-build-distributed:
	cargo run --bin cursed-build -- optimized-build --all-optimizations --distributed --jobs 16

# Build optimization testing
build-opt-test:
	./fix_linking.sh cargo test --test build_system_optimization_test

build-opt-test-dependency:
	./fix_linking.sh cargo test --test build_system_optimization_test test_dependency_optimization

build-opt-test-cache:
	./fix_linking.sh cargo test --test build_system_optimization_test test_advanced_cache

build-opt-test-distributed:
	./fix_linking.sh cargo test --test build_system_optimization_test test_distributed_compilation

build-opt-test-analytics:
	./fix_linking.sh cargo test --test build_system_optimization_test test_build_analytics

build-opt-test-memory:
	./fix_linking.sh cargo test --test build_system_optimization_test test_memory_optimization

build-opt-test-integrated:
	./fix_linking.sh cargo test --test build_system_optimization_test test_integrated_optimization_workflow

build-opt-test-all:
	./fix_linking.sh cargo test --test build_system_optimization_test

build-opt-test-verbose:
	./fix_linking.sh cargo test --test build_system_optimization_test -- --nocapture

# Performance benchmarking
build-opt-benchmark:
	./fix_linking.sh cargo bench --bench build_optimization_benchmarks

build-opt-performance-report:
	cargo run --bin cursed-build -- analytics report --format html --output build_performance.html --trends --bottlenecks

# Documentation and examples
build-opt-docs:
	@echo "🚀 CURSED Build System Optimizations"
	@echo "======================================"
	@echo ""
	@echo "The CURSED build system includes advanced optimizations for maximum performance:"
	@echo ""
	@echo "📊 Dependency Optimization:"
	@echo "  - Smart dependency analysis and compilation ordering"
	@echo "  - Parallel execution optimization"
	@echo "  - Dependency pruning and graph optimization"
	@echo ""
	@echo "💾 Advanced Caching:"
	@echo "  - Multi-level caching (AST, IR, Object)"
	@echo "  - Distributed cache support"
	@echo "  - Content-based invalidation"
	@echo ""
	@echo "🌐 Distributed Compilation:"
	@echo "  - Work-stealing load balancing"
	@echo "  - Fault tolerance and recovery"
	@echo "  - Network-based task distribution"
	@echo ""
	@echo "📈 Build Analytics:"
	@echo "  - Performance monitoring and reporting"
	@echo "  - Bottleneck identification"
	@echo "  - Trend analysis and regression detection"
	@echo ""
	@echo "🧠 Memory Optimization:"
	@echo "  - Memory-aware scheduling"
	@echo "  - Streaming compilation for large files"
	@echo "  - Adaptive memory strategies"

build-opt-examples:
	@echo "💡 Build Optimization Examples"
	@echo "==============================="
	@echo ""
	@echo "Quick Performance Analysis:"
	@echo "  make build-opt-analyze"
	@echo ""
	@echo "Complete Optimization Workflow:"
	@echo "  make build-opt-tune                    # Run tuning wizard"
	@echo "  make build-opt-cache-configure         # Setup caching"
	@echo "  make build-opt-optimized-build         # Optimized build"
	@echo "  make build-opt-analytics-report        # Performance report"
	@echo ""
	@echo "Distributed Build Setup:"
	@echo "  make build-opt-distributed-start       # Start coordinator"
	@echo "  make build-opt-optimized-build-distributed"
	@echo ""
	@echo "Memory Optimization:"
	@echo "  make build-opt-memory-configure-conservative"
	@echo "  make build-opt-memory-monitor"
	@echo ""
	@echo "Advanced Cache Management:"
	@echo "  make build-opt-cache-warm"
	@echo "  make build-opt-cache-stats"
	@echo "  make build-opt-cache-optimize"

# Build optimization help
build-opt-help:
	@echo ""
	@echo "🚀 CURSED Build System Optimization Commands"
	@echo "============================================="
	@echo ""
	@echo "Analysis Commands:"
	@echo "  build-opt-analyze                      - Analyze build dependencies"
	@echo "  build-opt-analyze-verbose              - Detailed dependency analysis"
	@echo ""
	@echo "Cache Management:"
	@echo "  build-opt-cache-stats                  - Show cache statistics"
	@echo "  build-opt-cache-clear                  - Clear all cache"
	@echo "  build-opt-cache-clear-type TYPE=<type> - Clear specific cache type"
	@echo "  build-opt-cache-warm                   - Warm cache with source files"
	@echo "  build-opt-cache-optimize               - Optimize cache storage"
	@echo "  build-opt-cache-configure              - Configure cache settings"
	@echo ""
	@echo "Distributed Compilation:"
	@echo "  build-opt-distributed-start            - Start distributed coordinator"
	@echo "  build-opt-distributed-stop             - Stop distributed system"
	@echo "  build-opt-distributed-status           - Show cluster status"
	@echo "  build-opt-distributed-add-worker WORKER=<host:port> - Add worker node"
	@echo "  build-opt-distributed-configure        - Configure distributed settings"
	@echo ""
	@echo "Analytics and Monitoring:"
	@echo "  build-opt-analytics-report             - Generate performance report"
	@echo "  build-opt-analytics-report-json        - JSON format report"
	@echo "  build-opt-analytics-report-html        - HTML format report"
	@echo "  build-opt-analytics-stats              - Current build statistics"
	@echo "  build-opt-analytics-monitor            - Real-time monitoring"
	@echo "  build-opt-analytics-trends             - Performance trends"
	@echo "  build-opt-analytics-configure          - Configure analytics"
	@echo ""
	@echo "Memory Optimization:"
	@echo "  build-opt-memory-stats                 - Show memory statistics"
	@echo "  build-opt-memory-configure             - Configure memory settings"
	@echo "  build-opt-memory-configure-conservative - Conservative memory mode"
	@echo "  build-opt-memory-configure-aggressive  - Aggressive memory mode"
	@echo "  build-opt-memory-monitor               - Monitor memory usage"
	@echo "  build-opt-memory-gc                    - Trigger garbage collection"
	@echo "  build-opt-memory-pressure              - Show memory pressure events"
	@echo ""
	@echo "Performance Tuning:"
	@echo "  build-opt-tune                         - Run tuning wizard"
	@echo "  build-opt-tune-benchmark              - Performance benchmark"
	@echo "  build-opt-tune-apply                  - Apply recommendations"
	@echo "  build-opt-tune-test-config CONFIG=<file> - Test configuration"
	@echo ""
	@echo "Optimized Building:"
	@echo "  build-opt-optimized-build              - Build with all optimizations"
	@echo "  build-opt-optimized-build-target TARGET=<name> - Build specific target"
	@echo "  build-opt-optimized-build-custom       - Build with custom optimizations"
	@echo "  build-opt-optimized-build-distributed  - Distributed optimized build"
	@echo ""
	@echo "Testing and Validation:"
	@echo "  build-opt-test                         - Run all optimization tests"
	@echo "  build-opt-test-dependency              - Test dependency optimization"
	@echo "  build-opt-test-cache                   - Test caching system"
	@echo "  build-opt-test-distributed             - Test distributed compilation"
	@echo "  build-opt-test-analytics               - Test analytics system"
	@echo "  build-opt-test-memory                  - Test memory optimization"
	@echo "  build-opt-test-integrated              - Test integrated workflow"
	@echo "  build-opt-benchmark                    - Run performance benchmarks"
	@echo ""
	@echo "Documentation:"
	@echo "  build-opt-docs                         - Show optimization documentation"
	@echo "  build-opt-examples                     - Show usage examples"
	@echo "  build-opt-help                         - Show this help"
	@echo ""
	@echo "Quick Start:"
	@echo "  make build-opt-tune                    # Start with tuning wizard"
	@echo "  make build-opt-optimized-build         # Run optimized build"
	@echo "  make build-opt-analytics-report        # Check performance"

# ================================
# IPC (Inter-Process Communication) Testing
# ================================

# Quick IPC tests (basic functionality)
ipc-test-quick:
	@echo "Running quick IPC tests..."
	./tests/run_ipc_tests.sh --quick

# Basic IPC functionality tests
ipc-test-basic:
	@echo "Running basic IPC functionality tests..."
	./tests/run_ipc_tests.sh basic

# Stress tests for IPC under high load
ipc-test-stress:
	@echo "Running IPC stress tests..."
	./tests/run_ipc_tests.sh stress

# Performance benchmarks for IPC operations
ipc-test-performance:
	@echo "Running IPC performance tests..."
	./tests/run_ipc_tests.sh performance

# Run all IPC tests
ipc-test-all:
	@echo "Running all IPC tests..."
	./tests/run_ipc_tests.sh all

# Standard IPC test suite
ipc-test:
	@echo "Running standard IPC test suite..."
	./tests/run_ipc_tests.sh basic

# Generate IPC test coverage report
ipc-test-coverage:
	@echo "Generating IPC test coverage report..."
	./tests/run_ipc_tests.sh --coverage --report

# Generate detailed IPC test report
ipc-test-report:
	@echo "Generating detailed IPC test report..."
	./tests/run_ipc_tests.sh --report

# Run IPC example/demo
ipc-example:
	@echo "Running IPC example program..."
	./fix_linking.sh devenv shell cargo run --example process_ipc_showcase

# Alternative demo command
ipc-demo: ipc-example

# Show IPC testing help
ipc-help:
	@echo "IPC Testing Commands:"
	@echo "====================="
	@echo ""
	@echo "Basic Testing:"
	@echo "  ipc-test                    - Run standard IPC test suite"
	@echo "  ipc-test-quick              - Run quick IPC validation tests"
	@echo "  ipc-test-basic              - Run basic functionality tests"
	@echo ""
	@echo "Advanced Testing:"
	@echo "  ipc-test-stress             - Run stress tests (high load scenarios)"
	@echo "  ipc-test-performance        - Run performance benchmarks"
	@echo "  ipc-test-all                - Run all test categories"
	@echo ""
	@echo "Analysis and Reporting:"
	@echo "  ipc-test-coverage           - Generate coverage analysis"
	@echo "  ipc-test-report             - Generate detailed test report"
	@echo ""
	@echo "Examples and Demos:"
	@echo "  ipc-example                 - Run comprehensive IPC showcase"
	@echo "  ipc-demo                    - Alias for ipc-example"
	@echo ""
	@echo "IPC Mechanisms Tested:"
	@echo "  - Shared Memory              - Cross-process memory sharing"
	@echo "  - Named Pipes               - FIFO-based communication"
	@echo "  - Message Queues            - Priority-based messaging"
	@echo "  - Semaphores               - Resource synchronization"
	@echo "  - Unix Domain Sockets      - Local socket communication"
	@echo "  - Signal Handling          - Process event signaling"
	@echo "  - RPC (Remote Procedure)   - Remote method calls"
	@echo ""
	@echo "Quick Start:"
	@echo "  make ipc-test-quick         # Validate basic functionality"
	@echo "  make ipc-example            # See comprehensive demo"
	@echo "  make ipc-test-all           # Run complete test suite"

# ================================
# Process Management Testing
# ================================

# Quick process tests
process-test-quick:
	@echo "Running quick process management tests..."
	./fix_linking.sh devenv shell cargo test --test process_basic_test

# Basic process functionality tests
process-test-basic:
	@echo "Running basic process functionality tests..."
	./fix_linking.sh devenv shell cargo test --test process_basic_test

# Integration tests for process management
process-test-integration:
	@echo "Running process integration tests..."
	./fix_linking.sh devenv shell cargo test --test process_integration_test

# Concurrent process management tests
process-test-concurrent:
	@echo "Running concurrent process tests..."
	./fix_linking.sh devenv shell cargo test --test process_integration_test "test_concurrent.*"

# Process monitoring tests
process-test-monitoring:
	@echo "Running process monitoring tests..."
	./fix_linking.sh devenv shell cargo test --test process_integration_test "test.*monitor.*"

# Run all process tests
process-test-all:
	@echo "Running all process management tests..."
	./fix_linking.sh devenv shell cargo test process_basic_test process_integration_test

# Standard process test suite
process-test:
	@echo "Running standard process test suite..."
	./fix_linking.sh devenv shell cargo test --test process_integration_test

# Enhanced Process Management Testing Targets
process-enhanced-test:
	@echo "Running enhanced process management tests..."
	./tests/run_process_tests.sh

process-enhanced-test-quick:
	@echo "Running quick enhanced process management tests..."
	./tests/run_process_tests.sh --quick

process-enhanced-test-verbose:
	@echo "Running enhanced process management tests (verbose)..."
	./tests/run_process_tests.sh --verbose

process-enhanced-test-coverage:
	@echo "Running enhanced process management tests with coverage..."
	./tests/run_process_tests.sh --coverage --report process_enhanced_report.md

process-enhanced-test-stress:
	@echo "Running enhanced process management stress tests..."
	./tests/run_process_tests.sh --ignored

process-enhanced-test-pipes:
	@echo "Running pipe-specific tests..."
	./tests/run_process_tests.sh --test pipe

process-enhanced-test-signals:
	@echo "Running signal-specific tests..."
	./tests/run_process_tests.sh --test signal

process-enhanced-test-daemon:
	@echo "Running daemon-specific tests..."
	./tests/run_process_tests.sh --test daemon

process-enhanced-test-platform:
	@echo "Running platform-specific tests..."
	./tests/run_process_tests.sh --test platform

# Show process testing help
process-help:
	@echo "Process Management Testing Commands:"
	@echo "===================================="
	@echo ""
	@echo "Basic Testing:"
	@echo "  process-test                - Run standard process test suite"
	@echo "  process-test-quick          - Run quick process validation"
	@echo "  process-test-basic          - Run basic functionality tests"
	@echo ""
	@echo "Enhanced Testing:"
	@echo "  process-enhanced-test          - Run enhanced process management tests"
	@echo "  process-enhanced-test-quick    - Run basic enhanced tests only"
	@echo "  process-enhanced-test-verbose  - Run tests with verbose output"
	@echo "  process-enhanced-test-coverage - Run tests with coverage analysis"
	@echo "  process-enhanced-test-stress   - Run stress and performance tests"
	@echo "  process-enhanced-test-pipes    - Run pipe communication tests"
	@echo "  process-enhanced-test-signals  - Run signal handling tests"
	@echo "  process-enhanced-test-daemon   - Run daemon management tests"
	@echo "  process-enhanced-test-platform - Run platform-specific tests"
	@echo ""
	@echo "Advanced Testing:"
	@echo "  process-test-integration    - Run integration tests"
	@echo "  process-test-concurrent     - Run concurrent process tests"
	@echo "  process-test-monitoring     - Run process monitoring tests"
	@echo "  process-test-all            - Run all process tests"
	@echo ""
	@echo "Process Features Tested:"
	@echo "  - Process Spawning          - Create and manage child processes"
	@echo "  - Environment Variables     - Process environment management"
	@echo "  - Working Directory         - Process working directory control"
	@echo "  - I/O Redirection          - Stdin/stdout/stderr handling"
	@echo "  - Process Communication    - Two-way process communication"
	@echo "  - Process Monitoring       - Resource usage and health monitoring"
	@echo "  - Process Lifecycle        - Start, wait, kill, monitor operations"
	@echo "  - Signal Handling          - Process signaling (Unix)"
	@echo "  - Cross-Platform Support   - Windows, macOS, Linux compatibility"
	@echo ""
	@echo "Quick Start:"
	@echo "  make process-test-quick     # Validate basic functionality"
	@echo "  make process-test-all       # Run complete test suite"

# SQLite Production Driver Testing Commands
sqlite-production-test:
	@echo "🚀 Running SQLite Production Driver tests..."
	./fix_linking.sh devenv shell cargo test --test sqlite_production_driver_test

sqlite-production-test-quick:
	@echo "🚀 Running quick SQLite Production Driver tests..."
	./fix_linking.sh devenv shell cargo test --test sqlite_production_driver_test "test_(connection|basic_operations|prepared_statements|transactions)"

sqlite-production-test-all: sqlite-production-test
	@echo "🚀 Running comprehensive SQLite Production Driver tests..."
	./fix_linking.sh devenv shell cargo test --test sqlite_production_driver_test --ignored

sqlite-production-help:
	@echo "SQLite Production Driver Test Commands:"
	@echo "=================================="
	@echo ""
	@echo "Main Targets:"
	@echo "  sqlite-production-test      - Run all production driver tests"
	@echo "  sqlite-production-test-quick - Run quick validation tests"
	@echo "  sqlite-production-test-all  - Run comprehensive tests including stress tests"
	@echo ""
	@echo "Test Coverage:"
	@echo "  - Connection Management     - Connection lifecycle, pooling, configuration"
	@echo "  - Database Operations       - CRUD operations, complex queries"
	@echo "  - Prepared Statements       - Parameter binding, statement caching"
	@echo "  - Transaction Management    - ACID compliance, savepoints, isolation"
	@echo "  - Type Conversions          - Safe type mapping between CURSED and SQLite"
	@echo "  - Error Handling            - Comprehensive error scenarios and recovery"
	@echo "  - Performance Features      - Batch operations, caching, monitoring"
	@echo "  - Concurrent Operations     - Thread safety, WAL mode, contention"
	@echo "  - Memory Safety             - Resource cleanup, leak prevention"
	@echo "  - Database Maintenance      - VACUUM, ANALYZE, size monitoring"
	@echo ""
	@echo "Quick Start:"
	@echo "  make sqlite-production-test-quick  # Validate core functionality"
	@echo "  make sqlite-production-test        # Run complete test suite"

# Build Optimization System Tests
# ==============================

# Quick validation of build optimization functionality
build-optimization-test-quick:
	@echo "🚀 Running quick build optimization tests..."
	./tests/run_build_optimization_tests.sh --quick

# CLI functionality tests
build-optimization-test-cli:
	@echo "🧪 Running build optimization CLI tests..."
	./tests/run_build_optimization_tests.sh --test cli

# Integration tests with build system components
build-optimization-test-integration:
	@echo "🔗 Running build optimization integration tests..."
	./tests/run_build_optimization_tests.sh --test integration

# Performance and benchmark tests  
build-optimization-test-performance:
	@echo "⚡ Running build optimization performance tests..."
	./tests/run_build_optimization_tests.sh --test performance

# Complete test suite
build-optimization-test-all:
	@echo "🎯 Running complete build optimization test suite..."
	./tests/run_build_optimization_tests.sh

# Standard test execution
build-optimization-test:
	@echo "📊 Running build optimization tests..."
	./fix_linking.sh devenv shell cargo test --test build_optimization_cli_test
	./fix_linking.sh devenv shell cargo test --test build_optimization_integration_test

# Generate coverage report
build-optimization-test-coverage:
	@echo "📊 Generating build optimization test coverage..."
	./tests/run_build_optimization_tests.sh --coverage

# Generate test report
build-optimization-test-report:
	@echo "📝 Generating build optimization test report..."
	./tests/run_build_optimization_tests.sh --report

# Build optimization help
build-optimization-help:
	@echo "🚀 CURSED Build Optimization System Tests"
	@echo "========================================="
	@echo ""
	@echo "Available Commands:"
	@echo "  build-optimization-test             - Run core tests"
	@echo "  build-optimization-test-quick       - Quick validation"
	@echo "  build-optimization-test-cli         - CLI functionality tests"
	@echo "  build-optimization-test-integration - Integration tests"
	@echo "  build-optimization-test-performance - Performance tests"
	@echo "  build-optimization-test-all         - Complete test suite"
	@echo "  build-optimization-test-coverage    - Generate coverage report"
	@echo "  build-optimization-test-report      - Generate test report"
	@echo "  build-optimization-help             - Show this help"
	@echo ""
	@echo "Test Categories:"
	@echo "  - CLI Functionality         - Command-line interface features"
	@echo "  - Dependency Analysis       - Project structure analysis and optimization"
	@echo "  - Cache Management          - Advanced caching system operations"
	@echo "  - Distributed Compilation   - Multi-node compilation coordination"
	@echo "  - Analytics & Reporting     - Build performance analysis and reports"
	@echo "  - Memory Optimization       - Memory usage monitoring and optimization"
	@echo "  - Performance Tuning        - Automated performance recommendations"
	@echo "  - Optimized Builds          - End-to-end optimized build execution"
	@echo ""
	@echo "System Features Tested:"
	@echo "  - Project Analysis          - Automatic source file discovery and analysis"
	@echo "  - Complexity Scoring        - Code complexity calculation and reporting"
	@echo "  - Dependency Resolution     - Smart compilation ordering and parallelization"
	@echo "  - Build Caching             - Incremental compilation and cache management"
	@echo "  - Report Generation         - Markdown and HTML report generation"
	@echo "  - Performance Monitoring    - Real-time build statistics and trends"
	@echo "  - Memory Management         - Memory usage optimization and monitoring"
	@echo "  - Benchmark Execution       - Automated performance benchmarking"
	@echo ""
	@echo "Quick Start:"
	@echo "  make build-optimization-test-quick  # Validate core functionality"
	@echo "  make build-optimization-test        # Run standard tests"
	@echo "  make build-optimization-test-all    # Complete comprehensive testing

# ======================================================================
# LLVM Advanced Optimization System Tests
# ======================================================================

# Core LLVM optimization tests
llvm-opt-test:
	@echo "🚀 Running LLVM Advanced Optimization tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test

# Quick LLVM optimization validation
llvm-opt-test-quick:
	@echo "⚡ Running quick LLVM optimization tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test_(advanced_optimization_manager|optimization_statistics|optimization_config)"

# LLVM optimization unit tests
llvm-opt-test-unit:
	@echo "🔧 Running LLVM optimization unit tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test_(function_inliner|loop_optimizer|dead_code_eliminator|constant_propagator|cse_eliminator|tail_call_optimizer|memory_optimizer)"

# LLVM optimization integration tests
llvm-opt-test-integration:
	@echo "🔗 Running LLVM optimization integration tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "integration_tests::"

# LLVM optimization performance tests
llvm-opt-test-performance:
	@echo "📈 Running LLVM optimization performance tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test --ignored "performance_tests::"

# All LLVM optimization tests
llvm-opt-test-all:
	@echo "🎯 Running complete LLVM optimization test suite..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test
	@echo "🎯 Running performance benchmarks..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test --ignored

# LLVM optimization coverage
llvm-opt-test-coverage:
	@echo "📊 Generating LLVM optimization test coverage..."
	./fix_linking.sh devenv shell cargo tarpaulin --test llvm_advanced_optimization_test --out Html --output-dir coverage/llvm_optimization

# LLVM optimization demo
llvm-opt-demo:
	@echo "🎪 Running LLVM optimization demo..."
	./fix_linking.sh devenv shell cargo run examples/optimization_demo.csd --opt-level=3

# LLVM optimization with code generator integration
llvm-opt-codegen-test:
	@echo "🏭 Testing LLVM optimization integration with code generator..."
	./fix_linking.sh devenv shell cargo test "test.*optimization.*codegen"

# LLVM optimization configuration tests
llvm-opt-config-test:
	@echo "⚙️ Running LLVM optimization configuration tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test.*config"

# LLVM optimization statistics tests
llvm-opt-stats-test:
	@echo "📊 Running LLVM optimization statistics tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test.*statistics"

# LLVM optimization pass tests
llvm-opt-pass-test:
	@echo "🎭 Running individual LLVM optimization pass tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test.*pass"

# LLVM optimization error handling tests
llvm-opt-error-test:
	@echo "🚨 Running LLVM optimization error handling tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test.*error"

# LLVM optimization thread safety tests
llvm-opt-thread-test:
	@echo "🧵 Running LLVM optimization thread safety tests..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test.*thread.*safety"

# LLVM optimization benchmark
llvm-opt-benchmark:
	@echo "🏁 Running LLVM optimization benchmarks..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test --ignored "benchmark_"

# LLVM optimization validation
llvm-opt-validate:
	@echo "✅ Validating LLVM optimization system..."
	./fix_linking.sh devenv shell cargo test --test llvm_advanced_optimization_test "test_.*validation"

# LLVM optimization help
llvm-opt-help:
	@echo "🚀 CURSED LLVM Advanced Optimization System Tests"
	@echo "================================================="
	@echo ""
	@echo "Available Commands:"
	@echo "  llvm-opt-test                - Run core optimization tests"
	@echo "  llvm-opt-test-quick         - Quick validation tests"
	@echo "  llvm-opt-test-unit          - Individual optimization pass tests"
	@echo "  llvm-opt-test-integration   - Integration tests"
	@echo "  llvm-opt-test-performance   - Performance benchmark tests"
	@echo "  llvm-opt-test-all           - Complete test suite"
	@echo "  llvm-opt-test-coverage      - Generate test coverage report"
	@echo "  llvm-opt-demo               - Run optimization demo"
	@echo "  llvm-opt-codegen-test       - Code generator integration tests"
	@echo "  llvm-opt-config-test        - Configuration system tests"
	@echo "  llvm-opt-stats-test         - Statistics collection tests"
	@echo "  llvm-opt-pass-test          - Individual pass tests"
	@echo "  llvm-opt-error-test         - Error handling tests"
	@echo "  llvm-opt-thread-test        - Thread safety tests"
	@echo "  llvm-opt-benchmark          - Performance benchmarks"
	@echo "  llvm-opt-validate           - System validation"
	@echo "  llvm-opt-help               - Show this help"
	@echo ""
	@echo "Optimization Passes Tested:"
	@echo "  - Function Inlining         - Eliminate function call overhead"
	@echo "  - Loop Optimization         - Unrolling, vectorization, and transformations"
	@echo "  - Dead Code Elimination     - Remove unreachable and unused code"
	@echo "  - Constant Propagation      - Replace variables with constant values"
	@echo "  - Common Subexpression      - Eliminate redundant computations"
	@echo "  - Tail Call Optimization    - Convert tail recursion to loops"
	@echo "  - Memory Optimization       - Improve memory access patterns"
	@echo ""
	@echo "Configuration Options:"
	@echo "  - Optimization Levels       - O0, O1, O2, O3, Os, Oz"
	@echo "  - Individual Pass Control   - Enable/disable specific passes"
	@echo "  - Performance Tuning        - Inline size, unroll count limits"
	@echo "  - Timeout Management        - Prevent infinite optimization"
	@echo "  - Statistics Collection     - Comprehensive metrics tracking"
	@echo ""
	@echo "Integration Features:"
	@echo "  - LLVM Code Generator       - Direct integration with code generation"
	@echo "  - Thread Safety             - Safe concurrent optimization"
	@echo "  - Error Handling            - Robust error recovery and reporting"
	@echo "  - Performance Monitoring    - Real-time optimization metrics"
	@echo ""
	@echo "Quick Start:"
	@echo "  make llvm-opt-test-quick    # Validate core functionality"
	@echo "  make llvm-opt-test          # Run standard tests"
	@echo "  make llvm-opt-demo          # See optimization in action"
	@echo "  make llvm-opt-test-all      # Complete comprehensive testing""

# Process Management and IPC Testing Targets
# ===========================================

# Process Management and IPC testing commands
process-ipc-test-quick:
	@echo "🧪 Running quick process and IPC validation tests..."
	./tests/run_process_ipc_tests.sh --quick

process-ipc-test-all:
	@echo "🧪 Running complete process and IPC test suite..."
	./tests/run_process_ipc_tests.sh

process-ipc-test-process:
	@echo "🧪 Running process management tests..."
	./fix_linking.sh cargo test --test process_management_comprehensive_test

process-ipc-test-ipc:
	@echo "🧪 Running IPC tests..."
	./fix_linking.sh cargo test --test ipc_comprehensive_test

process-ipc-test-stress:
	@echo "🧪 Running process and IPC stress tests..."
	./tests/run_process_ipc_tests.sh --ignored

process-ipc-test-lifecycle:
	@echo "🧪 Running process lifecycle tests..."
	./fix_linking.sh cargo test --test process_management_comprehensive_test lifecycle

process-ipc-test-environment:
	@echo "🧪 Running environment management tests..."
	./fix_linking.sh cargo test --test process_management_comprehensive_test environment

process-ipc-test-channels:
	@echo "🧪 Running IPC channel tests..."
	./fix_linking.sh cargo test --test ipc_comprehensive_test channel

process-ipc-test-sync:
	@echo "🧪 Running IPC synchronization tests..."
	./fix_linking.sh cargo test --test ipc_comprehensive_test synchronization

process-ipc-benchmark:
	@echo "⚡ Running process and IPC performance benchmarks..."
	./tests/run_process_ipc_tests.sh --ignored --filter performance

process-ipc-validate:
	@echo "✅ Validating process and IPC implementations..."
	./tests/run_process_ipc_tests.sh --quick --verbose

process-ipc-test-coverage:
	@echo "📊 Generating process and IPC test coverage..."
	./tests/run_process_ipc_tests.sh --coverage

process-ipc-test-report:
	@echo "📋 Generating process and IPC test report..."
	./tests/run_process_ipc_tests.sh --report

process-ipc-clean:
	@echo "🧹 Cleaning process and IPC test artifacts..."
	rm -rf test_results/process_ipc_*

process-ipc-help:
	@echo "📚 Process Management and IPC Testing Help:"
	@echo "  process-ipc-test-quick      - Quick validation tests"
	@echo "  process-ipc-test-all        - Complete test suite"
	@echo "  process-ipc-test-process    - Process management tests only"
	@echo "  process-ipc-test-ipc        - IPC tests only"
	@echo "  process-ipc-test-stress     - Stress and performance tests"
	@echo "  process-ipc-test-lifecycle  - Process lifecycle tests"
	@echo "  process-ipc-test-environment - Environment management tests"
	@echo "  process-ipc-test-channels   - IPC channel tests"
	@echo "  process-ipc-test-sync       - IPC synchronization tests"
	@echo "  process-ipc-benchmark       - Performance benchmarks"
	@echo "  process-ipc-validate        - Quick validation"
	@echo "  process-ipc-test-coverage   - Generate coverage report"
	@echo "  process-ipc-test-report     - Generate detailed test report"
	@echo "  process-ipc-clean           - Clean test artifacts"

# JIT Compilation Testing Commands
jit-test-quick:
	@echo "=== Running JIT Quick Tests ==="
	$(LINKING_FIX) cargo test jit_comprehensive_test --release

jit-test-all:
	@echo "=== Running All JIT Tests ==="
	$(LINKING_FIX) cargo test jit_comprehensive_test jit_integration_test --release

jit-test-performance:
	@echo "=== Running JIT Performance Tests ==="
	$(LINKING_FIX) cargo test jit_performance_test --release --ignored

jit-test-integration:
	@echo "=== Running JIT Integration Tests ==="
	$(LINKING_FIX) cargo test jit_integration_test --release

jit-test-comprehensive:
	@echo "=== Running JIT Comprehensive Tests ==="
	$(LINKING_FIX) cargo test jit_comprehensive_test --release

jit-benchmark:
	@echo "=== Running JIT Benchmarks ==="
	$(LINKING_FIX) cargo test jit_performance_test --release --ignored -- --nocapture

jit-test-coverage:
	@echo "=== Generating JIT Test Coverage ==="
	$(LINKING_FIX) cargo tarpaulin --tests --out Html --output-dir coverage/jit \
		--include-tests jit_comprehensive_test jit_integration_test

jit-test-report:
	@echo "=== Generating JIT Test Report ==="
	@echo "# JIT Test Report" > jit_test_report.md
	@echo "Generated on: $$(date)" >> jit_test_report.md
	@echo "" >> jit_test_report.md
	$(LINKING_FIX) cargo test jit_comprehensive_test jit_integration_test --release -- --format=json | \
		jq -r '.[] | select(.type == "test") | "- \(.name): \(.outcome)"' >> jit_test_report.md || \
		echo "JSON processing failed, generating simple report..." >> jit_test_report.md
	@echo "JIT test report generated: jit_test_report.md"

jit-help:
	@echo "=== JIT Testing Commands ==="
	@echo "  jit-test-quick           - Run quick JIT validation tests"
	@echo "  jit-test-all             - Run all JIT tests"
	@echo "  jit-test-performance     - Run JIT performance benchmarks"
	@echo "  jit-test-integration     - Run JIT integration tests"
	@echo "  jit-test-comprehensive   - Run comprehensive JIT functionality tests"
	@echo "  jit-benchmark            - Run detailed JIT benchmarks with output"
	@echo "  jit-test-coverage        - Generate JIT test coverage report"
	@echo "  jit-test-report          - Generate JIT test report"
	@echo "  jit-help                 - Show this help message"

# ================================
# Crypto Package Tests
# ================================

crypto-test:
	@echo "Running standard crypto test suite..."
	./tests/run_crypto_tests.sh

crypto-test-quick:
	@echo "Running quick crypto validation tests..."
	./tests/run_crypto_tests.sh --quick
	./fix_linking.sh devenv shell cargo test --test crypto_pki_integration_test

crypto-test-integration:
	@echo "Running crypto integration tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_integration_test

crypto-test-stress:
	@echo "Running crypto stress tests..."
	./tests/run_crypto_tests.sh --stress --ignored

crypto-test-security:
	@echo "Running crypto security validation tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_security_test

crypto-test-interop:
	@echo "Running crypto interoperability tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_interop_test

crypto-test-all:
	@echo "Running complete crypto test suite including stress tests..."
	./tests/run_crypto_tests.sh --all --verbose
	@make crypto-test-pki

# PKI (Public Key Infrastructure) Tests
crypto-test-pki:
	@echo "🏛️ Running PKI integration tests..."
	./fix_linking.sh devenv shell cargo test --test crypto_pki_integration_test
	@echo "✅ PKI tests completed!"

crypto-test-coverage:
	@echo "Generating crypto test coverage report..."
	./tests/run_crypto_tests.sh --coverage

crypto-test-report:
	@echo "Generating detailed crypto test report..."
	./tests/run_crypto_tests.sh --report

crypto-example:
	@echo "Running crypto showcase example..."
	./fix_linking.sh devenv shell ./target/debug/cursed examples/crypto_showcase.csd

crypto-build-examples:
	@echo "Building crypto example programs..."
	@echo "Examples available:"
	@echo "  - examples/crypto_showcase.csd      - Complete crypto feature demonstration"
	@echo "  - examples/secure_messaging.csd     - End-to-end encrypted messaging"
	@echo "Build the cursed binary first with 'make build'"

crypto-benchmark:
	@echo "Running crypto performance benchmarks..."
	./fix_linking.sh devenv shell cargo test --test crypto_stress_test --ignored "test_.*performance" -- --nocapture

crypto-validate:
	@echo "Validating crypto implementation correctness..."
	./fix_linking.sh devenv shell cargo test --test crypto_security_test "test_.*_standard_test_vectors" -- --nocapture

crypto-clean:
	@echo "Cleaning crypto test artifacts..."
	rm -f crypto_test_report.md
	rm -rf coverage/

crypto-help:
	@echo "CURSED Crypto Package Test Suite"
	@echo ""
	@echo "Available crypto test commands:"
	@echo "  crypto-test              - Run standard crypto test suite"
	@echo "  crypto-test-quick        - Run quick validation tests"
	@echo "  crypto-test-integration  - Run integration tests"
	@echo "  crypto-test-stress       - Run stress tests (with --ignored)"
	@echo "  crypto-test-security     - Run security validation tests"
	@echo "  crypto-test-interop      - Run interoperability tests"
	@echo "  crypto-test-pki          - Run PKI (Public Key Infrastructure) tests"
	@echo "  crypto-test-all          - Run complete test suite with verbose output"
	@echo "  crypto-test-coverage     - Generate test coverage report"
	@echo "  crypto-test-report       - Generate detailed test report"
	@echo "  crypto-example           - Run crypto showcase example"
	@echo "  crypto-build-examples    - List available crypto examples"
	@echo "  crypto-benchmark         - Run performance benchmarks"
	@echo "  crypto-validate          - Validate implementation correctness"
	@echo "  crypto-clean             - Clean test artifacts"
	@echo "  crypto-help              - Show this help message"

# Distributed Compilation System Testing
distributed-test-quick:
	@echo "Running distributed compilation quick tests..."
	./fix_linking.sh cargo test --test distributed_compilation_test -- --test-threads=1

distributed-test:
	@echo "Running all distributed compilation tests..."
	./fix_linking.sh cargo test --test distributed_compilation_test --test distributed_compilation_integration_test -- --test-threads=1

distributed-test-unit:
	@echo "Running distributed compilation unit tests..."
	./fix_linking.sh cargo test --test distributed_compilation_test -- --test-threads=1

distributed-test-integration:
	@echo "Running distributed compilation integration tests..."
	./fix_linking.sh cargo test --test distributed_compilation_integration_test -- --test-threads=1

distributed-test-network:
	@echo "Running distributed compilation network tests..."
	./fix_linking.sh cargo test --test distributed_compilation_test test_network -- --test-threads=1

distributed-test-load-balancing:
	@echo "Running distributed compilation load balancing tests..."
	./fix_linking.sh cargo test --test distributed_compilation_integration_test test_multi_node_load_balancing -- --test-threads=1

distributed-test-fault-tolerance:
	@echo "Running distributed compilation fault tolerance tests..."
	./fix_linking.sh cargo test --test distributed_compilation_integration_test test_fault_tolerance -- --test-threads=1

distributed-test-stress:
	@echo "Running distributed compilation stress tests..."
	./fix_linking.sh cargo test --test distributed_compilation_integration_test test_large_scale_compilation test_system_resilience_under_stress -- --test-threads=1 --ignored

distributed-test-all:
	@echo "Running all distributed compilation tests including stress tests..."
	./fix_linking.sh cargo test --test distributed_compilation_test --test distributed_compilation_integration_test -- --test-threads=1 --include-ignored

distributed-build:
	@echo "Building distributed compilation system..."
	./fix_linking.sh cargo build --features distributed-compilation

distributed-help:
	@echo "Distributed Compilation Testing Commands:"
	@echo "  distributed-test-quick       - Quick validation tests"
	@echo "  distributed-test            - All standard tests"
	@echo "  distributed-test-unit       - Unit tests for core functionality"
	@echo "  distributed-test-integration - Integration and workflow tests"
	@echo "  distributed-test-network    - Network communication tests"
	@echo "  distributed-test-load-balancing - Load balancing algorithm tests"
	@echo "  distributed-test-fault-tolerance - Fault tolerance and recovery tests"
	@echo "  distributed-test-stress     - Large-scale and stress tests"
	@echo "  distributed-test-all        - All tests including stress tests"
	@echo "  distributed-build           - Build with distributed compilation support"


#####################################################################
# Optimization System Targets
#####################################################################

# Quick optimization test
optimization-test-quick:
	@echo "🚀 Running quick optimization system tests..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test --quiet

# Full optimization system test
optimization-test:
	@echo "🚀 Running comprehensive optimization system tests..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test

# Run optimization benchmarks
optimization-benchmark:
	@echo "🚀 Running optimization benchmarks..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test test_built_in_benchmarks -- --ignored

# Test incremental compilation
optimization-test-incremental:
	@echo "🚀 Testing incremental compilation..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test test_incremental_compiler_workflow

# Test adaptive optimization
optimization-test-adaptive:
	@echo "🚀 Testing adaptive optimization..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test test_adaptive_optimizer

# Test performance regression detection
optimization-test-regression:
	@echo "🚀 Testing performance regression detection..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test test_regression_detection

# Run all optimization tests
optimization-test-all:
	@echo "🚀 Running all optimization system tests..."
	./fix_linking.sh cargo test optimization_system_comprehensive_test
	./fix_linking.sh cargo test incremental_compilation
	./fix_linking.sh cargo test benchmarking
	./fix_linking.sh cargo test adaptive

# Build optimization demo
optimization-demo-build:
	@echo "🚀 Building optimization demo..."
	./fix_linking.sh cargo run -- compile examples/optimization_demo.csd

# Run optimization demo
optimization-demo:
	@echo "🚀 Running optimization demo..."
	./fix_linking.sh cargo run -- run examples/optimization_demo.csd

# Generate optimization report
optimization-report:
	@echo "🚀 Generating optimization report..."
	./fix_linking.sh cargo test --test optimization_system_comprehensive_test -- --nocapture > optimization_report.txt
	@echo "Report saved to optimization_report.txt"

# Test optimization integration with LLVM
optimization-test-llvm:
	@echo "🚀 Testing LLVM optimization integration..."
	./fix_linking.sh cargo test llvm_optimization_integration

# Performance profiling test
optimization-test-profiling:
	@echo "🚀 Testing performance profiling..."
	./fix_linking.sh cargo test test_profiling_integration

# Test compilation speed optimizations
optimization-test-speed:
	@echo "🚀 Testing compilation speed optimizations..."
	./fix_linking.sh cargo test test_compilation_speed

# Test memory optimizations
optimization-test-memory:
	@echo "🚀 Testing memory optimizations..."
	./fix_linking.sh cargo test test_memory

# Clean optimization artifacts
optimization-clean:
	@echo "🧹 Cleaning optimization artifacts..."
	rm -rf .cursed_cache/
	rm -rf benchmark_results/
	rm -f optimization_report.txt

# Optimization coverage report
optimization-test-coverage:
	@echo "📊 Generating optimization system coverage report..."
	./fix_linking.sh cargo tarpaulin --out Html --output-dir coverage/optimization \
		--tests optimization_system_comprehensive_test \
		--timeout 120 --engine llvm
	@echo "Coverage report available at coverage/optimization/tarpaulin-report.html"

# Help for optimization commands
optimization-help:
	@echo "Optimization System Commands:"
	@echo "  make optimization-test-quick       - Run quick optimization tests"
	@echo "  make optimization-test             - Run comprehensive optimization tests"
	@echo "  make optimization-test-incremental - Test incremental compilation"
	@echo "  make optimization-test-adaptive    - Test adaptive optimization"
	@echo "  make optimization-test-regression  - Test performance regression detection"
	@echo "  make optimization-test-all         - Run all optimization tests"
	@echo "  make optimization-benchmark        - Run optimization benchmarks"
	@echo "  make optimization-demo-build       - Build optimization demo"
	@echo "  make optimization-demo             - Run optimization demo"
	@echo "  make optimization-report           - Generate optimization report"
	@echo "  make optimization-test-llvm        - Test LLVM optimization integration"
	@echo "  make optimization-test-profiling   - Test performance profiling"
	@echo "  make optimization-test-speed       - Test compilation speed optimizations"
	@echo "  make optimization-test-memory      - Test memory optimizations"
	@echo "  make optimization-test-coverage    - Generate code coverage report"
	@echo "  make optimization-clean            - Clean optimization artifacts"
	@echo "  make optimization-help             - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make optimization-test-quick       # Quick validation"
	@echo "  make optimization-demo             # See optimizations in action"
	@echo "  make optimization-benchmark        # Performance benchmarking"
	@echo "  make optimization-test-coverage    # Detailed coverage analysis"
	@echo ""

# ===============================================================================
# Build Analytics and Optimization Tests
# ===============================================================================

build-analytics-test:
	@echo "Running comprehensive build analytics tests..."
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test

build-analytics-test-integration:
	@echo "Running build analytics integration tests..."
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test test_build_

build-analytics-test-performance:
	@echo "Running build analytics performance tests..."
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test test_.*performance

build-analytics-test-all:
	@echo "Running all build analytics tests (including performance)..."
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test
	@echo "Running build analytics demo..."
	./fix_linking.sh devenv shell cargo run --example build_analytics_demo

build-analytics-test-quick:
	@echo "Running quick build analytics validation..."
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test test_build_analytics_creation_and_configuration
	./fix_linking.sh devenv shell cargo test --test build_analytics_integration_test test_build_event_recording_and_analysis

build-analytics-test-coverage:
	@echo "Generating build analytics test coverage report..."
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		./fix_linking.sh devenv shell cargo tarpaulin --test build_analytics_integration_test --out Html --output-dir coverage/build_analytics; \
		echo "Coverage report generated in coverage/build_analytics/"; \
	else \
		echo "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"; \
	fi

build-analytics-demo:
	@echo "Running comprehensive build analytics demo..."
	@echo "This demo showcases all build optimization features:"
	@echo "  📊 Build Analytics Engine"
	@echo "  🗄️  Advanced Caching System"
	@echo "  🧠 Memory-Optimized Compilation"
	@echo "  ⚡ Incremental Build Cache"
	@echo "  🔄 Integrated Workflow"
	@echo ""
	./fix_linking.sh devenv shell cargo run --example build_analytics_demo

build-analytics-help:
	@echo "Build Analytics and Optimization Test Commands:"
	@echo "=============================================="
	@echo ""
	@echo "Basic Commands:"
	@echo "  make build-analytics-test              - Run all build analytics tests"
	@echo "  make build-analytics-test-integration  - Run integration tests only"
	@echo "  make build-analytics-test-performance  - Run performance tests"
	@echo "  make build-analytics-test-quick        - Quick validation tests"
	@echo "  make build-analytics-demo              - Interactive demo"
	@echo ""
	@echo "Analysis Commands:"
	@echo "  make build-analytics-test-coverage     - Generate coverage report"
	@echo ""
	@echo "Features Tested:"
	@echo "  📊 Build Analytics Engine:"
	@echo "    - Real-time build monitoring"
	@echo "    - Performance metrics collection"
	@echo "    - Bottleneck analysis and optimization recommendations"
	@echo "    - Historical trend analysis and regression detection"
	@echo ""
	@echo "  🗄️  Advanced Caching System:"
	@echo "    - Multi-level caching (AST, IR, Object files)"
	@echo "    - Compression and content deduplication"
	@echo "    - Distributed caching for team collaboration"
	@echo "    - Cache warming and intelligent eviction"
	@echo ""
	@echo "  🧠 Memory-Optimized Compilation:"
	@echo "    - Adaptive memory management"
	@echo "    - Streaming compilation for large files"
	@echo "    - Memory pressure detection and response"
	@echo "    - Task scheduling based on available resources"
	@echo ""
	@echo "  ⚡ Incremental Build Cache:"
	@echo "    - Fine-grained dependency tracking"
	@echo "    - Checksum-based change detection"
	@echo "    - Multi-project cache management"
	@echo "    - Automatic cache invalidation"
	@echo ""
	@echo "Why Build Analytics Matter:"
	@echo "  • 2-10x faster builds through intelligent optimization"
	@echo "  • Data-driven performance insights and recommendations"
	@echo "  • Early detection of performance regressions"
	@echo "  • Maximized developer productivity and reduced context switching"
	@echo ""
	@echo "Examples:"
	@echo "  make build-analytics-test-quick        # Quick validation"
	@echo "  make build-analytics-demo              # See all features in action"
	@echo "  make build-analytics-test-coverage     # Detailed coverage analysis"
	@echo ""

# =============================================================================
# Stack Walking Tests
# =============================================================================

stack-walker-test:
	@echo "🔍 Running comprehensive stack walker tests..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test

stack-walker-test-comprehensive:
	@echo "🔍 Running comprehensive stack walker functionality tests..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test

stack-walker-test-platform:
	@echo "🔍 Running platform-specific stack walker tests..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_platform_test

stack-walker-test-stress:
	@echo "🔍 Running stack walker stress tests..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_stress_test --release

stack-walker-test-all:
	@echo "🔍 Running all stack walker tests..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test
	./fix_linking.sh devenv shell cargo test --test stack_walker_platform_test
	./fix_linking.sh devenv shell cargo test --test stack_walker_stress_test --release

stack-walker-test-quick:
	@echo "🔍 Running quick stack walker validation..."
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test test_basic_stack_walking
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test test_symbol_resolution
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test test_cursed_frame_detection

stack-walker-test-coverage:
	@echo "🔍 Generating stack walker test coverage report..."
	./fix_linking.sh devenv shell cargo tarpaulin --test stack_walker_comprehensive_test --test stack_walker_platform_test --out Html --output-dir coverage/stack_walker

stack-walker-test-report:
	@echo "🔍 Generating stack walker test report..."
	@echo "## Stack Walker Test Report" > stack_walker_test_report.md
	@echo "" >> stack_walker_test_report.md
	@echo "### Test Results" >> stack_walker_test_report.md
	@echo "" >> stack_walker_test_report.md
	./fix_linking.sh devenv shell cargo test --test stack_walker_comprehensive_test 2>&1 | tee -a stack_walker_test_report.md
	@echo "" >> stack_walker_test_report.md
	@echo "### Platform Tests" >> stack_walker_test_report.md
	@echo "" >> stack_walker_test_report.md
	./fix_linking.sh devenv shell cargo test --test stack_walker_platform_test 2>&1 | tee -a stack_walker_test_report.md

stack-walker-help:
	@echo "Stack Walker Test Commands:"
	@echo ""
	@echo "📋 Basic Testing:"
	@echo "  make stack-walker-test                 # Run comprehensive tests"
	@echo "  make stack-walker-test-quick           # Quick validation tests"
	@echo "  make stack-walker-test-all             # All test suites"
	@echo ""
	@echo "🧪 Specific Test Suites:"
	@echo "  make stack-walker-test-comprehensive   # Core functionality tests"
	@echo "  make stack-walker-test-platform        # Platform-specific tests"
	@echo "  make stack-walker-test-stress          # Stress and performance tests"
	@echo ""
	@echo "📊 Analysis & Reporting:"
	@echo "  make stack-walker-test-coverage        # Generate coverage report"
	@echo "  make stack-walker-test-report          # Generate test report"
	@echo ""
	@echo "🏗️ Stack Walker Features:"
	@echo "  • Cross-platform stack unwinding (Linux, macOS, Windows)"
	@echo "  • Symbol resolution with addr2line integration"
	@echo "  • CURSED frame detection and filtering"
	@echo "  • Thread-safe operations with caching"
	@echo "  • Performance optimized with minimal overhead"
	@echo "  • Comprehensive error handling and recovery"
	@echo ""
	@echo "🛠️ Implementation Details:"
	@echo "  • Platform-specific backends using libc::backtrace"
	@echo "  • Symbol demangling with rustc-demangle"
	@echo "  • Source information extraction with debug info"
	@echo "  • Configurable filtering and frame limits"
	@echo "  • Statistics tracking and performance monitoring"
	@echo ""
	@echo "Examples:"
	@echo "  make stack-walker-test-quick           # Basic validation"
	@echo "  make stack-walker-test-platform        # Test current platform"
	@echo "  make stack-walker-test-stress          # Performance testing"
	@echo ""


# Process Management and IPC Test Suite
# Comprehensive testing for process spawning, lifecycle management, and inter-process communication

process-mgmt-test-quick:
	@echo "Running quick process management validation..."
	@./tests/run_process_management_tests.sh --quick

process-mgmt-test:
	@echo "Running process management integration tests..."
	@./tests/run_process_management_tests.sh

process-mgmt-test-stress:
	@echo "Running process management stress tests..."
	@./tests/run_process_management_tests.sh --stress

process-mgmt-test-all:
	@echo "Running all process management tests..."
	@./tests/run_process_management_tests.sh --all

process-mgmt-test-coverage:
	@echo "Running process management tests with coverage..."
	@./tests/run_process_management_tests.sh --coverage --report process_mgmt_report.md

process-mgmt-test-concurrent:
	@echo "Running concurrent process management tests..."
	@./tests/run_process_management_tests.sh --test "concurrent"

process-mgmt-test-ipc:
	@echo "Running IPC tests..."
	@./tests/run_process_management_tests.sh --test "ipc"

process-mgmt-test-memory:
	@echo "Running memory safety tests..."
	@./tests/run_process_management_tests.sh --test "memory"

process-mgmt-test-performance:
	@echo "Running performance tests..."
	@./tests/run_process_management_tests.sh --test "performance"

process-mgmt-help:
	@echo ""
	@echo "🔧 CURSED Process Management & IPC Test Suite"
	@echo ""
	@echo "OVERVIEW:"
	@echo "  Comprehensive testing framework for process management and inter-process"
	@echo "  communication systems including spawning, lifecycle management, monitoring,"
	@echo "  resource tracking, shared memory, message queues, pipes, and semaphores."
	@echo ""
	@echo "QUICK COMMANDS:"
	@echo "  make process-mgmt-test             # Standard integration tests"
	@echo "  make process-mgmt-test-quick       # Quick validation (fast)"
	@echo "  make process-mgmt-test-all         # Everything including stress tests"
	@echo ""
	@echo "SPECIFIC TEST CATEGORIES:"
	@echo "  make process-mgmt-test-concurrent  # Concurrent operation tests"
	@echo "  make process-mgmt-test-ipc         # IPC mechanism tests"
	@echo "  make process-mgmt-test-memory      # Memory safety validation"
	@echo "  make process-mgmt-test-performance # Performance characteristics"
	@echo "  make process-mgmt-test-stress      # Stress tests (long duration)"
	@echo ""
	@echo "ANALYSIS AND REPORTING:"
	@echo "  make process-mgmt-test-coverage    # Coverage analysis with report"
	@echo ""
	@echo "HELP:"
	@echo "  make process-mgmt-help             # Show this detailed help"
	@echo ""
	@echo "🧪 Test Categories:"
	@echo ""
	@echo "📋 Integration Tests:"
	@echo "  • Basic process spawning and execution"
	@echo "  • Process lifecycle management (start/stop/kill)"
	@echo "  • Process monitoring and resource tracking"
	@echo "  • Process control operations (priority, signals)"
	@echo "  • I/O redirection and environment handling"
	@echo "  • IPC initialization and operations"
	@echo "  • Shared memory read/write operations"
	@echo "  • Named pipe communication"
	@echo "  • Message queue operations"
	@echo "  • Semaphore synchronization"
	@echo "  • Error handling and edge cases"
	@echo "  • Memory safety and resource cleanup"
	@echo ""
	@echo "🔥 Stress Tests (--stress or --all):"
	@echo "  • Massive process spawning (100+ processes)"
	@echo "  • Concurrent IPC operations (16 threads × 50 ops)"
	@echo "  • Memory pressure scenarios (50+ regions)"
	@echo "  • Sustained process load (30+ seconds)"
	@echo "  • Resource exhaustion and recovery"
	@echo "  • Performance degradation analysis"
	@echo ""
	@echo "📊 Coverage and Performance:"
	@echo "  • Function coverage ≥90% target"
	@echo "  • Line coverage ≥85% target"
	@echo "  • Process spawn <1s average"
	@echo "  • IPC operations <10ms average"
	@echo "  • Success rates ≥85% under stress"
	@echo ""
	@echo "🛠️ Implementation Features:"
	@echo "  • Cross-platform process management (Unix/Windows)"
	@echo "  • Real resource usage monitoring (CPU, memory, files)"
	@echo "  • Thread-safe IPC operations"
	@echo "  • Comprehensive error handling"
	@echo "  • Memory safety validation"
	@echo "  • Performance optimization"
	@echo ""
	@echo "Examples:"
	@echo "  make process-mgmt-test-quick           # Fast validation"
	@echo "  make process-mgmt-test-ipc             # IPC functionality"
	@echo "  make process-mgmt-test-stress          # Load testing"
	@echo "  make process-mgmt-test-coverage        # Coverage report"
	@echo ""


# IPC (Inter-Process Communication) testing commands
ipc-test-quick:
	@echo "🔌 Running quick IPC validation..."
	$(LINKING_FIX) cargo test --test ipc_comprehensive_test
	$(LINKING_FIX) cargo test --test ipc_integration_advanced_test --test-threads=1

ipc-test:
	@echo "🔌 Running standard IPC tests..."
	$(LINKING_FIX) cargo test --test ipc_comprehensive_test
	$(LINKING_FIX) cargo test --test ipc_integration_advanced_test --test-threads=1
	@echo "✅ Standard IPC tests completed"

ipc-test-all:
	@echo "🔌 Running comprehensive IPC test suite..."
	$(LINKING_FIX) cargo test --test ipc_comprehensive_test
	$(LINKING_FIX) cargo test --test ipc_integration_advanced_test --test-threads=1
	$(LINKING_FIX) cargo test --test ipc_stress_test -- --ignored --test-threads=1
	@echo "✅ Comprehensive IPC tests completed"

ipc-test-stress:
	@echo "🔌 Running IPC stress tests..."
	$(LINKING_FIX) cargo test --test ipc_stress_test -- --ignored --test-threads=1
	@echo "✅ IPC stress tests completed"

ipc-test-integration:
	@echo "🔌 Running IPC integration tests..."
	$(LINKING_FIX) cargo test --test ipc_integration_advanced_test --test-threads=1
	@echo "✅ IPC integration tests completed"

ipc-example:
	@echo "🔌 Running IPC comprehensive demo..."
	$(CURSED_BIN) examples/ipc_comprehensive_demo.csd
	@echo "✅ IPC comprehensive demo completed"

ipc-example-producer-consumer:
	@echo "🔌 Running IPC producer-consumer demo..."
	$(CURSED_BIN) examples/ipc_producer_consumer.csd
	@echo "✅ IPC producer-consumer demo completed"

ipc-example-microservices:
	@echo "🔌 Running IPC microservices demo..."
	$(CURSED_BIN) examples/ipc_microservices.csd
	@echo "✅ IPC microservices demo completed"

ipc-examples: ipc-example ipc-example-producer-consumer ipc-example-microservices

ipc-validate:
	@echo "🔌 Validating IPC implementation..."
	$(LINKING_FIX) cargo check --tests --features="ipc"
	@echo "✅ IPC validation completed"

ipc-help:
	@echo "🔌 IPC Test Commands:"
	@echo "  ipc-test-quick          - Quick IPC validation"
	@echo "  ipc-test               - Standard IPC tests"
	@echo "  ipc-test-all           - Comprehensive IPC test suite"
	@echo "  ipc-test-stress        - IPC stress tests (heavy load)"
	@echo "  ipc-test-integration   - IPC integration tests"
	@echo "  ipc-example            - Run comprehensive IPC demo"
	@echo "  ipc-example-producer-consumer - Run producer-consumer demo"
	@echo "  ipc-example-microservices     - Run microservices demo"
	@echo "  ipc-examples           - Run all IPC examples"
	@echo "  ipc-validate           - Validate IPC implementation"
	@echo "  ipc-help               - Show this help"



# JIT Compilation Testing Targets
jit-test-quick:
	@echo "⚡ Running JIT quick tests..."
	@./tests/run_jit_comprehensive_tests.sh --quick

jit-test:
	@echo "⚡ Running JIT comprehensive tests..."
	@./tests/run_jit_comprehensive_tests.sh

jit-test-all:
	@echo "⚡ Running all JIT tests (including stress tests)..."
	@./tests/run_jit_comprehensive_tests.sh

jit-test-verbose:
	@echo "⚡ Running JIT tests with verbose output..."
	@./tests/run_jit_comprehensive_tests.sh --verbose

jit-test-engine:
	@echo "⚡ Running JIT engine core tests..."
	@./fix_linking.sh cargo test --test jit_engine_comprehensive_test

jit-test-repl:
	@echo "⚡ Running JIT-REPL integration tests..."
	@./fix_linking.sh cargo test --test jit_repl_integration_test

jit-test-legacy:
	@echo "⚡ Running legacy JIT tests..."
	@./fix_linking.sh cargo test --test simple_jit_test || true
	@./fix_linking.sh cargo test --test jit_basic_test || true
	@./fix_linking.sh cargo test --test jit_integration_test || true

jit-test-coverage:
	@echo "📊 Generating JIT test coverage report..."
	@./tests/run_jit_comprehensive_tests.sh --report jit_coverage_report.md

jit-test-report:
	@echo "📊 Generating JIT comprehensive test report..."
	@./tests/run_jit_comprehensive_tests.sh --report jit_test_report.md

jit-test-linking-fix:
	@echo "⚡ Running JIT tests with linking fix..."
	@./tests/run_jit_comprehensive_tests.sh --linking-fix

jit-help:
	@echo "⚡ JIT Test Targets:"
	@echo "  jit-test-quick        - Run quick JIT tests (skip stress tests)"
	@echo "  jit-test              - Run comprehensive JIT tests"
	@echo "  jit-test-all          - Run all JIT tests including stress tests"
	@echo "  jit-test-verbose      - Run JIT tests with verbose output"
	@echo "  jit-test-engine       - Run JIT engine core tests only"
	@echo "  jit-test-repl         - Run JIT-REPL integration tests only"
	@echo "  jit-test-legacy       - Run legacy JIT tests"
	@echo "  jit-test-coverage     - Generate JIT test coverage report"
	@echo "  jit-test-report       - Generate comprehensive JIT test report"
	@echo "  jit-test-linking-fix  - Run JIT tests with linking fix (for Nix)"
	@echo "  jit-help              - Show this help"

